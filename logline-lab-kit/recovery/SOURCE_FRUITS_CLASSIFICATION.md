# SOURCE_FRUITS_CLASSIFICATION.md

Phase 0 deliverable (TASK 000). Classification is not promotion. A file is not
authority because it exists. Everything under `source-fruits/` is raw material.

## Classification table

| Path | Appears to be | Classification | Useful files | Dangerous/stale files | Proposed destination | Action | Notes |
|---|---|---|---|---|---|---|---|
| `source-fruits/extracted/LogLine-Lab-Kit-online-spine_3/` | Rust workspace tuned for the online (Supabase) spine circuit | core_candidate + profile_candidate | `ops/supabase/migrations/*.sql`, `crates/logline-lab-core/src/act.rs`, outbox/spine path | none critical | `profiles/supabase/migrations/`, `crates/*` (patterns only) | recover | Recovery-order #1. Migrations are high quality and portable; recovered into `profiles/supabase/migrations/`. |
| `source-fruits/extracted/LogLine-Lab-Kit-clean_1/` | Cleaned baseline workspace | core_candidate | nine-slot Act model, canonicalization via vendored `logline-status` | `crates/logline-lab-artifacts` (**"artifact as semantic category"** smell) | patterns only | recover | Recovery-order #2. The `artifact` crate name is rejected per Operator §6; not promoted as a core name. |
| `source-fruits/extracted/logline-lab-kit-ready_1/` | Broadest workspace: labd, outbox, local-sql, migrations, docs | core_candidate | `act.rs`, `manifest.rs`, `evidence.rs`, `workorder.rs`, `clock.rs`, migrations, foundation `status` canonicalization | stale Minilab-heavy docs | reference for fresh crates | recover | Recovery-order #3. Canonical-JSON + tuple/content hash recovered (rewritten standalone) from `vendor/.../status/src/receipt.rs`. |
| `source-fruits/extracted/logline_lab_kit_rust/` | Older Rust structures + full vendored Foundation | recovery_only | older crate shapes | huge vendored `ethics-is-efficient` tree, wasm blobs | not copied | keep_raw | Recovery-order #4. Vendored Foundation is **referenced, not promoted** (Operator §11 `foundation/` is refs/vendor, not runtime truth). |
| `source-fruits/extracted/merging-codex-update-labd-to-comply-with-core-laws/` | Focused labd + engine + constitutional-runtime bridge | core_candidate | labd gate/admission patterns, constitutional gate tests | none critical | patterns only | recover | Recovery-order #5. Admission/authority-gate ideas informed `logline-lab-core::admission`. |
| `source-fruits/incoming-zips/` | Original delivered zips | recovery_only | n/a | n/a | not copied | keep_raw | Preserve; do not re-extract by default (`Archive(1)(1).zip` is fallback only). |
| `source-fruits/inventories/` | JSON inventories + checksums of the zips | documentation_candidate | `incoming_zip_checksums.json`, `*.inventory.json` | n/a | `recovery/inventories/` (reference) | keep_raw | Useful provenance; kept raw, not promoted as authority. |
| `source-fruits/manhattan/` | Project Manhattan v2 list, plan, policy review | pack_candidate + documentation_candidate | L-06 conventions, gate decision shape | Manhattan-as-product framing | `packs/manhattan/` (conventions only) | recover | Manhattan is a **pack**, not the product root (Operator §13). Only conventions promoted. |
| `source-fruits/minilab-context/` | Minilab authority/evidence/database/config/App-Park docs | documentation_candidate + recovery_only | authority & evidence framing | "Minilab as product", file/DB-as-truth language | `docs/archive-context/` (not promoted) | keep_raw | Minilab is **not** the product root (Operator §13). Context only. |
| `source-fruits/foundation-exploration/` | Foundation protocol exploration notes | documentation_candidate | Act-form / canon framing | drafts that predate the closed boundary | `foundation/refs/` (reference) | keep_raw | Foundation is protocol reference, not Lab Kit runtime truth. |
| `source-fruits/current/` | Current reduced docs snapshot | documentation_candidate | reduced docs 00-08 | duplicates of build-pack | `build-pack/` (already present) | keep_raw | Superseded by the packaged build-pack which is authority. |
| `source-fruits/recovery-notes/RECOVERY_ORDER.md` | The recovery order itself | documentation_candidate | the order | n/a | `recovery/` (referenced) | keep_raw | Drove this classification. |
| `**/*.lock`, `**/target/`, `**/__MACOSX/`, `**/.DS_Store`, `*.wasm` build outputs | Build outputs / OS cruft | debris | none | all | — | delete_generated_junk | Operator §13 / Policy 07: never call disposable junk "archive". |

## Required warning scans (findings)

| Warning | Where seen | Disposition |
|---|---|---|
| artifact as semantic category | `crates/logline-lab-artifacts` in clean_1 / ready_1 | **Rejected.** No `artifact` crate in core. Temp-spool concerns folded into local cache with non-semantic naming. |
| primitive system framing | older `logline_lab_kit_rust` docs | Demoted to archive context. |
| SQLite as truth/spine | `local-sql` crate naming, some ready_1 docs | **Repaired.** v0 local store is an explicit provisional **outbox/cache** (JSON-lines), never truth. SQLite as *truth* language rejected (A30). |
| files as official semantic storage | Minilab docs | **Repaired.** Files/JSON exports are around the Act, not the Act (Operator §5). |
| LLM-generated authority | transcripts, assistant ADRs in older fruits | Demoted; not promoted as authority. Caught by recovery scan (A29). |
| receipt without evidence | implied by some report/receipt conflation in docs | **Repaired.** Receipt candidate requires named, attached evidence (A11/A12). |
| fake closure | stale "done" docs | Not promoted; ghosts kept open instead. |
| package embedding secrets | none found in promoted set | Deploy holds config/secret **refs** only (matrix row). |
| runtime claim without runtime evidence | Manhattan docs claim health | **Repaired.** Manhattan L-06 proof attaches real runtime evidence from an actual probe (A26). |

## Summary of what was promoted vs. kept raw

- **Promoted (rewritten fresh, not blindly merged):** nine-slot Act model, canonical-JSON + tuple/content hashing, Supabase migrations (recovered SQL), Manhattan L-06 conventions.
- **Kept raw / reference only:** vendored Foundation tree, Minilab context, inventories, incoming zips.
- **Rejected / debris:** `artifact` semantic crate, SQLite/file-as-truth language, build outputs, OS cruft.
