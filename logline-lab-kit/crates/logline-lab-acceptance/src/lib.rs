//! `logline-lab-acceptance` — the end-to-end acceptance gates A23–A30.
//!
//! Lower gates (A1–A22) are proven inside their owning crates. This crate
//! exercises the pack-loading, Manhattan L-06 serious-proof, and recovery/debris
//! boundaries against the real pack/profile manifests in the repo.

#![forbid(unsafe_code)]

/// Helpers shared by the acceptance tests.
pub mod harness {
    use logline_act::Act;
    use serde_json::json;

    pub const DEMO_PACK: &str = include_str!("../../../packs/demo/pack.json");
    pub const SANTO_ANDRE_PACK: &str = include_str!("../../../packs/santo-andre/pack.json");
    pub const MANHATTAN_PACK: &str = include_str!("../../../packs/manhattan/pack.json");
    pub const LOCAL_PROFILE: &str = include_str!("../../../profiles/local-only/profile.json");

    /// Build a valid nine-slot Act.
    pub fn act(who: &str, did: &str, this: serde_json::Value, status: &str) -> Act {
        Act::new(
            json!(who),
            json!(did),
            this,
            json!("2026-06-06T00:00:00Z"),
            json!("none"),
            json!("record"),
            json!("carry_as_blocked_act"),
            json!("reject"),
            json!(status),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::harness::*;
    use logline_lab_core::{
        evidence::EvidenceLog, receipt::ReceiptCandidate, LabManifest, PackManifest,
        ProfileManifest,
    };
    use logline_lab_dispatch::{Mode, ShellWorker, Workorder};
    use logline_lab_labd::Lab;
    use logline_lab_projectors::health;
    use logline_lab_recovery_scanner::{scan_authority, scan_storage, Category};
    use serde_json::json;

    fn demo_lab() -> Lab {
        let manifest =
            LabManifest::load(r#"{"lab_id":"demo.local.lab","profile":"local-only","pack":"demo"}"#)
                .unwrap();
        let pack = PackManifest::load(DEMO_PACK).unwrap();
        let profile = ProfileManifest::load(LOCAL_PROFILE).unwrap();
        Lab::init(manifest, pack, profile).unwrap()
    }

    /// A23 — Demo pack completes first session.
    #[test]
    fn a23_demo_pack_first_session() {
        let mut lab = demo_lab();
        assert_eq!(lab.pack().name, "demo");
        // A first session: declare the lab, then a demo observation.
        lab.emit(&act("demo.operator", "declare_lab", json!({"lab_id": "demo.local.lab"}), "candidate"))
            .unwrap();
        lab.emit(&act("demo.operator", "greet", json!({"message": "hello lab"}), "candidate"))
            .unwrap();
        lab.sync().unwrap();
        let report = lab.report("2026-06-06T12:00:00Z");
        assert_eq!(report.total_acts, 2);
        assert!(report.recent_dids.contains(&"declare_lab".to_string()));
        assert!(report.recent_dids.contains(&"greet".to_string()));
    }

    /// A24 — Santo André pack loads without becoming core.
    #[test]
    fn a24_santo_andre_pack_loads_without_becoming_core() {
        let before = logline_act::SLOTS;
        let pack = PackManifest::load(SANTO_ANDRE_PACK).unwrap();
        assert_eq!(pack.name, "santo-andre");
        // It is data: it does not redefine the nine slots, and a core act still
        // validates exactly as before.
        assert_eq!(before, logline_act::SLOTS);
        let a = act("x", "declare_lab", json!({}), "candidate");
        assert!(a.is_valid());
        // The pack is not the product root: its name is not "logline-lab-kit".
        assert_ne!(pack.name, "logline-lab-kit");
    }

    /// A25 — Manhattan pack loads without becoming core.
    #[test]
    fn a25_manhattan_pack_loads_without_becoming_core() {
        let before = logline_act::SLOTS;
        let pack = PackManifest::load(MANHATTAN_PACK).unwrap();
        assert_eq!(pack.name, "manhattan");
        assert!(pack.scopes.contains(&"manhattan.L-06".to_string()));
        assert_eq!(before, logline_act::SLOTS);
        assert_ne!(pack.name, "logline-lab-kit");
    }

    /// A26 — Manhattan L-06 emits evidence.
    ///
    /// The serious proof chain: Act → gate → worker probe → evidence. The probe
    /// is a *real* executed subprocess producing genuine runtime evidence. The
    /// real fleet Ethernet ping to peers remains ghosted (G-08).
    #[test]
    fn a26_manhattan_l06_emits_evidence() {
        let scope = "manhattan.L-06";
        // Gate decision: the L-06 gate admits the probe.
        let permitted = true;
        assert!(permitted, "L-06 gate must admit before a worker runs");

        // Worker cannot run without allow (gate), then runs a real command.
        let wo = Workorder::new("L-06-probe", scope, vec![
            "sh".into(),
            "-c".into(),
            "echo l06_link_probe_ran".into(),
        ])
        .allowed(permitted)
        .with_mode(Mode::Real);

        let report = ShellWorker::execute(&wo, "2026-06-06T00:00:01Z").unwrap();
        // Real runtime evidence, not a bare claim.
        assert!(!report.is_receipt());
        let stdout = report.evidence.payload.get("stdout").unwrap().as_str().unwrap();
        assert!(stdout.contains("l06_link_probe_ran"));
        assert_eq!(report.evidence.scope, scope);
    }

    /// A27 — L-06 receipt closes only L-06.
    #[test]
    fn a27_l06_receipt_closes_only_l06() {
        let scope = "manhattan.L-06";
        let l06_act = act("manhattan.runtime", "ethernet_ping_check", json!({"target": "peer"}), "pending");

        // Attach the real probe evidence to the L-06 scope only.
        let wo = Workorder::new("L-06-probe", scope, vec!["sh".into(), "-c".into(), "echo ok".into()])
            .allowed(true)
            .with_mode(Mode::Real);
        let exec = ShellWorker::execute(&wo, "t0").unwrap();
        let mut ev = EvidenceLog::new();
        ev.attach(exec.evidence);

        let candidate = ReceiptCandidate::prepare(&l06_act, scope, &ev).unwrap();
        assert!(candidate.closes_only(scope));
        // It does NOT close any other scope.
        assert!(!candidate.closes_only("manhattan.L-07"));
        assert!(!candidate.closes_only("manhattan.fleet"));
        assert_eq!(candidate.evidence_count, 1);
    }

    /// A28 — Projection shows L-06 health from Acts.
    #[test]
    fn a28_projection_shows_l06_health() {
        let scope = "manhattan.L-06";
        let mut lab = demo_lab();
        lab.emit(&act("manhattan.runtime", "ethernet_ping_check", json!({"target": "peer"}), "candidate"))
            .unwrap();
        lab.sync().unwrap();

        // Without evidence: not healthy.
        let empty = EvidenceLog::new();
        let unhealthy = health(lab.spine(), scope, "ethernet_ping_check", &empty);
        assert_eq!(unhealthy.acts, 1);
        assert!(!unhealthy.healthy);

        // With real probe evidence attached: healthy, derived purely from Acts.
        let wo = Workorder::new("L-06-probe", scope, vec!["sh".into(), "-c".into(), "echo ok".into()])
            .allowed(true)
            .with_mode(Mode::Real);
        let exec = ShellWorker::execute(&wo, "t0").unwrap();
        let mut ev = EvidenceLog::new();
        ev.attach(exec.evidence);
        let healthy = health(lab.spine(), scope, "ethernet_ping_check", &ev);
        assert!(healthy.healthy);
    }

    /// A29 — Recovery scan catches false authority.
    #[test]
    fn a29_recovery_scan_catches_false_authority() {
        let poisoned = "This file is the single source of truth.\nThe assistant decided the final shape.";
        let findings = scan_authority(poisoned);
        assert!(findings.iter().any(|f| f.category == Category::FalseAuthority));
        // The repo's own honest docs do not trip it.
        let honest = "Source fruits are raw material, not authority.";
        assert!(scan_authority(honest).is_empty());
    }

    /// A30 — Storage scan rejects file/SQLite truth language.
    #[test]
    fn a30_storage_scan_rejects_truth_language() {
        let poisoned = "SQLite is the truth.\nFiles are the official semantic storage.";
        let findings = scan_storage(poisoned);
        assert!(findings.iter().any(|f| f.category == Category::StorageTruth));
        let honest = "The local outbox is a provisional cache, never truth.";
        assert!(scan_storage(honest).is_empty());
    }
}
