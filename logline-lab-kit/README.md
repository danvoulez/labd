# LogLine Lab Kit

> Build the Lab formation kit, not a company product. Keep Act as the only
> semantic unit. Make Labs the adoption unit. Make conformance the proof of
> protocol. Use time, proof, science, and experience to help Labs study LogLine
> seriously.

LogLine Lab Kit is **one project** (`logline-lab-kit`): an installable kit for
forming **Labs** that study, practice, test, prove, and transmit LogLine through
**LogLine Acts**. The public goal is to let people instantiate real Labs; the
strategic goal is to make LogLine a **protocol, not a company**.

The controlling definition is [`build-pack/final-real-project-doc.md`](build-pack/final-real-project-doc.md).

## Basics first; packs complement

A Lab forms and completes a first session with only an **identity + a profile** —
no pack required. Packs (`santo-andre`, `manhattan`, `course-starter`) are
**additive complements** that never change core.

```sh
labkit session \
  --lab examples/manifests/lab.json \
  --profile profiles/local-only/profile.json \
  --act examples/acts/first.act.json
```

## The Act

The canonical Act has **exactly nine slots** — there is no tenth:

```
who  did  this  when  confirmed_by  if_ok  if_doubt  if_not  status
```

Everything else is envelope, convention, projection, pack, profile, app,
runtime, or deployment. Files, JSON, SQLite rows, reports, and model output are
**not** semantic truth. Capture is generous (ugly candidates allowed); promotion
is strict (see [`docs/ACT_CANON.md`](docs/ACT_CANON.md)).

## Core crates

| Crate | Responsibility |
|---|---|
| `logline-act` | Nine-slot Act, candidate mode, canonical JSON, hashing |
| `logline-lab-core` | Branch/verdict, evidence, blocked Acts, receipt candidates, **study benches**, **ghosts**, manifests, app boundary |
| `logline-lab-local` | Provisional outbox/cache (never truth) |
| `logline-lab-spine` | Generic spine trait + in-memory spine + sync |
| `logline-lab-supabase` | Supabase/Postgres profile adapter (content-addressed ingest) |
| `logline-lab-projectors` | Read models (recent, registry, blocked, health) |
| `logline-lab-clock` | Tick, due, reschedule |
| `logline-lab-ruler` | Due/overdue/blocked, **capacity band**, next-study |
| `logline-lab-hooks` | Hook runner |
| `logline-lab-dispatch` | Worker contract + shell worker (evidence, not closure) |
| `logline-lab-reports` | Lab report + **learning report** (never a receipt) |
| `logline-lab-conformance` | Reference vectors, runner, exportable examples |
| `logline-lab-cli` | `labkit` command surface |
| `logline-lab-labd` | Lab host + the nine experience surfaces |

## Experience surfaces

Start · Today · Timeline · Write · Schedule · Workbench · Proof · Learn ·
Settings — library functions on the Lab host, wrapped by `labkit`. The surface is
flexible; the grammar is not (see [`docs/HUMAN_EXPERIENCE.md`](docs/HUMAN_EXPERIENCE.md)).

## Protocol & conformance

```sh
labkit conformance      # runs offline; exits non-zero if not green
```

Vectors live in `foundation/conformance/`; valid examples export with a
deterministic `content_hash` so another Lab can compare behavior — no central
service required ([`docs/PROTOCOL_STRATEGY.md`](docs/PROTOCOL_STRATEGY.md)).

## Quick start

```sh
bash install/install.sh    # build + install labkit and recovery-scan
bash install/doctor.sh     # toolchain + offline conformance + a real first session
labkit slots               # show the nine slots
cargo test                 # acceptance A01-A52
```

## Acceptance

Gated by **A01–A52** (`build-pack/final-real-project-doc.md` §20). Verified status
in [`recovery/ACCEPTANCE_STATUS.md`](recovery/ACCEPTANCE_STATUS.md).

## Boundaries (do not cross)

Root stays `logline-lab-kit`. Act is the only semantic unit. No native domain
objects, no artifact-as-truth, no file/SQLite truth, no central-service
requirement. Packs/profiles are additive; workers return evidence not closure;
receipts close only what evidence proves; ghosts are named, never silently
closed. See [`docs/`](docs/) and `build-pack/`.
