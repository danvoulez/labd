//! `logline-lab-acceptance` — the end-to-end acceptance gates **A01–A52** from
//! `build-pack/final-real-project-doc.md` §20.
//!
//! Lower mechanics are also proven inside their owning crates; this harness is
//! the authoritative, doc-aligned mapping. The basics are exercised with NO pack
//! (a Lab forms and runs on identity + profile alone); packs are loaded only to
//! prove they are additive complements.

#![forbid(unsafe_code)]

/// Shared fixtures.
pub mod harness {
    use logline_act::Act;
    use serde_json::{json, Value};

    pub const SANTO_ANDRE_PACK: &str = include_str!("../../../packs/santo-andre/pack.json");
    pub const MANHATTAN_PACK: &str = include_str!("../../../packs/manhattan/pack.json");
    pub const COURSE_PACK: &str = include_str!("../../../packs/course-starter/pack.json");
    pub const LOCAL_PROFILE: &str = include_str!("../../../profiles/local-only/profile.json");
    pub const L06_BENCH: &str = include_str!("../../../benches/manhattan-l06/bench.json");

    pub fn act(who: &str, did: &str, this: Value, status: &str) -> Act {
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
    use logline_act::{Act, SLOTS};
    use logline_lab_conformance::{builtin_vectors, export_examples, run as run_conformance};
    use logline_lab_core::{
        bench::{BenchOutcome, StudyBench},
        evidence::Evidence,
        ghost::Ghost,
        LabManifest, PackManifest, ProfileManifest,
    };
    use logline_lab_dispatch::{Mode, ShellWorker, Workorder};
    use logline_lab_labd::{Lab, WriteOutcome};
    use logline_lab_mcp_server::{McpServer, ToolCall};
    use logline_lab_recovery_scanner::{scan_authority, scan_storage, Category};
    use logline_lab_ruler::{capacity, due_work, evaluate_due, CapacityState, Disposition, DueDecision};
    use serde_json::json;

    // --- helpers ---------------------------------------------------------

    /// The basics: a Lab with identity + profile and NO pack.
    fn basics_lab() -> Lab {
        let manifest =
            LabManifest::load(r#"{"lab_id":"basics.local.lab","profile":"local-only"}"#).unwrap();
        let profile = ProfileManifest::load(LOCAL_PROFILE).unwrap();
        Lab::init(manifest, vec![], profile).unwrap()
    }

    fn lab_with(pack_json: &str) -> Lab {
        let manifest =
            LabManifest::load(r#"{"lab_id":"basics.local.lab","profile":"local-only"}"#).unwrap();
        let profile = ProfileManifest::load(LOCAL_PROFILE).unwrap();
        let pack = PackManifest::load(pack_json).unwrap();
        Lab::init(manifest, vec![pack], profile).unwrap()
    }

    fn nine_slot(did: &str) -> Act {
        act("lab.operator", did, json!({"k": did}), "candidate")
    }

    // === 20.1 Act / canon ===============================================

    /// A01 — Act has exactly nine canonical slots.
    #[test]
    fn a01_exactly_nine_slots() {
        assert_eq!(SLOTS.len(), 9);
        assert!(nine_slot("declare_lab").is_valid());
    }

    /// A02 — Unknown semantic domains do not become native authority.
    #[test]
    fn a02_no_native_domain_authority() {
        // A "task" is just an Act with a `did` — not a native authoritative type.
        let task = act("lab.operator", "task", json!({"title": "x"}), "candidate");
        assert!(task.is_valid());
        // Adding a native domain field is rejected as a tenth slot.
        let mut v = task.to_value();
        v["priority"] = json!("high");
        assert!(Act::from_value_strict(&v).is_err());
    }

    /// A03 — Candidate Acts can be ugly.
    #[test]
    fn a03_ugly_candidates_allowed() {
        let ugly = json!({"who": "", "did": "scribble", "status": "candidate"});
        assert!(Act::from_value_strict(&ugly).is_err());
        let cand = Act::candidate_from_value(&ugly);
        assert_eq!(cand.value(), &ugly);
        assert!(!cand.missing_slots().is_empty());
    }

    /// A04 — Promotion requires validation.
    #[test]
    fn a04_promotion_requires_validation() {
        let ugly = json!({"who": "", "did": "scribble", "status": "candidate"});
        assert!(Act::candidate_from_value(&ugly).promote().is_err());
        let repaired = nine_slot("declare_lab").to_value();
        assert!(Act::candidate_from_value(&repaired).promote().is_ok());
    }

    /// A05 — Hash/canonicalization is stable.
    #[test]
    fn a05_hash_stable() {
        let a = nine_slot("declare_lab");
        let mut reordered = serde_json::Map::new();
        for slot in SLOTS.iter().rev() {
            reordered.insert(slot.to_string(), a.to_value()[*slot].clone());
        }
        let b = Act::from_value_strict(&serde_json::Value::Object(reordered)).unwrap();
        assert_eq!(a.content_hash().unwrap(), b.content_hash().unwrap());
    }

    // === 20.2 Lab formation =============================================

    /// A06 — User can initialize a Lab (basics: no pack).
    #[test]
    fn a06_initialize_lab() {
        let lab = basics_lab();
        assert!(lab.packs().is_empty());
        assert_eq!(lab.lab_id(), "basics.local.lab");
    }

    /// A07 — Lab manifest loads.
    #[test]
    fn a07_lab_manifest_loads() {
        let m = LabManifest::load(r#"{"lab_id":"x","profile":"local-only"}"#).unwrap();
        assert_eq!(m.profile, "local-only");
        assert!(m.all_packs().is_empty());
        assert!(LabManifest::load(r#"{"profile":"local-only"}"#).is_err());
    }

    /// A08 — Profile loads (and carries an honest storage grade).
    #[test]
    fn a08_profile_loads() {
        use logline_lab_core::Grade;
        let p = ProfileManifest::load(LOCAL_PROFILE).unwrap();
        assert_eq!(p.spine, "dev-ephemeral");
        assert_eq!(p.grade(), Grade::DevEphemeral);
        let doctor = basics_lab().doctor();
        assert_eq!(doctor.spine_kind, "memory"); // in-process backing store
        assert!(!doctor.publication_grade); // dev-ephemeral is not publication-grade
        assert!(doctor.storage_warning.is_some());
    }

    /// A09 — Pack loads without mutating core.
    #[test]
    fn a09_pack_loads_without_mutating_core() {
        let before = SLOTS;
        let lab = lab_with(SANTO_ANDRE_PACK);
        assert_eq!(lab.packs()[0].name, "santo-andre");
        assert_eq!(before, SLOTS);
        assert!(nine_slot("x").is_valid());
    }

    /// A10 — First candidate Act is created.
    #[test]
    fn a10_first_candidate_created() {
        let mut lab = basics_lab();
        let out = lab.write(&json!({"did": "rough_idea", "status": "candidate"})).unwrap();
        assert!(matches!(out, WriteOutcome::Candidate { .. }));
        assert_eq!(lab.candidates().len(), 1);
    }

    /// A11 — First admitted Act is stored.
    #[test]
    fn a11_first_admitted_stored() {
        let mut lab = basics_lab();
        let out = lab.emit(&nine_slot("declare_lab")).unwrap();
        lab.sync().unwrap();
        assert!(lab.get(out.content_hash()).is_some());
    }

    /// A12 — First Lab report renders.
    #[test]
    fn a12_first_report_renders() {
        let mut lab = basics_lab();
        lab.emit(&nine_slot("declare_lab")).unwrap();
        lab.sync().unwrap();
        assert_eq!(lab.report("t0").total_acts, 1);
    }

    // === 20.3 Protocol ==================================================

    /// A13 — Conformance vectors run.
    #[test]
    fn a13_conformance_vectors_run() {
        assert!(run_conformance(&builtin_vectors()).total >= 5);
    }

    /// A14 — Conformance report is generated.
    #[test]
    fn a14_conformance_report_green() {
        let report = run_conformance(&builtin_vectors());
        assert!(report.is_green(), "{:?}", report.results);
    }

    /// A15 — Example Acts can be exported.
    #[test]
    fn a15_examples_exported() {
        assert_eq!(export_examples(&builtin_vectors()).len(), 2);
    }

    /// A16 — Another Lab can compare against the examples.
    #[test]
    fn a16_examples_reproducible() {
        for e in export_examples(&builtin_vectors()) {
            let act = Act::from_json_strict(&e.act_json).unwrap();
            assert_eq!(act.content_hash().unwrap(), e.content_hash);
        }
    }

    /// A17 — No central hosted service required for minimum Lab operation.
    #[test]
    fn a17_no_central_service_required() {
        // The whole minimum loop runs offline, in-process.
        let mut lab = basics_lab();
        lab.emit(&nine_slot("declare_lab")).unwrap();
        lab.sync().unwrap();
        assert!(lab.conformance().is_green());
        assert_eq!(lab.report("t0").total_acts, 1);
    }

    // === 20.4 Scientific method =========================================

    fn l06_bench() -> StudyBench {
        StudyBench::load(L06_BENCH).unwrap()
    }

    /// A18 — Study bench declares question/hypothesis/probe/evidence/receipt scope.
    #[test]
    fn a18_bench_declares_fields() {
        let b = l06_bench();
        assert!(b.is_well_formed());
        assert_eq!(b.receipt_scope, "manhattan.L-06");
    }

    /// A19 — Bench produces Acts.
    #[test]
    fn a19_bench_produces_acts() {
        let acts = l06_bench().declare_acts("lab", "2026-06-06T00:00:00Z");
        assert_eq!(acts.len(), 2);
        assert!(acts.iter().all(|a| a.is_valid()));
    }

    /// A20 — Observation becomes evidence or ghost.
    #[test]
    fn a20_observation_evidence_or_ghost() {
        let b = l06_bench();
        let ev = b.observe(true, json!({"stdout": "ok"}), "t0");
        assert!(matches!(ev, BenchOutcome::Evidence(_)));
        let gh = b.observe(false, json!(null), "t0");
        assert!(matches!(gh, BenchOutcome::Ghost(_)));
    }

    /// A21 — Learning report proposes next Act.
    #[test]
    fn a21_learning_proposes_next() {
        let lab = basics_lab();
        let report = lab.learn("t0");
        assert!(report.next_action.contains("propose"));
    }

    // === 20.5 Time / ruler ==============================================

    /// A22 — Future Act can be scheduled.
    #[test]
    fn a22_schedule_future_act() {
        let mut lab = basics_lab();
        let scheduled = lab.schedule(&nine_slot("check_link"), "2999-01-01T00:00:00Z").unwrap();
        assert_eq!(scheduled.this.get("due_at").unwrap(), "2999-01-01T00:00:00Z");
        assert_eq!(scheduled.status.as_str(), Some("scheduled"));
    }

    /// A23 — Tick/check discovers due work.
    #[test]
    fn a23_discovers_due_work() {
        let mut lab = basics_lab();
        lab.schedule(&nine_slot("check_link"), "2026-06-06T00:00:00Z").unwrap();
        lab.sync().unwrap();
        assert_eq!(due_work(lab.spine(), "2026-06-06T12:00:00Z").len(), 1);
    }

    /// A24/A25 — Due work resolves/blocks/reschedules visibly; none skipped.
    #[test]
    fn a24_a25_dispositions_cover_all_due() {
        let mut lab = basics_lab();
        lab.schedule(&nine_slot("run_me"), "2026-06-06T00:00:00Z").unwrap();
        lab.schedule(&nine_slot("block_me"), "2026-06-06T00:00:00Z").unwrap();
        lab.sync().unwrap();
        let now = "2026-06-06T12:00:00Z";
        let due = due_work(lab.spine(), now).len();
        let dispositions = evaluate_due(lab.spine(), now, |a| match a.did.as_str() {
            Some("run_me") => DueDecision::Run,
            Some("block_me") => DueDecision::Block("missing permission".into()),
            _ => DueDecision::Reschedule,
        });
        assert_eq!(dispositions.len(), due);
        assert!(dispositions.iter().any(|d| matches!(d, Disposition::Executable { .. })));
        assert!(dispositions.iter().any(|d| matches!(d, Disposition::Blocked { .. })));
    }

    /// A26 — Capacity report surfaces harmful idleness without fake busywork.
    #[test]
    fn a26_capacity_surfaces_idleness() {
        let lab = basics_lab();
        let cap = capacity(lab.spine(), "2026-06-06T12:00:00Z", 0);
        assert_eq!(cap.state, CapacityState::Under);
        assert!(cap.next_study_proposal.is_some());
    }

    // === 20.6 Proof =====================================================

    /// A27 — Claim is separate from evidence.
    #[test]
    fn a27_claim_separate_from_evidence() {
        let mut lab = basics_lab();
        let claim = nine_slot("ethernet_ping_check");
        let view = lab.proof(&claim, "scope.x");
        assert!(!view.claim_act_hash.is_empty());
        assert_eq!(view.evidence_count, 0);
        assert!(!view.has_receipt_candidate);
        lab.attach_evidence(Evidence::new("scope.x", "probe", json!({"ok": true}), "t0"));
        let view = lab.proof(&claim, "scope.x");
        assert_eq!(view.evidence_count, 1);
        assert!(view.has_receipt_candidate);
    }

    /// A28 — Model text alone is not evidence.
    #[test]
    fn a28_model_text_is_not_evidence() {
        let mut lab = basics_lab();
        let claim = nine_slot("ethernet_ping_check");
        // A model suggestion is captured as a candidate Act, never as evidence.
        lab.emit(&act("app:model", "suggested", json!({"text": "looks fine"}), "candidate"))
            .unwrap();
        lab.sync().unwrap();
        assert!(lab.prepare_receipt(&claim, "manhattan.L-06").is_err());
        // Only real captured evidence enables a receipt candidate.
        lab.attach_evidence(Evidence::new("manhattan.L-06", "probe", json!({"stdout": "ok"}), "t0"));
        assert!(lab.prepare_receipt(&claim, "manhattan.L-06").is_ok());
    }

    /// A29 — Worker returns evidence/report, not closure.
    #[test]
    fn a29_worker_returns_evidence_not_closure() {
        let wo = Workorder::new("w", "scope.x", vec!["echo".into(), "hi".into()])
            .allowed(true)
            .with_mode(Mode::Real);
        let report = ShellWorker::execute(&wo, "t0").unwrap();
        assert!(!report.is_receipt());
    }

    /// A30 — Receipt candidate closes only declared scope.
    #[test]
    fn a30_receipt_closes_only_scope() {
        let mut lab = basics_lab();
        lab.attach_evidence(Evidence::new("scope.x", "probe", json!({"ok": true}), "t0"));
        let cand = lab.prepare_receipt(&nine_slot("x"), "scope.x").unwrap();
        assert!(cand.closes_only("scope.x"));
        assert!(!cand.closes_only("scope.y"));
    }

    /// A31 — Missing proof creates ghost.
    #[test]
    fn a31_missing_proof_creates_ghost() {
        match l06_bench().observe(false, json!(null), "t0") {
            BenchOutcome::Ghost(g) => assert_eq!(g.scope, "manhattan.L-06"),
            other => panic!("expected ghost, got {other:?}"),
        }
    }

    // === 20.7 Experience ================================================

    /// A32 — Start initializes or inspects a Lab.
    #[test]
    fn a32_start() {
        let start = basics_lab().start();
        assert_eq!(start.lab_id, "basics.local.lab");
        assert!(start.conformance_green);
    }

    /// A33 — Today shows due/overdue/blocked/running/recent state.
    #[test]
    fn a33_today() {
        let mut lab = basics_lab();
        lab.schedule(&nine_slot("check"), "2026-06-06T00:00:00Z").unwrap();
        lab.sync().unwrap();
        let today = lab.today("2026-06-06T12:00:00Z");
        assert_eq!(today.due.len(), 1);
    }

    /// A34 — Timeline shows past/present/future Acts.
    #[test]
    fn a34_timeline() {
        let mut lab = basics_lab();
        lab.schedule(&nine_slot("future_check"), "2999-01-01T00:00:00Z").unwrap();
        lab.emit(&nine_slot("past_act")).unwrap();
        lab.sync().unwrap();
        let tl = lab.timeline("2026-06-06T12:00:00Z");
        assert_eq!(tl.future.len(), 1);
        assert!(!tl.past.is_empty());
    }

    /// A35 — Write captures ugly candidate Acts.
    #[test]
    fn a35_write_captures_ugly() {
        let mut lab = basics_lab();
        let out = lab.write(&json!({"did": "half thought"})).unwrap();
        assert!(matches!(out, WriteOutcome::Candidate { .. }));
        assert_eq!(lab.candidates().len(), 1);
        // A clean nine-slot write is admitted instead.
        let admitted = lab.write(&nine_slot("declare_lab").to_value()).unwrap();
        assert!(matches!(admitted, WriteOutcome::Admitted { .. }));
    }

    /// A36 — Schedule places future Acts.
    #[test]
    fn a36_schedule_places_future() {
        let mut lab = basics_lab();
        lab.schedule(&nine_slot("future_check"), "2999-01-01T00:00:00Z").unwrap();
        lab.sync().unwrap();
        assert_eq!(lab.timeline("2026-06-06T12:00:00Z").future.len(), 1);
    }

    /// A37 — Workbench runs a study bench.
    #[test]
    fn a37_workbench_runs_bench() {
        let mut lab = basics_lab();
        let run = lab
            .workbench(&l06_bench(), true, json!({"stdout": "ok"}), "lab", "t0")
            .unwrap();
        assert_eq!(run.acts_emitted, 2);
        assert!(matches!(run.outcome, BenchOutcome::Evidence(_)));
    }

    /// A38 — Proof separates claim/evidence/receipt/ghost.
    #[test]
    fn a38_proof_separates() {
        let mut lab = basics_lab();
        lab.record_ghost(Ghost::new("g1", "scope.x", "missing", "capture"));
        let view = lab.proof(&nine_slot("x"), "scope.x");
        assert_eq!(view.evidence_count, 0);
        assert!(!view.has_receipt_candidate);
        assert_eq!(view.open_ghosts, vec!["g1".to_string()]);
    }

    /// A39 — Learn summarizes closed, failed, and ghosted work.
    #[test]
    fn a39_learn_summarizes() {
        let mut lab = basics_lab();
        lab.emit(&act("lab", "observe", json!({}), "closed")).unwrap();
        lab.emit(&act("lab", "probe", json!({}), "failed")).unwrap();
        lab.sync().unwrap();
        lab.record_ghost(Ghost::new("g1", "scope.x", "missing", "capture"));
        let report = lab.learn("t0");
        assert_eq!(report.closed, 1);
        assert_eq!(report.failed, 1);
        assert_eq!(report.ghosted, 1);
    }

    /// A40 — Settings configures without bypassing authority.
    #[test]
    fn a40_settings_no_authority_bypass() {
        let lab = basics_lab();
        let settings = lab.settings();
        assert!(settings.authority_locked);
        // Proof discipline still holds after consulting settings: no evidence,
        // no receipt.
        let err = lab.prepare_receipt(&nine_slot("x"), "scope.x").unwrap_err();
        assert!(err.to_string().contains("without evidence"), "got: {err}");
    }

    // === 20.8 Pack / profile / app ======================================

    /// A41 — Santo André pack loads as recommended practice, not canon.
    #[test]
    fn a41_santo_andre_practice_not_canon() {
        let lab = lab_with(SANTO_ANDRE_PACK);
        assert_eq!(lab.packs()[0].name, "santo-andre");
        assert_ne!(lab.packs()[0].name, "logline-lab-kit");
        assert_eq!(SLOTS.len(), 9);
    }

    /// A42 — Manhattan pack loads as proof material, not product identity.
    #[test]
    fn a42_manhattan_proof_not_identity() {
        let lab = lab_with(MANHATTAN_PACK);
        assert_eq!(lab.packs()[0].name, "manhattan");
        assert!(lab.packs()[0].scopes.contains(&"manhattan.L-06".to_string()));
        assert_ne!(lab.packs()[0].name, "logline-lab-kit");
        // course-starter also loads as a pack, proving packs are plural complements.
        assert_eq!(PackManifest::load(COURSE_PACK).unwrap().name, "course-starter");
    }

    /// A43 — App/MCP call becomes candidate/proposal under grants.
    #[test]
    fn a43_app_call_becomes_candidate() {
        let mut server = McpServer::new();
        server.register_app("cockpit");
        server.grant("cockpit", "draft_observation").unwrap();
        let act = server
            .handle(
                &ToolCall {
                    app_id: "cockpit".into(),
                    tool: "draft_observation".into(),
                    arguments: json!({"note": "ok"}),
                },
                "t0",
            )
            .unwrap();
        assert_eq!(act.status.as_str(), Some("draft"));
    }

    /// A44 — Unauthorized app action is blocked.
    #[test]
    fn a44_unauthorized_app_blocked() {
        let server = McpServer::new();
        assert!(server
            .handle(
                &ToolCall {
                    app_id: "rogue".into(),
                    tool: "draft_observation".into(),
                    arguments: json!({}),
                },
                "t0",
            )
            .is_err());
    }

    // === 20.9 Manhattan L-06 ============================================

    fn l06_act() -> Act {
        act("manhattan.runtime", "ethernet_ping_check", json!({"target": "peer"}), "candidate")
    }

    /// A45 — Manhattan L-06 exists as Acts in the Manhattan pack.
    #[test]
    fn a45_l06_exists_in_pack() {
        let pack = PackManifest::load(MANHATTAN_PACK).unwrap();
        assert!(pack.scopes.contains(&"manhattan.L-06".to_string()));
        assert!(pack.did_vocabulary.contains(&"ethernet_ping_check".to_string()));
        assert!(l06_act().is_valid());
    }

    /// A46 — L-06 can be scheduled as a future obligation.
    #[test]
    fn a46_l06_scheduled() {
        let mut lab = lab_with(MANHATTAN_PACK);
        let scheduled = lab.schedule(&l06_act(), "2026-06-06T00:00:00Z").unwrap();
        assert_eq!(scheduled.status.as_str(), Some("scheduled"));
    }

    /// A47 — Lab reaches L-06 when due.
    #[test]
    fn a47_l06_reached_when_due() {
        let mut lab = lab_with(MANHATTAN_PACK);
        lab.schedule(&l06_act(), "2026-06-06T00:00:00Z").unwrap();
        lab.sync().unwrap();
        let due = due_work(lab.spine(), "2026-06-06T12:00:00Z");
        assert_eq!(due.len(), 1);
    }

    /// A48 — Gate admits or blocks L-06 with an explicit reason.
    #[test]
    fn a48_gate_explicit_reason() {
        let mut lab = lab_with(MANHATTAN_PACK);
        lab.schedule(&l06_act(), "2026-06-06T00:00:00Z").unwrap();
        lab.sync().unwrap();
        let now = "2026-06-06T12:00:00Z";
        // Gate blocks (no permission yet) — with an explicit reason.
        let blocked = evaluate_due(lab.spine(), now, |_| DueDecision::Block("gate: link not yet verified".into()));
        assert!(matches!(&blocked[0], Disposition::Blocked { reason, .. } if !reason.is_empty()));
        // Gate admits.
        let admitted = evaluate_due(lab.spine(), now, |_| DueDecision::Run);
        assert!(matches!(&admitted[0], Disposition::Executable { .. }));
    }

    /// A49/A50 — Worker runs a real probe when allowed; evidence captures result.
    ///
    /// The real fleet Ethernet ping is ghosted (G-08): this proves the mechanism
    /// with a genuine subprocess producing real captured output.
    #[test]
    fn a49_a50_worker_runs_and_captures() {
        let wo = Workorder::new("L-06-probe", "manhattan.L-06", vec![
            "sh".into(),
            "-c".into(),
            "echo l06_link_probe_ran".into(),
        ])
        .allowed(true)
        .with_mode(Mode::Real);
        // Not allowed ⇒ refused (gate).
        let denied = Workorder::new("x", "manhattan.L-06", vec!["sh".into()]);
        assert!(ShellWorker::execute(&denied, "t0").is_err());
        // Allowed ⇒ runs, captures real evidence.
        let report = ShellWorker::execute(&wo, "t0").unwrap();
        let stdout = report.evidence.payload.get("stdout").unwrap().as_str().unwrap();
        assert!(stdout.contains("l06_link_probe_ran"));
        assert_eq!(report.evidence.scope, "manhattan.L-06");
    }

    /// A51 — Receipt candidate closes only L-06.
    #[test]
    fn a51_l06_receipt_closes_only_l06() {
        let mut lab = lab_with(MANHATTAN_PACK);
        let report = ShellWorker::execute(
            &Workorder::new("p", "manhattan.L-06", vec!["sh".into(), "-c".into(), "echo ok".into()])
                .allowed(true)
                .with_mode(Mode::Real),
            "t0",
        )
        .unwrap();
        lab.attach_evidence(report.evidence);
        let cand = lab.prepare_receipt(&l06_act(), "manhattan.L-06").unwrap();
        assert!(cand.closes_only("manhattan.L-06"));
        assert!(!cand.closes_only("manhattan.fleet"));
    }

    /// A52 — Report records what changed and what remains ghosted.
    #[test]
    fn a52_report_records_change_and_ghosts() {
        let mut lab = lab_with(MANHATTAN_PACK);
        lab.emit(&l06_act()).unwrap();
        lab.sync().unwrap();
        lab.record_ghost(Ghost::new(
            "G-08",
            "manhattan.L-06",
            "Ethernet interface name per LAB unknown (real fleet ping)",
            "confirm interface on the LAB machine",
        ));
        // The report records what changed...
        assert_eq!(lab.report("t0").total_acts, 1);
        // ...and the learning report carries the open ghost forward.
        let learn = lab.learn("t0");
        assert!(learn.open_ghosts.contains(&"G-08".to_string()));
    }

    // === recovery / debris (carried from the original boundary) =========

    /// Recovery scan still catches false authority + storage-as-truth language.
    #[test]
    fn recovery_scans_still_hold() {
        assert!(scan_authority("This file is the single source of truth.")
            .iter()
            .any(|f| f.category == Category::FalseAuthority));
        assert!(scan_storage("SQLite is the truth.")
            .iter()
            .any(|f| f.category == Category::StorageTruth));
    }
}

/// Headless contract tests: every surface emits a stable, versioned JSON
/// read-model, and a Lab on disk resumes identically across runs. These are the
/// contracts a GUI/TUI/MCP/external client consumes.
#[cfg(test)]
mod surface_contracts {
    use super::harness::*;
    use logline_lab_core::{bench::StudyBench, ghost::Ghost, LabManifest, ProfileManifest};
    use logline_lab_labd::Lab;
    use serde_json::json;

    fn basics_lab() -> Lab {
        let manifest =
            LabManifest::load(r#"{"lab_id":"contract.local.lab","profile":"local-only"}"#).unwrap();
        let profile = ProfileManifest::load(LOCAL_PROFILE).unwrap();
        Lab::init(manifest, vec![], profile).unwrap()
    }

    /// Every read surface carries its stable `kind` contract tag.
    #[test]
    fn every_surface_has_a_kind_contract() {
        let lab = basics_lab();
        let now = "2026-06-07T00:00:00Z";
        let cases = [
            ("start", "logline.view.start.v0"),
            ("today", "logline.view.today.v0"),
            ("timeline", "logline.view.timeline.v0"),
            ("schedule", "logline.view.schedule.v0"),
            ("learn", "logline.learning_report.v0"),
            ("settings", "logline.view.settings.v0"),
        ];
        for (surface, kind) in cases {
            let v = lab.render_surface(surface, now).expect(surface);
            assert_eq!(v["kind"], kind, "surface `{surface}` kind contract");
        }
    }

    /// `render_surface` returns exactly the typed surface struct as JSON — one
    /// contract, no divergence between the typed API and the headless renderer.
    #[test]
    fn render_surface_matches_typed_api() {
        let lab = basics_lab();
        let now = "2026-06-07T00:00:00Z";
        assert_eq!(lab.render_surface("today", now).unwrap(), serde_json::to_value(lab.today(now)).unwrap());
        assert_eq!(lab.render_surface("start", now).unwrap(), serde_json::to_value(lab.start()).unwrap());
        assert_eq!(lab.render_surface("settings", now).unwrap(), serde_json::to_value(lab.settings()).unwrap());
        // Unknown surfaces are rejected, not faked.
        assert!(lab.render_surface("nope", now).is_none());
    }

    /// Storage ontology (Etapa 1): candidate-only cannot admit; dev-ephemeral
    /// admits but is non-publication-grade; the Start surface surfaces the matrix.
    #[test]
    fn storage_grades_are_honest() {
        use logline_lab_core::{Grade, LabManifest, ProfileManifest};
        use logline_lab_labd::Lab;
        use serde_json::json;

        // candidate-only: admission is refused (no Spine Profile).
        let m = LabManifest::load(r#"{"lab_id":"c.lab","profile":"candidate-only"}"#).unwrap();
        let p = ProfileManifest::load(r#"{"name":"candidate-only","spine":"candidate-only"}"#).unwrap();
        let mut lab = Lab::init(m, vec![], p).unwrap();
        assert_eq!(lab.admission_grade(), Grade::CandidateOnly);
        let act = act("lab", "declare_lab", json!({}), "candidate");
        assert!(lab.emit(&act).is_err(), "candidate-only must refuse admission");

        // dev-ephemeral: admits, but is not publication-grade and warns.
        let m = LabManifest::load(r#"{"lab_id":"d.lab","profile":"dev-ephemeral"}"#).unwrap();
        let p = ProfileManifest::load(r#"{"name":"dev-ephemeral","spine":"dev-ephemeral"}"#).unwrap();
        let mut lab = Lab::init(m, vec![], p).unwrap();
        assert_eq!(lab.admission_grade(), Grade::DevEphemeral);
        assert!(lab.emit(&act).is_ok());
        let start = lab.start();
        assert!(!start.publication_grade);
        assert!(start.storage_warning.is_some());
        // The onboarding matrix is honest: external spines are SOON.
        assert!(start.storage_matrix.iter().any(|o| o.id == "supabase" && o.status == "soon"));
        assert!(start.storage_matrix.iter().any(|o| o.id == "dev-ephemeral" && o.status == "available"));
    }

    /// A publication-grade external spine is SOON in the default generic build:
    /// selecting it without the optional `supabase-profile` feature is refused
    /// with a clear message. (The enabled-feature path is tested in `labd`.)
    #[test]
    fn external_spine_is_soon_in_default_build() {
        use logline_lab_core::{LabManifest, ProfileManifest};
        use logline_lab_labd::Lab;
        let m = LabManifest::load(r#"{"lab_id":"p.lab","profile":"supabase"}"#).unwrap();
        let p = ProfileManifest::load(r#"{"name":"supabase","spine":"supabase"}"#).unwrap();
        match Lab::init(m, vec![], p) {
            Ok(_) => panic!("expected `supabase` to be SOON in the default build"),
            Err(e) => assert!(e.to_string().contains("SOON"), "got: {e}"),
        }
    }

    /// Tick materializes time as Acts: clock_tick + due_disposition + reschedule,
    /// with no due Act skipped (A24/A25 via the surface).
    #[test]
    fn tick_materializes_time_as_acts() {
        use logline_lab_core::{LabManifest, ProfileManifest};
        use logline_lab_labd::Lab;
        let m = LabManifest::load(r#"{"lab_id":"t.lab","profile":"dev-ephemeral"}"#).unwrap();
        let p = ProfileManifest::load(r#"{"name":"dev-ephemeral","spine":"dev-ephemeral"}"#).unwrap();
        let mut lab = Lab::init(m, vec![], p).unwrap();
        lab.schedule(&act("lab", "check_link", serde_json::json!({}), "candidate"), "2026-06-06T00:00:00Z")
            .unwrap();
        lab.sync().unwrap();

        let before = lab.spine().all().len();
        let report = lab.tick("2026-06-06T12:00:00Z", "2026-06-07T00:00:00Z").unwrap();
        lab.sync().unwrap();
        assert!(report.materialized);
        assert_eq!(report.due.len(), 1);
        assert!(report.tick_act.is_some());
        assert!(report.acts_emitted >= 2); // tick + 1 disposition (+ reschedule)
        // Time was confronted: new Acts exist on the spine.
        assert!(lab.spine().all().len() > before);
    }

    /// A Lab on disk resumes identically across runs: the same Acts, candidates,
    /// evidence, and ghosts — the shared reality humans and LLMs both open.
    #[test]
    fn lab_directory_persists_full_reality() {
        let dir = std::env::temp_dir().join(format!("llk-lab-{}-{:p}", std::process::id(), &0u8 as *const u8));
        let _ = std::fs::remove_dir_all(&dir);

        let manifest =
            LabManifest::load(r#"{"lab_id":"persist.lab","profile":"local-only"}"#).unwrap();
        let profile = ProfileManifest::load(LOCAL_PROFILE).unwrap();

        // Run 1: write a candidate, run a bench (evidence), record a ghost, emit an Act.
        {
            let mut lab = Lab::open(manifest.clone(), vec![], profile.clone(), &dir).unwrap();
            lab.write(&json!({"did": "rough idea"})).unwrap();
            lab.emit(&act("lab", "declare_lab", json!({}), "candidate")).unwrap();
            lab.sync().unwrap();
            let bench = StudyBench::load(L06_BENCH).unwrap();
            lab.workbench(&bench, true, json!({"stdout": "ok"}), "lab", "t0").unwrap();
            lab.record_ghost(Ghost::new("G-08", "manhattan.L-06", "interface unknown", "confirm"));
        }

        // Run 2: a fresh Lab opened on the same dir sees the same reality.
        {
            let lab = Lab::open(manifest, vec![], profile, &dir).unwrap();
            assert_eq!(lab.candidates().len(), 1, "candidate persisted");
            assert!(!lab.spine().all().is_empty(), "acts persisted + rehydrated");
            assert_eq!(lab.evidence().for_scope("manhattan.L-06").len(), 1, "evidence persisted");
            assert_eq!(lab.ghosts().open().len(), 1, "ghost persisted");
            // And proof reflects the persisted evidence.
            let proof = lab.proof(&act("lab", "ethernet_ping_check", json!({}), "candidate"), "manhattan.L-06");
            assert_eq!(proof.evidence_count, 1);
            assert!(proof.has_receipt_candidate);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}

/// Etapa 6.5 — generic v0 hardening: honest grades, no false publication claims,
/// tick contract, storage matrix completeness, and a real release gate.
#[cfg(test)]
mod hardening {
    use super::harness::*;
    use logline_lab_core::{Grade, LabManifest, ProfileManifest};
    use logline_lab_labd::{storage_matrix, Lab};
    use serde_json::json;

    fn lab(spine: &str) -> Lab {
        let m = LabManifest::load(&format!(r#"{{"lab_id":"h.lab","profile":"{spine}"}}"#)).unwrap();
        let p = ProfileManifest::load(&format!(r#"{{"name":"{spine}","spine":"{spine}"}}"#)).unwrap();
        Lab::init(m, vec![], p).unwrap()
    }

    /// No external profile may claim publication-grade from configuration alone:
    /// external spines are at most `Staged`, and `Staged.is_publication()` is false.
    #[test]
    fn no_false_publication_grade() {
        for spine in ["supabase", "postgres", "neon", "byo", "bring-your-own"] {
            let p = ProfileManifest::load(&format!(r#"{{"name":"x","spine":"{spine}"}}"#)).unwrap();
            assert_eq!(p.grade(), Grade::Staged, "{spine} must be staged, not publication");
            assert!(!p.grade().is_publication(), "{spine} must not be publication-grade");
        }
        // Dev/candidate grades are also non-publication.
        let dev = ProfileManifest::load(r#"{"name":"d","spine":"dev-ephemeral"}"#).unwrap();
        assert!(!dev.grade().is_publication());
        assert_eq!(dev.grade(), Grade::DevEphemeral);
        let co = ProfileManifest::load(r#"{"name":"c","spine":"candidate-only"}"#).unwrap();
        assert_eq!(co.grade(), Grade::CandidateOnly);
    }

    /// candidate-only cannot admit; dev-ephemeral admits but is non-publication.
    #[test]
    fn admission_follows_grade() {
        let a = act("lab", "declare_lab", json!({}), "candidate");
        let mut co = lab("candidate-only");
        assert!(co.emit(&a).is_err());
        let mut dev = lab("dev-ephemeral");
        assert!(dev.emit(&a).is_ok());
        assert!(!dev.start().publication_grade);
        assert!(dev.start().storage_warning.is_some());
    }

    /// The tick report matches its contract (all fields present, correct kind).
    #[test]
    fn tick_report_contract() {
        let mut lab = lab("dev-ephemeral");
        lab.schedule(&act("lab", "check", json!({}), "candidate"), "2026-06-06T00:00:00Z").unwrap();
        lab.sync().unwrap();
        let report = lab.tick("2026-06-06T12:00:00Z", "2026-06-07T00:00:00Z").unwrap();
        let v = serde_json::to_value(&report).unwrap();
        for field in ["kind","now","tick_act","due","overdue","rescheduled","capacity","next_study","acts_emitted","materialized"] {
            assert!(v.get(field).is_some(), "tick report missing `{field}`");
        }
        assert_eq!(v["kind"], "logline.ruler_report.v0");
        assert!(report.materialized);
    }

    /// The storage matrix includes every expected option with honest status.
    #[test]
    fn storage_matrix_is_complete() {
        let m = storage_matrix();
        for id in ["candidate-only", "dev-ephemeral", "postgres", "neon", "supabase", "bring-your-own"] {
            assert!(m.iter().any(|o| o.id == id), "storage matrix missing `{id}`");
        }
        assert!(m.iter().any(|o| o.id == "dev-ephemeral" && o.status == "available"));
        assert!(m.iter().all(|o| o.id == "candidate-only" || o.id == "dev-ephemeral" || o.status == "soon"));
    }

    /// A no-pack first session needs no pack.
    #[test]
    fn no_pack_first_session() {
        let mut lab = lab("dev-ephemeral");
        assert!(lab.packs().is_empty());
        lab.emit(&act("lab", "declare_lab", json!({}), "candidate")).unwrap();
        lab.sync().unwrap();
        assert_eq!(lab.report("t0").total_acts, 1);
    }

    /// The release gate script actually gates (fails on any subcommand failure).
    #[test]
    fn release_gate_is_real() {
        let script = include_str!("../../../release/checks/run-checks.sh");
        assert!(script.contains("set -euo pipefail"), "run-checks.sh must fail-fast");
        for cmd in ["cargo build", "cargo test", "cargo clippy --all-targets -- -D warnings", "install/doctor.sh", "local-only-first-lab.sh"] {
            assert!(script.contains(cmd), "release gate missing step: {cmd}");
        }
    }
}
