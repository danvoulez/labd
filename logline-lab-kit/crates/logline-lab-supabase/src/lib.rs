//! `logline-lab-supabase` — the Supabase/Postgres spine profile adapter.
//!
//! This adapter builds the exact ingest payload accepted by the recovered
//! `ops.ingest_logline_act(payload jsonb)` migration (content-addressed
//! idempotency: `idempotency_key = content_hash`). It implements the generic
//! `Spine` trait in a **staging** mode: payloads are canonicalized and buffered
//! locally, ready to ship.
//!
//! GHOST `supabase-live-ingest`: the live network POST to a real Supabase
//! project is not performed here (no project/credentials in this environment).
//! Closing this ghost requires running `init/doctor --profile supabase` against a
//! real project and capturing the returned ingest row as evidence.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use logline_act::Act;
use logline_lab_spine::{IngestOutcome, Spine, SpineError, StoredAct};
use serde_json::{json, Value};

/// Connection configuration. Secrets are references only — never embedded
/// (Operator §13: package embedding secrets is forbidden).
#[derive(Clone, Debug, Default)]
pub struct SupabaseConfig {
    /// e.g. `https://<project>.supabase.co`
    pub url: String,
    /// Name of the env var holding the service key (a *reference*, not the key).
    pub service_key_env: String,
}

/// The Supabase spine profile adapter.
pub struct SupabaseSpine {
    config: SupabaseConfig,
    /// Staged acts (content_hash -> stored). Mirrors what a live spine would hold
    /// after idempotent ingest.
    staged: BTreeMap<String, StoredAct>,
    /// Canonical payloads ready to POST to `ops.ingest_logline_act`.
    pending: Vec<Value>,
}

impl SupabaseSpine {
    pub fn new(config: SupabaseConfig) -> Self {
        Self {
            config,
            staged: BTreeMap::new(),
            pending: Vec::new(),
        }
    }

    /// The migrations this profile expects on the spine (recovered SQL lives in
    /// `profiles/supabase/migrations/`).
    pub fn migrations() -> &'static [&'static str] {
        &[
            "0001_core_acts",
            "0002_registry",
            "0003_audit_views",
            "0004_lab_observability",
            "0005_evidence",
            "0006_receipts",
            "0007_workorders",
            "0008_authz",
            "0009_functions_projectors",
            "0011_ingest_idempotent_immutable",
        ]
    }

    /// Build the jsonb payload accepted by `ops.ingest_logline_act`. Slots arrive
    /// as the engine models them; `content_hash`/`tuple_hash` ride alongside as
    /// envelope, not as a tenth slot.
    pub fn ingest_payload(act: &Act) -> Result<Value, SpineError> {
        let content_hash = act.content_hash().map_err(|e| SpineError::Hash(e.to_string()))?;
        let tuple_hash = act.tuple_hash().map_err(|e| SpineError::Hash(e.to_string()))?;
        let mut payload = act.to_value();
        let obj = payload.as_object_mut().expect("act is object");
        obj.insert("content_hash".into(), json!(content_hash));
        obj.insert("tuple_hash".into(), json!(tuple_hash));
        Ok(payload)
    }

    /// Canonical payloads queued for the live spine (drives the ghost closure).
    pub fn pending_payloads(&self) -> &[Value] {
        &self.pending
    }

    pub fn config(&self) -> &SupabaseConfig {
        &self.config
    }

    /// Whether live ingest is wired. Always false in v0 (ghost).
    pub fn live(&self) -> bool {
        false
    }
}

impl Spine for SupabaseSpine {
    fn ingest(&mut self, act: &Act) -> Result<IngestOutcome, SpineError> {
        let hash = act.content_hash().map_err(|e| SpineError::Hash(e.to_string()))?;
        if self.staged.contains_key(&hash) {
            return Ok(IngestOutcome::AlreadyPresent(hash));
        }
        self.pending.push(Self::ingest_payload(act)?);
        self.staged.insert(
            hash.clone(),
            StoredAct {
                content_hash: hash.clone(),
                act: act.clone(),
            },
        );
        Ok(IngestOutcome::Ingested(hash))
    }

    fn get(&self, content_hash: &str) -> Option<StoredAct> {
        self.staged.get(content_hash).cloned()
    }

    fn all(&self) -> Vec<StoredAct> {
        self.staged.values().cloned().collect()
    }

    fn kind(&self) -> &str {
        "supabase"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn act() -> Act {
        Act::new(
            json!("lab.operator"),
            json!("declare_lab"),
            json!({"lab_id": "x"}),
            json!("2026-06-06T00:00:00Z"),
            json!("none"),
            json!("record"),
            json!("carry_blocked"),
            json!("reject"),
            json!("candidate"),
        )
    }

    #[test]
    fn builds_content_addressed_payload() {
        let payload = SupabaseSpine::ingest_payload(&act()).unwrap();
        let obj = payload.as_object().unwrap();
        assert!(obj.contains_key("content_hash"));
        assert!(obj.contains_key("tuple_hash"));
        // The nine slots are all present.
        for slot in logline_act::SLOTS {
            assert!(obj.contains_key(slot));
        }
    }

    #[test]
    fn staging_spine_is_idempotent_and_queues_payloads() {
        let mut spine = SupabaseSpine::new(SupabaseConfig {
            url: "https://example.supabase.co".into(),
            service_key_env: "SUPABASE_SERVICE_KEY".into(),
        });
        let a = act();
        assert!(matches!(spine.ingest(&a).unwrap(), IngestOutcome::Ingested(_)));
        assert!(matches!(
            spine.ingest(&a).unwrap(),
            IngestOutcome::AlreadyPresent(_)
        ));
        assert_eq!(spine.pending_payloads().len(), 1);
        assert!(!spine.live()); // live ingest is ghosted
    }
}
