//! `logline-lab-projectors` — read models over the spine.
//!
//! Projections are derived views, never truth. They read Acts from the spine and
//! present recent activity, a registry, blocked Acts, and scope health. A
//! projection mirrors the recovered SQL views (`0002_registry`, `0003_audit`,
//! `0004_lab_observability`) as in-process functions.

#![forbid(unsafe_code)]

use logline_lab_core::{evidence::EvidenceLog, BlockContext, BlockedAct};
use logline_lab_spine::{Spine, StoredAct};
use serde::{Deserialize, Serialize};

/// Most recent Acts (by stored order), newest first, up to `limit`.
pub fn recent(spine: &dyn Spine, limit: usize) -> Vec<StoredAct> {
    let mut all = spine.all();
    // Stable by `when` then content hash so the view is deterministic.
    all.sort_by(|a, b| {
        let aw = a.act.when.as_str().unwrap_or("");
        let bw = b.act.when.as_str().unwrap_or("");
        bw.cmp(aw).then(b.content_hash.cmp(&a.content_hash))
    });
    all.into_iter().take(limit).collect()
}

/// A registry row: a distinct (who, did) pair and how often it occurs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryRow {
    pub who: String,
    pub did: String,
    pub count: usize,
}

/// Registry projection: distinct (who, did) with counts.
pub fn registry(spine: &dyn Spine) -> Vec<RegistryRow> {
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<(String, String), usize> = BTreeMap::new();
    for s in spine.all() {
        let who = s.act.who.as_str().unwrap_or("").to_string();
        let did = s.act.did.as_str().unwrap_or("").to_string();
        *counts.entry((who, did)).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .map(|((who, did), count)| RegistryRow { who, did, count })
        .collect()
}

/// All Acts on the spine matching a `did`.
pub fn by_did(spine: &dyn Spine, did: &str) -> Vec<StoredAct> {
    spine
        .all()
        .into_iter()
        .filter(|s| s.act.did.as_str() == Some(did))
        .collect()
}

/// Health of a scope, derived purely from Acts + evidence (A28). A scope is
/// healthy when at least one Act for it is backed by evidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthView {
    pub scope: String,
    pub acts: usize,
    pub with_evidence: bool,
    pub healthy: bool,
}

/// Project the health of a scope from the spine, given an evidence log. The
/// `did` identifies the Acts that belong to the scope.
pub fn health(spine: &dyn Spine, scope: &str, did: &str, evidence: &EvidenceLog) -> HealthView {
    let acts = by_did(spine, did).len();
    let with_evidence = evidence.has_for_scope(scope);
    HealthView {
        scope: scope.to_string(),
        acts,
        with_evidence,
        healthy: acts > 0 && with_evidence,
    }
}

/// Blocked-Act projection: evaluate every stored Act for a scope and collect the
/// blocked ones. Permission is supplied by the caller's policy.
pub fn blocked(
    spine: &dyn Spine,
    scope: &str,
    evidence: &EvidenceLog,
    permitted: bool,
) -> Vec<BlockedAct> {
    let ctx = BlockContext { evidence, permitted };
    spine
        .all()
        .iter()
        .filter_map(|s| logline_lab_core::evaluate_blocked(&s.act, scope, &ctx))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use logline_act::Act;
    use logline_lab_local::LocalActLog;
    use logline_lab_spine::{sync, MemorySpine};
    use serde_json::json;

    fn act(did: &str) -> Act {
        Act::new(
            json!("lab.operator"),
            json!(did),
            json!({"k": did}),
            json!("2026-06-06T00:00:00Z"),
            json!("none"),
            json!("record"),
            json!("carry_blocked"),
            json!("reject"),
            json!("candidate"),
        )
    }

    /// A9 — Projection reads Act from spine.
    #[test]
    fn a9_projection_reads_from_spine() {
        let mut act_log = LocalActLog::in_memory();
        act_log.emit(&act("declare_lab")).unwrap();
        act_log.emit(&act("observe")).unwrap();
        let mut spine = MemorySpine::new();
        sync(&mut act_log, &mut spine).unwrap();

        let view = recent(&spine, 10);
        assert_eq!(view.len(), 2);

        let reg = registry(&spine);
        assert_eq!(reg.len(), 2);
        assert!(reg.iter().all(|r| r.who == "lab.operator"));

        assert_eq!(by_did(&spine, "declare_lab").len(), 1);
    }
}
