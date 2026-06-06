//! `logline-lab-core` — generic Lab Kit domain machinery layered over the Act:
//! branch/verdict, evidence, blocked Acts, receipt candidates, manifests, and
//! the app/MCP authority boundary. No pack-specific or Dan-specific semantics
//! live here (Operator §13).

#![forbid(unsafe_code)]

pub mod app;
pub mod blocked;
pub mod branch;
pub mod evidence;
pub mod manifest;
pub mod receipt;

pub use app::{AppCall, AppError, AppRegistry};
pub use blocked::{evaluate as evaluate_blocked, BlockContext, BlockReason, BlockedAct};
pub use branch::{select_branch, Verdict};
pub use evidence::{Evidence, EvidenceLog};
pub use manifest::{LabManifest, ManifestError, PackManifest, ProfileManifest};
pub use receipt::{ReceiptCandidate, ReceiptError};

#[cfg(test)]
mod tests {
    use super::*;
    use logline_act::{Act, SLOTS};
    use serde_json::json;

    fn act(confirmed_by: &str, status: &str) -> Act {
        Act::new(
            json!("lab.operator"),
            json!("declare_state"),
            json!({"target": "x"}),
            json!("2026-06-06T00:00:00Z"),
            json!(confirmed_by),
            json!("record"),
            json!("carry_blocked"),
            json!("reject"),
            json!(status),
        )
    }

    /// A10 — Blocked Act appears when evidence or permission is missing.
    #[test]
    fn a10_blocked_on_missing_permission_and_evidence() {
        let ev = EvidenceLog::new();
        let a = act("evidence", "pending");

        // Missing permission.
        let ctx = BlockContext { evidence: &ev, permitted: false };
        let blocked = evaluate_blocked(&a, "scope.x", &ctx).expect("blocked");
        assert_eq!(blocked.reason, BlockReason::MissingPermission);

        // Permitted, but evidence required and absent.
        let ctx = BlockContext { evidence: &ev, permitted: true };
        let blocked = evaluate_blocked(&a, "scope.x", &ctx).expect("blocked");
        assert_eq!(blocked.reason, BlockReason::MissingEvidence);

        // Missing confirmation on a non-candidate act.
        let a2 = act("none", "pending");
        let blocked = evaluate_blocked(&a2, "scope.x", &ctx).expect("blocked");
        assert_eq!(blocked.reason, BlockReason::MissingConfirmation);

        // Once evidence is attached, it is no longer blocked.
        let mut ev2 = EvidenceLog::new();
        ev2.attach(Evidence::new("scope.x", "probe", json!({"ok": true}), "2026-06-06T00:00:01Z"));
        let ctx2 = BlockContext { evidence: &ev2, permitted: true };
        assert!(evaluate_blocked(&a, "scope.x", &ctx2).is_none());
    }

    /// A11 — Evidence can be attached to scope.
    #[test]
    fn a11_evidence_attaches_to_scope() {
        let mut ev = EvidenceLog::new();
        ev.attach(Evidence::new("scope.a", "probe", json!({"v": 1}), "t0"));
        ev.attach(Evidence::new("scope.b", "probe", json!({"v": 2}), "t1"));
        assert_eq!(ev.for_scope("scope.a").len(), 1);
        assert!(ev.has_for_scope("scope.a"));
        assert!(!ev.has_for_scope("scope.c"));
    }

    /// A12 — Receipt candidate names exact scope (and requires evidence).
    #[test]
    fn a12_receipt_candidate_names_exact_scope() {
        let a = act("evidence", "pending");
        let mut ev = EvidenceLog::new();

        // Without evidence: rejected (no receipt without evidence).
        assert!(matches!(
            ReceiptCandidate::prepare(&a, "scope.x", &ev),
            Err(ReceiptError::NoEvidence(_))
        ));

        ev.attach(Evidence::new("scope.x", "probe", json!({"ok": true}), "t0"));
        ev.attach(Evidence::new("scope.y", "probe", json!({"ok": true}), "t1"));
        let cand = ReceiptCandidate::prepare(&a, "scope.x", &ev).unwrap();
        assert!(cand.closes_only("scope.x"));
        assert!(!cand.closes_only("scope.y"));
        assert_eq!(cand.evidence_count, 1); // only scope.x evidence, not scope.y
    }

    /// A14 — Pack loads without changing core.
    #[test]
    fn a14_pack_loads_without_changing_core() {
        let before = SLOTS;
        let pack = PackManifest::load(
            r#"{"name":"demo","version":"0.1.0","did_vocabulary":["greet"],"scopes":["demo.session"]}"#,
        )
        .unwrap();
        assert_eq!(pack.name, "demo");
        // Core is untouched: slots identical, a core act still validates the same.
        assert_eq!(before, SLOTS);
        assert!(act("system", "candidate").is_valid());
    }

    /// A15 — Profile loads without changing core.
    #[test]
    fn a15_profile_loads_without_changing_core() {
        let before = SLOTS;
        let profile =
            ProfileManifest::load(r#"{"name":"local-only","spine":"memory","settings":{}}"#).unwrap();
        assert_eq!(profile.spine, "memory");
        assert_eq!(before, SLOTS);
    }

    /// A16 — App/MCP call becomes draft Act.
    #[test]
    fn a16_app_call_becomes_draft_act() {
        let mut reg = AppRegistry::new();
        reg.register("cockpit");
        reg.grant("cockpit", "draft_observation").unwrap();
        let call = AppCall {
            app_id: "cockpit".into(),
            capability: "draft_observation".into(),
            this: json!({"note": "tunnel looks up"}),
        };
        let draft = reg.draft_act(&call, "2026-06-06T00:00:00Z").unwrap();
        assert_eq!(draft.who.as_str(), Some("app:cockpit"));
        assert_eq!(draft.status.as_str(), Some("draft"));
        assert_eq!(draft.did.as_str(), Some("draft_observation"));
    }

    /// A17 — Unauthorized app action is blocked.
    #[test]
    fn a17_unauthorized_app_blocked() {
        let mut reg = AppRegistry::new();
        let call = AppCall {
            app_id: "rogue".into(),
            capability: "draft_observation".into(),
            this: json!({}),
        };
        // Unregistered app.
        assert!(matches!(reg.draft_act(&call, "t0"), Err(AppError::UnknownApp(_))));

        // Registered but not granted.
        reg.register("rogue");
        assert!(matches!(
            reg.draft_act(&call, "t0"),
            Err(AppError::NotGranted { .. })
        ));
    }
}
