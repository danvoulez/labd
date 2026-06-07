//! `logline-lab-spine` — the generic spine boundary.
//!
//! A spine is the configured place Acts are ingested and read back. The spine
//! trait is generic: a profile chooses a concrete adapter (in-memory, Postgres,
//! Supabase). Ingest is idempotent on the content hash, matching the recovered
//! Supabase `ops.ingest_logline_act` migration (content-addressed dedupe). The
//! spine stores Acts immutably; storage metadata lives around the Act.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use logline_act::Act;
use logline_lab_local::LocalActLog;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpineError {
    #[error("act hashing failed: {0}")]
    Hash(String),
    #[error("spine backend error: {0}")]
    Backend(String),
}

/// An Act as stored on the spine: the canonical Act plus storage metadata kept
/// strictly *around* it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoredAct {
    pub content_hash: String,
    pub act: Act,
}

/// Outcome of a single ingest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IngestOutcome {
    Ingested(String),
    AlreadyPresent(String),
}

impl IngestOutcome {
    pub fn content_hash(&self) -> &str {
        match self {
            IngestOutcome::Ingested(h) | IngestOutcome::AlreadyPresent(h) => h,
        }
    }
}

/// The generic spine interface. Concrete profiles implement this.
pub trait Spine {
    /// Idempotently ingest an Act (content-addressed dedupe).
    fn ingest(&mut self, act: &Act) -> Result<IngestOutcome, SpineError>;
    /// Read a stored Act by content hash.
    fn get(&self, content_hash: &str) -> Option<StoredAct>;
    /// Read all stored Acts.
    fn all(&self) -> Vec<StoredAct>;
    /// A label for the configured spine (for doctor/reports).
    fn kind(&self) -> &str;
}

/// The default in-memory spine. Used by the `local-only` profile and as the
/// reference implementation for the spine contract.
#[derive(Debug, Default)]
pub struct MemorySpine {
    acts: BTreeMap<String, StoredAct>,
}

impl MemorySpine {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Spine for MemorySpine {
    fn ingest(&mut self, act: &Act) -> Result<IngestOutcome, SpineError> {
        let hash = act.content_hash().map_err(|e| SpineError::Hash(e.to_string()))?;
        if self.acts.contains_key(&hash) {
            return Ok(IngestOutcome::AlreadyPresent(hash));
        }
        self.acts.insert(
            hash.clone(),
            StoredAct {
                content_hash: hash.clone(),
                act: act.clone(),
            },
        );
        Ok(IngestOutcome::Ingested(hash))
    }

    fn get(&self, content_hash: &str) -> Option<StoredAct> {
        self.acts.get(content_hash).cloned()
    }

    fn all(&self) -> Vec<StoredAct> {
        self.acts.values().cloned().collect()
    }

    fn kind(&self) -> &str {
        "memory"
    }
}

/// Report of a replay run (local Act-log → query spine).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SyncReport {
    pub ingested: usize,
    pub already_present: usize,
}

/// Replay all not-yet-replayed entries from the local Act-log into the configured
/// spine, marking them replayed. (For `dev-ephemeral` this rebuilds the in-process
/// query store from the durable local Act-log; a future external `TransportOutbox`
/// would reuse the same idempotent ingest.)
pub fn sync(act_log: &mut LocalActLog, spine: &mut dyn Spine) -> Result<SyncReport, SpineError> {
    let pending: Vec<(String, Act)> = act_log
        .unreplayed()
        .into_iter()
        .map(|e| (e.content_hash.clone(), e.act.clone()))
        .collect();

    let mut report = SyncReport::default();
    for (hash, act) in pending {
        match spine.ingest(&act)? {
            IngestOutcome::Ingested(_) => report.ingested += 1,
            IngestOutcome::AlreadyPresent(_) => report.already_present += 1,
        }
        act_log
            .mark_replayed(&hash)
            .map_err(|e| SpineError::Backend(e.to_string()))?;
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_act(did: &str) -> Act {
        Act::new(
            json!("lab.operator"),
            json!(did),
            json!({"k": "v"}),
            json!("2026-06-06T00:00:00Z"),
            json!("none"),
            json!("record"),
            json!("carry_blocked"),
            json!("reject"),
            json!("candidate"),
        )
    }

    /// A8 — Sync writes Act to configured spine (and is idempotent).
    #[test]
    fn a8_sync_writes_to_spine() {
        let mut act_log = LocalActLog::in_memory();
        act_log.emit(&sample_act("declare_lab")).unwrap();
        act_log.emit(&sample_act("declare_other")).unwrap();

        let mut spine = MemorySpine::new();
        let report = sync(&mut act_log, &mut spine).unwrap();
        assert_eq!(report.ingested, 2);
        assert_eq!(spine.all().len(), 2);
        assert!(act_log.unreplayed().is_empty());

        // A second sync is a no-op (already synced + idempotent ingest).
        let report2 = sync(&mut act_log, &mut spine).unwrap();
        assert_eq!(report2.ingested, 0);
        assert_eq!(spine.all().len(), 2);
    }

    /// Ingest itself is content-addressed idempotent (matches 0011 migration).
    #[test]
    fn ingest_is_idempotent() {
        let mut spine = MemorySpine::new();
        let act = sample_act("declare_lab");
        let first = spine.ingest(&act).unwrap();
        let second = spine.ingest(&act).unwrap();
        assert!(matches!(first, IngestOutcome::Ingested(_)));
        assert!(matches!(second, IngestOutcome::AlreadyPresent(_)));
        assert_eq!(first.content_hash(), second.content_hash());
    }
}
