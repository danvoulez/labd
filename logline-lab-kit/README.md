# LogLine Lab Kit

> LogLine Lab Kit is the instrument. Packs are the music. Profiles are the stage.
> Workers are the hands. Apps are the doors. Labs are the performances. Acts are
> the record. Receipts close only what was proven.

LogLine Lab Kit is an **installable Act machine**. It lets you initialize a Lab,
emit Acts, validate them, store them, project them, block them, attach evidence,
prepare scoped receipts, load packs/profiles, run workers, expose CLI/MCP
surfaces, and generate reports.

This is a **complete, right-sized v0** — not a minimal demo, and not a maximal
everything-machine.

## The Act

Everything consequential starts as a LogLine Act. The canonical Act has **exactly
nine slots** — there is no tenth slot:

```txt
who  did  this  when  confirmed_by  if_ok  if_doubt  if_not  status
```

Hashes, signatures, runtime data, selected branch, storage timestamps, and
envelopes live *around* the Act, never inside it. Files, JSON exports, SQLite
rows, UI state, reports, and provider responses are **not** semantic truth.

## Repository shape

```txt
build-pack/    the build control surface (authority for this project)
crates/        the Lab Kit core (Rust)
schemas/       the Act JSON schema and friends
profiles/      infrastructure choices (local-only, supabase, postgres, ...)
packs/         opinionated conventions (demo, santo-andre, manhattan)
packages/      optional app surfaces (mcp-server, model-middleware, ts-spine-client)
apps/          optional adapters (pitwall, cockpit)
runtimes/      optional workers (shell-worker, hermes, openclaw, manhattan)
deploy/        real-world machine/cloud configuration
recovery/      recovery scans + the Phase-0 assembly documents
install/        install / uninstall / doctor
release/       release artifacts + checksums
docs/          atlas, reference, archive context
```

## Core crates

| Crate | Responsibility |
|---|---|
| `logline-act` | Nine-slot Act, candidate mode, canonical JSON, hashing |
| `logline-lab-core` | Branch/verdict, evidence, blocked Acts, receipt candidates, manifests, app boundary |
| `logline-lab-local` | Provisional local outbox/cache (never truth) |
| `logline-lab-spine` | Generic spine trait + in-memory spine + sync |
| `logline-lab-supabase` | Supabase/Postgres profile adapter (content-addressed ingest) |
| `logline-lab-projectors` | Read models (recent, registry, blocked, health) |
| `logline-lab-clock` | Tick, due checks, reschedule |
| `logline-lab-hooks` | Hook runner |
| `logline-lab-dispatch` | Worker contract + shell worker (returns evidence, not closure) |
| `logline-lab-reports` | Report generator (a report is never a receipt) |
| `logline-lab-cli` | `labkit` command surface |
| `logline-lab-labd` | The generic Lab host |

## Quick start

```sh
# build + install the CLI and the recovery scanner
bash install/install.sh

# verify the toolchain and run a real example session
bash install/doctor.sh

# show the nine slots
labkit slots

# run a Lab session
labkit session \
  --lab examples/manifests/lab.json \
  --pack packs/demo/pack.json \
  --profile profiles/local-only/profile.json \
  --act examples/acts/first.act.json
```

## Acceptance

The build is gated by acceptance tests **A1–A30** (see
`build-pack/05_ACCEPTANCE_TESTS.md`). Current verified status lives in
`recovery/ACCEPTANCE_STATUS.md`. Run the whole suite:

```sh
cargo test
```

## Boundaries (do not cross)

- The project root is `logline-lab-kit`. Santo André, Manhattan, and Minilab are
  packs/contexts, **not** the product.
- Packs and profiles load as **data**; loading one never changes core.
- Workers return **evidence**, never closure. Receipts close only what evidence
  proves.
- Source material under recovery is raw material, not authority.

See `build-pack/OPERATOR_PROMPT_V2.md` for the full closed boundary.
