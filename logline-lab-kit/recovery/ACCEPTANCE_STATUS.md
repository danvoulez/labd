# ACCEPTANCE_STATUS.md

Acceptance gates **A01–A52** from `build-pack/final-real-project-doc.md` §20.
A test is green only with command output / inspected evidence.

**Verified:** `cargo test` = **94 passed, 0 failed** (whole workspace);
`cargo clippy --workspace --all-targets` = **0 warnings**;
`bash install/doctor.sh` runs offline conformance + a real first session on the
basics (no pack). The end-to-end A01–A52 mapping lives in
`crates/logline-lab-acceptance` (51 tests; `a24_a25` and `a49_a50` are combined).

| ID | Acceptance test | Status | Evidence (test) | Module |
|---|---|---|---|---|
| A01 | Act has exactly nine canonical slots | green | `a01_exactly_nine_slots` | `logline-act` |
| A02 | Unknown domains don't become native authority | green | `a02_no_native_domain_authority` | `logline-act` |
| A03 | Candidate Acts can be ugly | green | `a03_ugly_candidates_allowed` | `logline-act` |
| A04 | Promotion requires validation | green | `a04_promotion_requires_validation` | `logline-act` |
| A05 | Hash/canonicalization is stable | green | `a05_hash_stable` | `logline-act` |
| A06 | User can initialize a Lab (basics, no pack) | green | `a06_initialize_lab` | `logline-lab-labd` |
| A07 | Lab manifest loads | green | `a07_lab_manifest_loads` | `logline-lab-core` |
| A08 | Profile loads | green | `a08_profile_loads` | `logline-lab-core` |
| A09 | Pack loads without mutating core | green | `a09_pack_loads_without_mutating_core` | `logline-lab-core` |
| A10 | First candidate Act is created | green | `a10_first_candidate_created` | `logline-lab-labd` |
| A11 | First admitted Act is stored | green | `a11_first_admitted_stored` | `logline-lab-labd` |
| A12 | First Lab report renders | green | `a12_first_report_renders` | `logline-lab-reports` |
| A13 | Conformance vectors run | green | `a13_conformance_vectors_run` | `logline-lab-conformance` |
| A14 | Conformance report is generated | green | `a14_conformance_report_green` | `logline-lab-conformance` |
| A15 | Example Acts can be exported | green | `a15_examples_exported` | `logline-lab-conformance` |
| A16 | Another Lab can compare examples | green | `a16_examples_reproducible` | `logline-lab-conformance` |
| A17 | No central hosted service required | green | `a17_no_central_service_required` | `logline-lab-conformance` |
| A18 | Study bench declares its fields | green | `a18_bench_declares_fields` | `logline-lab-core::bench` |
| A19 | Bench produces Acts | green | `a19_bench_produces_acts` | `logline-lab-core::bench` |
| A20 | Observation becomes evidence or ghost | green | `a20_observation_evidence_or_ghost` | `logline-lab-core::bench` |
| A21 | Learning report proposes next Act | green | `a21_learning_proposes_next` | `logline-lab-reports::learning` |
| A22 | Future Act can be scheduled | green | `a22_schedule_future_act` | `logline-lab-labd` |
| A23 | Tick/check discovers due work | green | `a23_discovers_due_work` | `logline-lab-ruler` |
| A24 | Due work resolves/blocks/reschedules visibly | green | `a24_a25_dispositions_cover_all_due` | `logline-lab-ruler` |
| A25 | No due Act is skipped silently | green | `a24_a25_dispositions_cover_all_due` | `logline-lab-ruler` |
| A26 | Capacity surfaces idleness without fake busywork | green | `a26_capacity_surfaces_idleness` | `logline-lab-ruler` |
| A27 | Claim is separate from evidence | green | `a27_claim_separate_from_evidence` | `logline-lab-labd` (Proof) |
| A28 | Model text alone is not evidence | green | `a28_model_text_is_not_evidence` | `logline-lab-core` |
| A29 | Worker returns evidence, not closure | green | `a29_worker_returns_evidence_not_closure` | `logline-lab-dispatch` |
| A30 | Receipt candidate closes only declared scope | green | `a30_receipt_closes_only_scope` | `logline-lab-core` |
| A31 | Missing proof creates ghost | green | `a31_missing_proof_creates_ghost` | `logline-lab-core` |
| A32 | Start initializes/inspects a Lab | green | `a32_start` | `logline-lab-labd` |
| A33 | Today shows due/overdue/blocked/running/recent | green | `a33_today` | `logline-lab-labd` |
| A34 | Timeline shows past/present/future | green | `a34_timeline` | `logline-lab-labd` |
| A35 | Write captures ugly candidate Acts | green | `a35_write_captures_ugly` | `logline-lab-labd` |
| A36 | Schedule places future Acts | green | `a36_schedule_places_future` | `logline-lab-labd` |
| A37 | Workbench runs a study bench | green | `a37_workbench_runs_bench` | `logline-lab-labd` |
| A38 | Proof separates claim/evidence/receipt/ghost | green | `a38_proof_separates` | `logline-lab-labd` |
| A39 | Learn summarizes closed/failed/ghosted | green | `a39_learn_summarizes` | `logline-lab-reports` |
| A40 | Settings configures without bypassing authority | green | `a40_settings_no_authority_bypass` | `logline-lab-labd` |
| A41 | Santo André pack loads as practice, not canon | green | `a41_santo_andre_practice_not_canon` | `packs/santo-andre` |
| A42 | Manhattan pack loads as proof, not identity | green | `a42_manhattan_proof_not_identity` | `packs/manhattan` |
| A43 | App/MCP call becomes candidate under grants | green | `a43_app_call_becomes_candidate` | `apps/mcp-server` |
| A44 | Unauthorized app action is blocked | green | `a44_unauthorized_app_blocked` | `apps/mcp-server` |
| A45 | Manhattan L-06 exists as Acts in the pack | green | `a45_l06_exists_in_pack` | `packs/manhattan` |
| A46 | L-06 can be scheduled as future obligation | green | `a46_l06_scheduled` | `logline-lab-labd` |
| A47 | Lab reaches L-06 when due | green | `a47_l06_reached_when_due` | `logline-lab-ruler` |
| A48 | Gate admits/blocks L-06 with explicit reason | green | `a48_gate_explicit_reason` | `logline-lab-ruler` |
| A49 | Worker runs a real probe when allowed | green | `a49_a50_worker_runs_and_captures` | `logline-lab-dispatch` |
| A50 | Evidence captures the probe result | green | `a49_a50_worker_runs_and_captures` | `logline-lab-dispatch` |
| A51 | Receipt candidate closes only L-06 | green | `a51_l06_receipt_closes_only_l06` | `logline-lab-core` |
| A52 | Report records what changed and what is ghosted | green | `a52_report_records_change_and_ghosts` | `logline-lab-reports` |

## Honesty caveats
- **A08/A17** are green against the in-process spine (the `local-only` profile).
  The Supabase spine stages content-addressed payloads; live network ingest is
  ghosted (`supabase-live-ingest`).
- **A49/A50** use a genuine subprocess for real runtime evidence. The real fleet
  Ethernet ping (per-LAB interface) stays ghosted (`G-08`); the proof demonstrates
  the full mechanism.
- The original Act-machine gates A1–A30 are superseded by A01–A52 but their
  mechanics remain backed by per-crate unit tests (94 total).
