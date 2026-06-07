# Changelog

## v0.4.0-rc1 — generic v0 cut + hardening (Etapa 6.5): storage ontology + time (Etapas 0–6)

Disciplined cut to a clean generic v0. Scope frozen in `recovery/RELEASE_SCOPE.md`.

- **Storage is ontology.** Local files are capture/transport/cache, never
  protocol-grade truth. New grade ladder (`candidate-only` / `dev-ephemeral` /
  `publication`); admitted Acts require a declared Spine Profile. Honest grade +
  warning surfaced in Start/Doctor. See `docs/STORAGE.md`.
- **Supabase/Postgres out of the default path.** Excluded via `default-members`;
  optional `labd --features supabase-profile`. Selecting an external spine without
  it is refused as SOON. Santo André high-frequency storage is documented as a
  LATER deployment, not generic.
- **`labkit tick`** materializes time as Acts (clock_tick + due_disposition +
  reschedule); no due Act skipped. **`labkit storage`** shows the onboarding matrix.
- **No-pack end-to-end fixture** `release/examples/local-only-first-lab.sh` (now
  asserts JSON fields) and a real **release gate** `release/checks/run-checks.sh`
  (`set -euo pipefail`; fails if any step fails). Optional adapter evidence is
  separate (`release/checks/optional-supabase-profile.sh`).
- **Hardening (Etapa 6.5):** honest storage naming (`LocalActLog`, not "outbox is
  truth"); external spines are at most `staged` — no false `publication_grade`
  without a proven external-spine doctor check; schemas updated to match real
  contracts (profile/ruler-tick) + added start-view/tick-report/storage-matrix;
  version unified to `0.4.0-rc1`. **105 tests** pass; clippy `-D warnings` clean.
- Honest stubs to align the controlling-doc tree (experience/, benches/, install/,
  release/, build-pack/package.manifest.yaml).
- **Deferred to next cycle (review gate):** science rigor (Etapas 7–8:
  SCIENCE_STANDARDS + science-core projections).

## v0.3.0 — headless surfaces (the final 30%)

Close the Lab Kit as a protocol-grade experimental substrate, not a UI project.

- **Headless-first surfaces.** All nine surfaces are stable, versioned JSON
  read-models (`logline.view.*.v0`) with `labkit` commands (added `schedule`,
  `workbench`, `proof`). Contract: `docs/SURFACES.md`.
- **A Lab is a directory on disk** (`--store`): `outbox/evidence/ghosts/
  candidates.jsonl`, resumed identically across runs (`Lab::open`).
- **Same map for human and LLM.** MCP read surfaces (`McpServer::read_surface`,
  `grant_read`) return byte-identical JSON to the CLI; tested in `apps/mcp-server`.
  Reads need grants; writes become draft Acts (LLMs propose, never decide).
- Docs + runnable examples: `examples/human-and-llm/` (human-flow.sh, llm-flow.md);
  `docs/HUMAN_EXPERIENCE.md`/`SURFACES.md`.
- Contract + persistence tests added. **99 tests** pass; clippy clean.

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
