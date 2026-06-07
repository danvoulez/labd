//! Learning report — reading history back into the Lab (FINAL §11.8).
//!
//! Summarizes what closed, failed, and remained ghosted, what repeated, and —
//! crucially — proposes the next Act (A21). Like every report, it is a projection
//! and never a receipt: it proves and closes nothing.

use logline_lab_core::GhostLog;
use logline_lab_spine::Spine;
use serde::{Deserialize, Serialize};

/// A (did, count) pair for repeated activity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Repeated {
    pub did: String,
    pub count: usize,
}

/// What the Lab learned from its history, with a proposed next Act.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LearningReport {
    pub kind: String,
    pub lab_id: String,
    pub generated_at: String,
    pub closed: usize,
    pub failed: usize,
    pub ghosted: usize,
    pub repeated: Vec<Repeated>,
    pub open_ghosts: Vec<String>,
    /// The proposed next Act (always present — learning always points forward).
    pub next_action: String,
}

impl LearningReport {
    pub fn is_receipt(&self) -> bool {
        false
    }
}

/// Generate a learning report from the spine and the open ghosts (A21).
pub fn generate_learning(
    spine: &dyn Spine,
    ghosts: &GhostLog,
    lab_id: &str,
    now: &str,
) -> LearningReport {
    use std::collections::BTreeMap;

    let acts: Vec<_> = spine.all().into_iter().map(|s| s.act).collect();
    let status_is = |a: &logline_act::Act, s: &str| a.status.as_str() == Some(s);

    let closed = acts
        .iter()
        .filter(|a| status_is(a, "closed") || status_is(a, "resolved") || status_is(a, "done"))
        .count();
    let failed = acts.iter().filter(|a| status_is(a, "failed")).count();

    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for a in &acts {
        if let Some(did) = a.did.as_str() {
            *counts.entry(did.to_string()).or_insert(0) += 1;
        }
    }
    let repeated: Vec<Repeated> = counts
        .into_iter()
        .filter(|(_, c)| *c > 1)
        .map(|(did, count)| Repeated { did, count })
        .collect();

    let open: Vec<String> = ghosts.open().iter().map(|g| g.id.clone()).collect();

    // Learning always proposes a next Act; it never ends in silence.
    let next_action = if let Some(first) = open.first() {
        format!("propose Act: drive ghost `{first}` toward its closure condition")
    } else if failed > 0 {
        "propose Act: re-run the failed study bench with repaired inputs".to_string()
    } else if acts.is_empty() {
        "propose Act: write the first candidate Act for this Lab".to_string()
    } else {
        "propose Act: open a new study bench to extend proven practice".to_string()
    };

    LearningReport {
        kind: "logline.learning_report.v0".to_string(),
        lab_id: lab_id.to_string(),
        generated_at: now.to_string(),
        closed,
        failed,
        ghosted: open.len(),
        repeated,
        open_ghosts: open,
        next_action,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logline_act::Act;
    use logline_lab_core::{Ghost, GhostLog};
    use logline_lab_local::LocalOutbox;
    use logline_lab_spine::{sync, MemorySpine};
    use serde_json::json;

    fn act(did: &str, status: &str) -> Act {
        Act::new(
            json!("lab.operator"),
            json!(did),
            json!({"k": did}),
            json!("2026-06-06T00:00:00Z"),
            json!("none"),
            json!("record"),
            json!("carry_blocked"),
            json!("reject"),
            json!(status),
        )
    }

    /// A21 — Learning report proposes next Act.
    #[test]
    fn a21_learning_proposes_next_act() {
        let mut outbox = LocalOutbox::in_memory();
        outbox.emit(&act("observe", "closed")).unwrap();
        outbox.emit(&act("observe", "failed")).unwrap();
        let mut spine = MemorySpine::new();
        sync(&mut outbox, &mut spine).unwrap();

        let mut ghosts = GhostLog::new();
        ghosts.record(Ghost::new("ghost:probe", "scope.x", "missing evidence", "capture probe output"));

        let report = generate_learning(&spine, &ghosts, "test.lab", "2026-06-06T12:00:00Z");
        assert!(!report.is_receipt());
        assert_eq!(report.ghosted, 1);
        assert!(report.next_action.contains("ghost:probe"));
        // repeated `observe` (count 2) is surfaced.
        assert!(report.repeated.iter().any(|r| r.did == "observe" && r.count == 2));
    }
}
