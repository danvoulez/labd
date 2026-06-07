# Migration — Act machine v0 → Lab formation kit (FINAL doc)

The controlling definition changed to `build-pack/final-real-project-doc.md`. This
note records the pivot. It was an **evolution, not a teardown**: every load-bearing
decision carried over (root name, nine-slot Act, candidate-generous/promotion-
strict, packs/profiles additive, no artifact/file/SQLite truth, Manhattan L-06).

## What changed

### Reframed mission
From "installable Act machine" → "installable **Lab formation kit** for studying
LogLine as a **protocol, not a company**." Same root: `logline-lab-kit`.

### Basics first; packs complement (permanent design)
`LabManifest.pack` is now optional (`pack` + `packs`). A Lab forms and runs a first
session with only identity + profile. The **demo pack was removed**: the basics are
the real first-run path. Santo André / Manhattan / course-starter are complements.

### New crates
- `logline-lab-ruler` — due/overdue/blocked, capacity band, next-study (A23–A26).
- `logline-lab-conformance` — vectors, runner, exportable examples (A13–A17).

### New domain (in `logline-lab-core`)
- `bench` — study benches (A18–A21), `ghost` — named missing proof (A31).
- `logline-lab-reports::learning` — learning report (A21/A39).

### Experience surfaces
Start/Today/Timeline/Write/Schedule/Workbench/Proof/Learn/Settings as library
functions on the Lab host + `labkit` subcommands (A32–A40).

### Renames / restructure
- `deploy/` → `deployments/` (+ `local-dev`, `santo-andre`, `manhattan`).
- `schemas/logline-act.schema.json` → `schemas/act.schema.json`; added 9 schemas.
- profiles: `supabase`→`supabase-default`, `postgres`→`postgres-default`,
  + `personal-offline`; dropped `filesystem-manual`.
- `packages/mcp-server` → `apps/mcp-server`; added `apps/model-middleware`,
  `apps/chatgpt-bridge`; retired `pitwall`/`cockpit` adapters (ghosts).
- runtimes: `shell-worker`→`local-worker`, `*-adapter` → `hermes`/`openclaw`.
- Added `conventions/*.yaml`, `benches/*/bench.json`, `docs/*`,
  `foundation/conformance/*`.

### Acceptance
A1–A30 → **A01–A52**. Verified green (94 tests). See `ACCEPTANCE_STATUS.md`.

## Ghosts carried forward
`supabase-live-ingest`, `pgmq/pg_cron` (0013), `sqlite-vs-json-outbox`, evidence/
ghost persistence in the CLI, `runtimes/manhattan` macOS install, `G-01..G-08`,
Rust-vs-TS MCP/CLI, model-middleware/chatgpt-bridge, first release channel.
