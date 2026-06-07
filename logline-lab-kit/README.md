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
# whole no-pack flow: open -> write -> admit -> conformance -> bench ->
# schedule -> tick -> proof -> learn -> export
bash release/examples/local-only-first-lab.sh
```

## Storage is ontology

A local file is capture/transport/cache — **never** the protocol-grade home of
admitted Acts. Storage has a grade ladder; admitted Acts need a declared Spine
Profile ([`docs/STORAGE.md`](docs/STORAGE.md)):

| Grade | v0 |
|---|---|
| `candidate-only` (capture only) | available |
| `dev-ephemeral` (local Act-log, dev-only) | available |
| `publication` (postgres/neon/supabase/byo) | **SOON** |

```sh
labkit storage     # the onboarding matrix
```

Supabase/Postgres are **optional and out of the default build** — enable with
`--features supabase-profile`. Santo André's high-frequency SQL+outbox+queue is a
LATER deployment, not generic.

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

**Canon conformance** means labd obeys the vendored, pinned LogLine canon
(`LogLine-Foundation/conformance@389a6b6`), proven four ways — the hard C3 gate
([`release/checks/run-checks.sh`](release/checks/run-checks.sh)):

1. **Rust canon harness** over the vendored receipt vectors → `21/21 conform`.
2. **Node reference verifier** cross-check (`foundation/conformance/canon/tools/verify-receipt.mjs --suite`) → `21 passed, 0 failed`.
3. **Drift check** against the pinned SHA (offline manifest + online upstream).
4. **Adversarial JCS probe** for RFC 8785 edge cases → `0/7 diverge`.

Canonicalization is **RFC 8785 / JCS** (`serde_json_canonicalizer`, vetted by
behavior). Three hash layers (LIP-0007): `tuple_hash` (9 slots), `content_hash`
(receipt + AUX, the `id`), `envelope_hash` (transport wrapper, sender-computed,
receiver-verified). See [`recovery/CONFORMANCE_PLAN.md`](recovery/CONFORMANCE_PLAN.md)
and [`recovery/CANON_ERRATA.md`](recovery/CANON_ERRATA.md).

Distinctions that matter:
- An **Act** is the internal semantic unit (exactly nine slots). A **receipt** is
  the protocol projection/package (`id`, `hashes`, `receipt_version`, AUX, profile).
- **AUX is not a tenth slot** — non-reserved fields are valid AUX (in `content_hash`,
  not `tuple_hash`). The **Envelope** is a transport wrapper, never truth, never a slot.
- `foundation/conformance/canon/` is the authority. `foundation/conformance/kit-examples/`
  are labd lab-formation examples, **not** canon conformance. The old `i02_tenth_slot`
  vector was wrong (it forbade AUX) and is quarantined **outside** the repo.

```sh
labkit conformance      # kit-tier runner (offline)
```
([`docs/PROTOCOL_STRATEGY.md`](docs/PROTOCOL_STRATEGY.md))

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
