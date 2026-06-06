# ACCEPTANCE_STATUS.md

Phase 0 deliverable (TASK 000). A test is green ONLY with command output or
inspected-file evidence (FIRST_RUN_PROTOCOL §3). This file is written before
implementation (all `red`/`unknown`) and updated with real evidence after each
implementation run. **See the "Post-build status" section at the bottom for the
current verified state.**

## Initial status (pre-implementation)

| ID | Acceptance test | Status | Test command | Responsible module |
|---|---|---|---|---|
| A1 | Act with exactly nine slots validates. | red | `cargo test -p logline-act` | `crates/logline-act` |
| A2 | Act with a missing slot fails. | red | `cargo test -p logline-act` | `crates/logline-act` |
| A3 | Act with a tenth canonical slot fails. | red | `cargo test -p logline-act` | `crates/logline-act` |
| A4 | Same Act produces same canonical hash. | red | `cargo test -p logline-act` | `crates/logline-act` |
| A5 | Ugly candidate can be preserved. | red | `cargo test -p logline-act` | `crates/logline-act` |
| A6 | Local emit stores Act in local cache/outbox. | red | `cargo test -p logline-lab-local` | `crates/logline-lab-local` |
| A7 | Re-emitting same Act is idempotent. | red | `cargo test -p logline-lab-local` | `crates/logline-lab-local` |
| A8 | Sync writes Act to configured spine. | red | `cargo test -p logline-lab-spine` | `crates/logline-lab-spine` |
| A9 | Projection reads Act from spine. | red | `cargo test -p logline-lab-projectors` | `crates/logline-lab-projectors` |
| A10 | Blocked Act appears when evidence/permission missing. | red | `cargo test -p logline-lab-core` | `crates/logline-lab-core` |
| A11 | Evidence can be attached to scope. | red | `cargo test -p logline-lab-core` | `crates/logline-lab-core` |
| A12 | Receipt candidate names exact scope. | red | `cargo test -p logline-lab-core` | `crates/logline-lab-core` |
| A13 | Report does not pretend to be receipt. | red | `cargo test -p logline-lab-reports` | `crates/logline-lab-reports` |
| A14 | Pack loads without changing core. | red | `cargo test -p logline-lab-core` | `crates/logline-lab-core` |
| A15 | Profile loads without changing core. | red | `cargo test -p logline-lab-core` | `crates/logline-lab-core` |
| A16 | App/MCP call becomes draft Act. | red | `cargo test -p logline-lab-core` | `packages/mcp-server` + core app boundary |
| A17 | Unauthorized app action is blocked. | red | `cargo test -p logline-lab-core` | `packages/mcp-server` + core app boundary |
| A18 | Worker cannot execute without allow. | red | `cargo test -p logline-lab-dispatch` | `crates/logline-lab-dispatch` |
| A19 | Worker returns evidence, not closure. | red | `cargo test -p logline-lab-dispatch` | `crates/logline-lab-dispatch` |
| A20 | Clock tick emits or evaluates due Acts. | red | `cargo test -p logline-lab-clock` | `crates/logline-lab-clock` |
| A21 | Due Act resolves or creates reschedule Act. | red | `cargo test -p logline-lab-clock` | `crates/logline-lab-clock` |
| A22 | Lab report renders state from projections. | red | `cargo test -p logline-lab-reports` | `crates/logline-lab-reports` |
| A23 | Demo pack completes first session. | red | `cargo test -p logline-lab-acceptance a23` | `packs/demo` |
| A24 | Santo André pack loads without becoming core. | red | `cargo test -p logline-lab-acceptance a24` | `packs/santo-andre` |
| A25 | Manhattan pack loads without becoming core. | red | `cargo test -p logline-lab-acceptance a25` | `packs/manhattan` |
| A26 | Manhattan L-06 emits evidence. | red | `cargo test -p logline-lab-acceptance a26` | `packs/manhattan` + dispatch |
| A27 | L-06 receipt closes only L-06. | red | `cargo test -p logline-lab-acceptance a27` | `packs/manhattan` + core |
| A28 | Projection shows L-06 health from Acts. | red | `cargo test -p logline-lab-acceptance a28` | `packs/manhattan` + projectors |
| A29 | Recovery scan catches false authority. | red | `cargo test -p logline-lab-acceptance a29` | `recovery/` scanner |
| A30 | Storage scan rejects file/SQLite truth language. | red | `cargo test -p logline-lab-acceptance a30` | `recovery/` scanner |

---

## Post-build status — VERIFIED

Evidence: `cargo test` (whole workspace) = **43 passed, 0 failed**;
`cargo clippy --workspace --all-targets` = **0 warnings**;
`bash install/doctor.sh` runs a real emit→sync→report session.

| ID | Status | Evidence (test name) | Test command | Responsible module |
|---|---|---|---|---|
| A1 | green | `a1_nine_slots_validate` | `cargo test -p logline-act` | `crates/logline-act` |
| A2 | green | `a2_missing_slot_fails`, `a2_empty_slot_fails` | `cargo test -p logline-act` | `crates/logline-act` |
| A3 | green | `a3_tenth_slot_fails` | `cargo test -p logline-act` | `crates/logline-act` |
| A4 | green | `a4_same_act_same_hash` | `cargo test -p logline-act` | `crates/logline-act` |
| A5 | green | `a5_ugly_candidate_preserved` | `cargo test -p logline-act` | `crates/logline-act` |
| A6 | green | `a6_local_emit_stores` (durable reopen) | `cargo test -p logline-lab-local` | `crates/logline-lab-local` |
| A7 | green | `a7_reemit_idempotent` | `cargo test -p logline-lab-local` | `crates/logline-lab-local` |
| A8 | green | `a8_sync_writes_to_spine` | `cargo test -p logline-lab-spine` | `crates/logline-lab-spine` |
| A9 | green | `a9_projection_reads_from_spine` | `cargo test -p logline-lab-projectors` | `crates/logline-lab-projectors` |
| A10 | green | `a10_blocked_on_missing_permission_and_evidence` | `cargo test -p logline-lab-core` | `crates/logline-lab-core` |
| A11 | green | `a11_evidence_attaches_to_scope` | `cargo test -p logline-lab-core` | `crates/logline-lab-core` |
| A12 | green | `a12_receipt_candidate_names_exact_scope` | `cargo test -p logline-lab-core` | `crates/logline-lab-core` |
| A13 | green | `a13_report_is_not_receipt` | `cargo test -p logline-lab-reports` | `crates/logline-lab-reports` |
| A14 | green | `a14_pack_loads_without_changing_core` | `cargo test -p logline-lab-core` | `crates/logline-lab-core` |
| A15 | green | `a15_profile_loads_without_changing_core` | `cargo test -p logline-lab-core` | `crates/logline-lab-core` |
| A16 | green | `a16_app_call_becomes_draft_act`, `authorized_tool_call_becomes_draft_act` | `cargo test -p logline-lab-core -p logline-lab-mcp-server` | core + `packages/mcp-server` |
| A17 | green | `a17_unauthorized_app_blocked`, `unauthorized_tool_call_rejected` | `cargo test -p logline-lab-core -p logline-lab-mcp-server` | core + `packages/mcp-server` |
| A18 | green | `a18_worker_requires_allow` | `cargo test -p logline-lab-dispatch` | `crates/logline-lab-dispatch` |
| A19 | green | `a19_worker_returns_evidence_not_closure` | `cargo test -p logline-lab-dispatch` | `crates/logline-lab-dispatch` |
| A20 | green | `a20_tick_evaluates_due_acts` | `cargo test -p logline-lab-clock` | `crates/logline-lab-clock` |
| A21 | green | `a21_unresolved_due_act_reschedules` | `cargo test -p logline-lab-clock` | `crates/logline-lab-clock` |
| A22 | green | `a22_report_renders_from_projections` | `cargo test -p logline-lab-reports` | `crates/logline-lab-reports` |
| A23 | green | `a23_demo_pack_first_session` | `cargo test -p logline-lab-acceptance` | `packs/demo` |
| A24 | green | `a24_santo_andre_pack_loads_without_becoming_core` | `cargo test -p logline-lab-acceptance` | `packs/santo-andre` |
| A25 | green | `a25_manhattan_pack_loads_without_becoming_core` | `cargo test -p logline-lab-acceptance` | `packs/manhattan` |
| A26 | green | `a26_manhattan_l06_emits_evidence` (real subprocess stdout) | `cargo test -p logline-lab-acceptance` | `packs/manhattan` + dispatch |
| A27 | green | `a27_l06_receipt_closes_only_l06` | `cargo test -p logline-lab-acceptance` | `packs/manhattan` + core |
| A28 | green | `a28_projection_shows_l06_health` | `cargo test -p logline-lab-acceptance` | `packs/manhattan` + projectors |
| A29 | green | `a29_recovery_scan_catches_false_authority`, `a29_catches_false_authority` | `cargo test -p logline-lab-acceptance -p logline-lab-recovery-scanner` | `recovery/scanner` |
| A30 | green | `a30_storage_scan_rejects_truth_language`, `a30_rejects_storage_truth_language` | `cargo test -p logline-lab-acceptance -p logline-lab-recovery-scanner` | `recovery/scanner` |

### Notes / honesty caveats
- **A8/A9** are green against the in-memory spine (the `local-only` profile's
  configured spine). The Supabase spine is staged and content-addressed; live
  network ingest is ghosted (`supabase-live-ingest`).
- **A26** uses a real executed subprocess for genuine runtime evidence. The real
  fleet Ethernet ping to peers stays ghosted (`G-08`); the proof demonstrates the
  full chain mechanism, not the live fleet link.
- No test is marked green without a named test and command output.
