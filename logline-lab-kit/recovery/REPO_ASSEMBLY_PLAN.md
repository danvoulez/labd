> **Historical.** Superseded for generic v0 by `recovery/RELEASE_SCOPE.md` and `recovery/ACCEPTANCE_STATUS.md`. Kept for the record; figures like "43 tests", "A1–A30", "demo pack", and any "outbox is the record" / Supabase-as-default language reflect an earlier state.

# REPO_ASSEMBLY_PLAN.md

Phase 0 deliverable (TASK 000). No feature implementation precedes this document.

## Target root

```txt
logline-lab-kit/
```

## Target tree (Operator §11)

```txt
logline-lab-kit/
  build-pack/        foundation/      crates/        schemas/
  conventions/       profiles/        packs/         manifests/
  projectors/        hooks/           benches/       reports/
  examples/          tests/           generator/     recovery/
  packages/          apps/            runtimes/      deploy/
  install/           release/         docs/
```

## Folders already present (from repo-skeleton + Phase 0 copy)

```txt
build-pack/            (copied from package: build control surface — authority)
docs/atlas/            (ATLAS_TABELA.md, MAP.md, MAP_ATLAS.md)
schemas/               (logline-act.schema.json)
examples/acts/         (first.act.json)
recovery/              (these three Phase 0 documents)
```

## Folders to create (this assembly)

```txt
crates/logline-act              crates/logline-lab-core      crates/logline-lab-local
crates/logline-lab-spine        crates/logline-lab-supabase  crates/logline-lab-projectors
crates/logline-lab-clock        crates/logline-lab-hooks     crates/logline-lab-dispatch
crates/logline-lab-reports      crates/logline-lab-cli       crates/logline-lab-labd
crates/logline-lab-acceptance   (A1-A30 gate harness)
profiles/{local-only,supabase,postgres,filesystem-manual}
packs/{demo,santo-andre,manhattan}
packages/{mcp-server,model-middleware,ts-spine-client}
apps/{pitwall-adapter,cockpit-adapter}
runtimes/{shell-worker,hermes-adapter,openclaw-adapter,manhattan}
deploy/{local,supabase,cloudflare,lab8gb,lab512,lab256}
install/   release/   conventions/   manifests/   hooks/   foundation/refs/
```

## Source fruit mapping

| Target | Source fruit(s) | Action | Notes |
|---|---|---|---|
| `crates/logline-act/` | `*/crates/logline-lab-core/src/act.rs`; `*/vendor/.../status/src/receipt.rs` | recover | Nine-slot Act + candidate mode + canonical-JSON/hash rewritten **standalone** (no heavy vendored dep). |
| `crates/logline-lab-core/` | `ready_1/.../{manifest,evidence,admission,ghost,visa}.rs`; merging-codex labd gates | recover | Evidence, blocked Act, receipt candidate, manifests, app/authz admission — fresh implementation. |
| `crates/logline-lab-local/` | `*/crates/logline-lab-outbox`, `logline-lab-local-sql` | recover (repaired) | Provisional **outbox/cache**, JSON-lines. **Not** SQLite-as-truth (A30). |
| `crates/logline-lab-spine/` | online-spine_3 spine path | recover | Generic spine trait + in-memory spine; idempotent ingest by content hash. |
| `crates/logline-lab-supabase/` | `online-spine_3/.../logline-lab-supabase`, migrations | recover | Profile adapter wired to recovered SQL; **live** ingest ghosted. |
| `crates/logline-lab-projectors/` | SQL views `0002-0009` | recover | Read models only (recent/registry/blocked/evidence/receipts/health/staleness). |
| `crates/logline-lab-clock/` | `ready_1/.../clock.rs`, `interval.rs` | recover | tick/due/reschedule. |
| `crates/logline-lab-hooks/` | hooks/default in skeleton | recover | Hook runner. |
| `crates/logline-lab-dispatch/` | `ready_1/.../workorder.rs`; shell-worker | recover | Worker contract: dispatch packet, workorder, execution report, allow-gate, dry-run + real. |
| `crates/logline-lab-reports/` | reports/templates | recover | Report generator; report ≠ receipt (A13). |
| `crates/logline-lab-cli/` | `*/crates/logline-lab-cli` | recover | Generic command surface + doctor. |
| `crates/logline-lab-labd/` | `*/crates/logline-lab-labd`; merging-codex labd | recover | Lab host: load manifest/pack/profile, expose API. |
| `profiles/supabase/migrations/` | `online-spine_3/ops/supabase/migrations/*.sql` | recover | Portable DDL recovered verbatim where stock-Postgres-safe; extension-dependent files ghosted. |
| `packs/manhattan/` | `source-fruits/manhattan/*` | recover | **Conventions only** — L-06 checklist. Manhattan is a pack, not the root. |
| `packs/santo-andre/` | reduced docs references | recover | Reference pack manifest. |
| `packs/demo/` | first-session example | recover | Minimal demo pack for first session (A23). |
| `runtimes/manhattan/` | Manhattan v2 plan | ghosted | macOS impl not built in this environment (see ghosts). |

## Not copied

| Path | Reason |
|---|---|
| `source-fruits/extracted/*/vendor/logline-foundation/**` | Vendored Foundation is reference, not Lab Kit runtime truth; standalone canonicalization recovered instead. |
| `**/target/`, `**/*.lock`, `**/__MACOSX/`, `**/.DS_Store`, `*.wasm` | Build outputs / OS cruft = debris (Policy 07). |
| `crates/logline-lab-artifacts/**` | "artifact as semantic category" — rejected (Operator §6). |
| `source-fruits/minilab-context/**` (as authority) | Minilab is not the product root; kept raw as archive context only. |
| `*/crates/logline-lab-hermes` (as core) | Hermes is a runtime adapter, not core; runtime boundary stubbed/ghosted. |

## Ghosted

| Ghost | Why it remains open | Next proof |
|---|---|---|
| `supabase-live-ingest` | No live Supabase project/credentials in this environment | Run `init/doctor --profile supabase` against a real project; capture ingest receipt. |
| `pgmq/pg_cron queue runtime` (0013) | Extension-dependent, not in stock Postgres | Apply on real Supabase; attach worker. |
| `sqlite-vs-json-outbox` | Architecture names SQLite cache; v0 ships JSON-lines outbox to stay dependency-light | Decide final local store; both are "cache, not truth". |
| `runtimes/manhattan` macOS install | No macOS host here | Install on a LAB machine; capture runtime evidence. |
| `G-01..G-08` (Manhattan ghosts) | Carried from build-pack 06 (tunnel/MAC/WOL/Bluetooth/MDM/etc.) | Per-ghost field proof on real fleet. |
| `mcp-server / model-middleware as TS` | Operator §10 lists TS packages; Rust-vs-TS CLI/MCP strategy open (ghost 06) | v0 implements the **boundary logic** in Rust core + Rust `packages/mcp-server`; TS surface deferred. |
| `apps/pitwall`, `apps/cockpit` | Optional app adapters; depend on external UIs | Wire to real apps; read projections only. |

## Build order followed

Phase 0 (this doc + classification + acceptance status) → Phase 1 Act core → Phase 2 local
→ Phase 3 spine → Phase 4 projections → Phase 5 labd/CLI → Phase 6 evidence/blocked/receipts
→ Phase 7 clock/hooks → Phase 8 worker → Phase 9 MCP/model boundary → Phase 10 packs
→ Phase 11 Manhattan L-06 proof → Phase 12 install/release. No phase skipped.
