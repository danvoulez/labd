# Changelog

## v0.2.0 — Lab formation kit (FINAL doc)

Pivoted to `build-pack/final-real-project-doc.md`: an installable **Lab formation
kit** for studying LogLine as a **protocol, not a company**. Evolution, not
teardown — see `recovery/MIGRATION_TO_FINAL_DOC.md`.

- **Basics first; packs complement.** `pack` is optional; the demo pack was
  removed — a Lab runs a first session on identity + profile alone.
- New crates: `logline-lab-ruler` (due/overdue/blocked/capacity), `logline-lab-
  conformance` (vectors/runner/export).
- New domain: study `bench` + `ghost` in core; `learning` report.
- Nine **experience surfaces** (Start/Today/Timeline/Write/Schedule/Workbench/
  Proof/Learn/Settings) as library functions + `labkit` subcommands.
- 10 schemas, 7 conventions, 11 docs, conformance vectors, restructured
  deployments/profiles/apps/runtimes.
- Acceptance **A01–A52** green; **94 tests** pass; clippy clean.

## v0.1.0 — first assembled v0

Recovered and assembled from `LOG_LINE_LAB_KIT_BUILD_PACKAGE_V2` per
`build-pack/OPERATOR_PROMPT_V2.md`.

### Phase 0 — boundary + classification + assembly
- `recovery/SOURCE_FRUITS_CLASSIFICATION.md`, `recovery/REPO_ASSEMBLY_PLAN.md`,
  `recovery/ACCEPTANCE_STATUS.md`.

### Phases 1–12 — Lab Kit core
- `logline-act`: nine-slot Act, candidate mode, canonical JSON + content/tuple
  hashing (recovered standalone from the Foundation `status` crate).
- `logline-lab-local`: provisional outbox/cache, idempotent emit.
- `logline-lab-spine`: generic spine trait, in-memory spine, sync.
- `logline-lab-supabase`: content-addressed Supabase/Postgres adapter +
  recovered migrations (`profiles/supabase/migrations/`).
- `logline-lab-projectors`: recent / registry / blocked / health read models.
- `logline-lab-core`: branch/verdict, evidence, blocked Acts, receipt
  candidates, manifests, app/MCP authority boundary.
- `logline-lab-clock`: tick / due / reschedule.
- `logline-lab-hooks`: hook runner.
- `logline-lab-dispatch`: worker contract + shell worker (evidence, not closure).
- `logline-lab-reports`: report generator (report ≠ receipt).
- `logline-lab-labd`: generic Lab host.
- `logline-lab-cli`: `labkit` command surface.
- `packages/mcp-server`: generic MCP/app boundary.
- `recovery/scanner`: false-authority + storage-as-truth scans.
- Packs: `demo`, `santo-andre`, `manhattan` (conventions only).
- Manhattan **L-06** serious proof: Act → gate → worker probe → evidence →
  scoped receipt → projection.
- `install/`: install / uninstall / doctor (real install check).

### Acceptance
- A1–A30 green (43 tests passing). See `recovery/ACCEPTANCE_STATUS.md`.

### Ghosts carried (not closed)
- `supabase-live-ingest`, `pgmq/pg_cron queue runtime`, `sqlite-vs-json-outbox`,
  `runtimes/manhattan` macOS install, Manhattan `G-01..G-08`, Rust-vs-TS MCP/CLI
  strategy, app adapters (pitwall/cockpit). See `recovery/REPO_ASSEMBLY_PLAN.md`
  and `build-pack/06_GHOSTS_AND_OPEN_DECISIONS.md`.
