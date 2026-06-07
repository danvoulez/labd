> **Historical.** Superseded for generic v0 by `recovery/RELEASE_SCOPE.md` and `recovery/ACCEPTANCE_STATUS.md`. Kept for the record; figures like "43 tests", "A1–A30", "demo pack", and any "outbox is the record" / Supabase-as-default language reflect an earlier state.

# Recovery Receipt — LogLine Lab Kit v0 assembly

This is a **scoped** recovery record. It closes only what evidence proves
(Operator §14). It is not a blanket "done".

```txt
scope:        logline-lab-kit.recovery.v0-assembly
who:          operator
did:          assemble_logline_lab_kit_v0
confirmed_by: evidence
status:       prepared
```

## What was proven (with evidence)

| Claim | Evidence |
|---|---|
| Phase 0 deliverables exist | `recovery/SOURCE_FRUITS_CLASSIFICATION.md`, `recovery/REPO_ASSEMBLY_PLAN.md`, `recovery/ACCEPTANCE_STATUS.md` |
| Workspace builds | `cargo build` — Finished dev profile, 0 errors |
| Acceptance gates A1–A30 green | `cargo test` — 43 tests passed, 0 failed |
| Lints clean | `cargo clippy --workspace --all-targets` — 0 warnings |
| Install check runs a real session | `bash install/doctor.sh` — emit→sync→report on the demo pack |
| Release checksums are real | `release/checksums/SHA256SUMS` (sha256sum of built binaries) |
| Manhattan L-06 produced runtime evidence | `a26`/`a27`/`a28` — real subprocess stdout captured, scoped receipt closes only `manhattan.L-06`, projection health derived from Acts |
| Migrations recovered | `profiles/supabase/migrations/0001–0013` recovered from online-spine fruit |

## What was explicitly NOT closed (ghosts carried)

- `supabase-live-ingest` — no live project/credentials; adapter staged only.
- `pgmq/pg_cron queue runtime` (migration 0013) — extension-dependent.
- `sqlite-vs-json-outbox` — v0 ships a JSON-lines outbox as the provisional cache.
- `runtimes/manhattan` macOS install — no macOS host.
- Manhattan `G-01..G-08` — fleet field proofs (tunnel/MAC/WOL/Bluetooth/MDM/
  Ethernet interface).
- Rust-vs-TS MCP/CLI strategy; `apps/pitwall`, `apps/cockpit` adapters.

See `recovery/REPO_ASSEMBLY_PLAN.md` and `build-pack/06_GHOSTS_AND_OPEN_DECISIONS.md`.

## Debris policy applied

Vendored Foundation trees, `target/`, `__MACOSX/`, `.DS_Store`, `*.wasm` build
outputs, the `artifact` semantic crate, and file/SQLite-as-truth language were
**not** promoted (Operator §6/§13, Policy 07). Disposable junk was never called
"archive".
