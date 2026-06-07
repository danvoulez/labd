//! `logline-lab-reports` — the report generator.
//!
//! A report renders Lab state from projections. A report is **not** a receipt: it
//! proves nothing and closes nothing (A13, Operator §14: "Receipts close only
//! what was proven"). The type deliberately carries no hashes, no evidence
//! references, and no closure semantics.

#![forbid(unsafe_code)]

pub mod learning;
pub use learning::{generate_learning, LearningReport};

use logline_lab_projectors::{recent, registry, RegistryRow};
use logline_lab_spine::Spine;
use serde::{Deserialize, Serialize};

/// A point-in-time Lab report rendered from projections.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabReport {
    /// Discriminator. Note it is a *report*, never a receipt.
    pub kind: String,
    pub lab_id: String,
    pub generated_at: String,
    pub total_acts: usize,
    pub registry: Vec<RegistryRow>,
    pub recent_dids: Vec<String>,
}

impl LabReport {
    /// A report never closes a scope and is never a receipt (A13).
    pub fn is_receipt(&self) -> bool {
        false
    }

    /// A report has no closure power — it cannot close any scope.
    pub fn closes(&self, _scope: &str) -> bool {
        false
    }
}

/// Generate a Lab report from the spine via projections (A22).
pub fn generate(spine: &dyn Spine, lab_id: &str, now: &str) -> LabReport {
    let recent_acts = recent(spine, 20);
    LabReport {
        kind: "logline.report.v0".to_string(),
        lab_id: lab_id.to_string(),
        generated_at: now.to_string(),
        total_acts: spine.all().len(),
        registry: registry(spine),
        recent_dids: recent_acts
            .iter()
            .filter_map(|s| s.act.did.as_str().map(|d| d.to_string()))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logline_act::Act;
    use logline_lab_local::LocalOutbox;
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

    /// A22 — Lab report renders state from projections.
    #[test]
    fn a22_report_renders_from_projections() {
        let mut outbox = LocalOutbox::in_memory();
        outbox.emit(&act("declare_lab")).unwrap();
        outbox.emit(&act("observe")).unwrap();
        let mut spine = MemorySpine::new();
        sync(&mut outbox, &mut spine).unwrap();

        let report = generate(&spine, "test.local.lab", "2026-06-06T12:00:00Z");
        assert_eq!(report.total_acts, 2);
        assert_eq!(report.registry.len(), 2);
        assert!(report.recent_dids.contains(&"declare_lab".to_string()));
    }

    /// A13 — Report does not pretend to be receipt.
    #[test]
    fn a13_report_is_not_receipt() {
        let spine = MemorySpine::new();
        let report = generate(&spine, "test.local.lab", "t0");
        assert!(!report.is_receipt());
        assert!(!report.closes("any.scope"));
        assert_eq!(report.kind, "logline.report.v0");
        // Serialized form carries no hashes / evidence / closure fields.
        let v = serde_json::to_value(&report).unwrap();
        let obj = v.as_object().unwrap();
        assert!(!obj.contains_key("hashes"));
        assert!(!obj.contains_key("evidence"));
        assert!(!obj.contains_key("receipt"));
    }
}
