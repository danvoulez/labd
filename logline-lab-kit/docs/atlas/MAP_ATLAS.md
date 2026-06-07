> **Historical.** Superseded for generic v0 by `recovery/RELEASE_SCOPE.md` and `recovery/ACCEPTANCE_STATUS.md`. Kept for the record; figures like "43 tests", "A1–A30", "demo pack", and any "outbox is the record" / Supabase-as-default language reflect an earlier state.

# MAP / ATLAS — living build map

A working tracker, not prose. This file is the **ecosystem atlas**: diagram + index + tasklist.  
Narrative versions live in the docs; this file tells us **what exists, what must exist, and where each thing belongs**.

**Status marker rule:** Dan fills the status. I left most items as unset so the map does not pretend proof.

**Legend (left marker):**
```txt
⬜  status unset — Dan fills
✅  done / exists & verified
🟡  partial / exists but needs work
🔴  not started / needed
❓  unknown — verify before trusting
⛔  forbidden / removed / do not rebuild
```

**Per-section fields:** `where` · `kind` · `language / SDK` · `code?` · `assemble`.

**Hard semantic rules for this map:**
```txt
Act is the only canonical form.
Everything else is convention, envelope, projection, bundle, pack, profile, adapter, or surface.
Projection reads; it never governs.
Files are examples/exports/reports/debug unless explicitly scoped otherwise.
SQLite is cache/outbox unless a local-only profile explicitly says otherwise.
Supabase/Postgres is the v0/Santo André profile spine, not universal canon.
LLM output is draft/candidate, never authority/proof/receipt.
No receipt without evidence.
No claim of "working" without scoped receipt or named evidence.
```

---

## 0. Ecosystem diagram

```txt
                                      ┌──────────────────────────────────────┐
                                      │          LOG LINE FOUNDATION          │
                                      │   Act form · canonicalization · hash  │
                                      │   conformance · governance · anchor   │
                                      └──────────────────┬───────────────────┘
                                                         │ anyone may implement
                                                         ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────┐
│                                  LOG LINE LAB KIT                                           │
│  generic installable Lab host: labd · CLI · gate · worker boundary · spine adapters          │
│  envelopes · projections · bundles · convention table · packs · profiles · reports · benches │
└──────────────────────────────────────┬──────────────────────────────────────────────────────┘
                                       │ Kit + Pack + Profile + Anchor + Spine
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────┐
│                                          LABS                                               │
│                                                                                             │
│  Santo André Lab      = Lab Kit + Santo André Pack + Supabase profile + minilab.work face    │
│  Manhattan Lab        = Lab Kit + Manhattan Pack + physical fleet + own/federated spine       │
│  Personal Offline Lab = Lab Kit + Personal Offline Pack + local/private profile               │
│  Community/Company Lab= Lab Kit + their Pack + their Profile + their Anchor                   │
└──────────────────────────────────────┬──────────────────────────────────────────────────────┘
                                       │ signed Acts / inter-Lab contracts / anchor verification
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────┐
│                                      FEDERATION                                             │
│   no shared DB required · verify remote Acts by content hash + signature + published anchor  │
└─────────────────────────────────────────────────────────────────────────────────────────────┘

Runtime hands:
  CLI · MCP apps · labd API · pitwall bench · cockpit · Manhattan · Hermes/OpenClaw · models

Storage/projection:
  spine of Acts → projectors → registry/evidence/receipt/blocked/status/report views

Human surface:
  daily status · bench · phone · minilab.work · ChatGPT cockpit · terminal
```

---

## 1. LogLine Foundation — protocol / form / conformance

`where:` GitHub Foundation repos + vendored archive copies  
`kind:` protocol foundation  
`language / SDK:` Markdown spec · JSON Schema · Rust engine · conformance vectors  
`code?` partly exists, must be reconciled  
`assemble:` Foundation publishes the form; Labs implement and conform.

### 1.1 Canonical Act form

```txt
The canon has forms, not names.
At the limit, it has one form: the LogLine Act.
```

```txt
who
did
this
when
confirmed_by
if_ok
if_doubt
if_not
status
```

```txt
⬜  Act has exactly nine slots
⬜  no tenth slot
⬜  selected_branch is not a canon slot
⬜  hashes are not canon slots
⬜  runtime_id is not a canon slot
⬜  runtime envelope is not a canon slot
⬜  signing/checkpoint metadata is not a canon slot
⬜  status is a slot, but status vocabulary is profile/pack/conformance/practice-defined
⬜  if_ok / if_doubt / if_not are declared before acting
⬜  confirmed_by is the evidence/witness pivot
⬜  ugly candidates may be captured
⬜  strictness belongs at admission/execution/receipt/closure
```

### 1.2 Canonical behavior

```txt
⬜  log writes → code runs
⬜  no line, no action
⬜  claim with missing evidence routes to doubt / blocked
⬜  failure/contradiction routes to not
⬜  evidence support routes to ok
⬜  technical failure is not automatically semantic falsification
⬜  blocked Act preserves denied/stuck/non-event
⬜  blocked Act is visible, signed, and carried until resolved/retired
⬜  receipt closes only declared scope
⬜  projection gives domain meaning but is not semantic source
```

### 1.3 Identity, canonicalization, hash

```txt
⬜  same meaning → same bytes → same hash
⬜  canonical JSON profile decided and documented
⬜  JCS RFC 8785 profile tested
⬜  tuple_hash over the nine slots
⬜  content_hash over the full record / receipt profile as defined
⬜  envelope_hash only at transport/envelope layer when applicable
⬜  identity stable without central registry
⬜  content-addressed refs accepted throughout the ecosystem
⬜  conformance vectors include hash byte fixtures
```

### 1.4 Signature and anchor basis

```txt
⬜  Ed25519 signature profile
⬜  did:key identity profile
⬜  genesis Act profile
⬜  genesis Act is the only Act allowed confirmed_by:self
⬜  content_hash is signed
⬜  public anchor can be verified offline
⬜  anchor verification recipe documented
⬜  anchor rotation / revoke-compromised-anchor convention
⬜  Foundation anchor defined
⬜  Lab anchor profile defined
```

### 1.5 Receipt / closure profile

```txt
⬜  logline.receipt.v0 mold reconciled with latest Act-only canon
⬜  receipt candidate vs receipt distinction
⬜  receipt includes scope
⬜  receipt includes closed Acts
⬜  receipt includes evidence refs
⬜  receipt names what it does not close
⬜  receipt names blocked Acts remaining
⬜  conformance result can support receipt validity
⬜  receipt overclaim rejection path
⬜  receipt hash profile
```

### 1.6 Conformance corpus

```txt
⬜  valid examples
⬜  invalid examples
⬜  ambiguous / blocked examples
⬜  Act canonicalization vectors
⬜  receipt profile vectors
⬜  adapter declaration vectors
⬜  if_doubt / blocked Act vectors
⬜  signature / anchor vectors
⬜  projection staleness vectors
⬜  no tenth slot negative tests
⬜  no Foundation leak negative tests
⬜  no projection-as-truth negative tests
```

### 1.7 Governance / LIP process

```txt
⬜  governance repo exists
⬜  LIP process written
⬜  Draft / Proposed / Accepted / Superseded / Rejected states
⬜  errata process
⬜  conformance-first change rule
⬜  Foundation changes are rare
⬜  Lab Pack/Profile changes do not modify Foundation
⬜  assistant-generated ADRs are not authority
⬜  transcript material is historical/contextual, not canon
```

### 1.8 Foundation implementation artifacts

```txt
⬜  canon repo
⬜  conformance repo
⬜  engine-main Rust repo
⬜  constitutional/admission runtime repo
⬜  Ethics-is-Efficient / receipt service reference
⬜  What-Runs-Natively research archive
⬜  adapter protocol specs
⬜  Foundation docs frozen/published
⬜  vendored archive reconciled with public GitHub rev
```

---

## 2. Foundation engine and admission runtime

`where:` `_archive/logline-foundation-canonical-repos 2/engine-main/`, constitutional runtime archive, merge branches  
`kind:` executable judge / admission layer  
`language / SDK:` Rust  
`code?` exists in archive; needs live extraction and verification  
`assemble:` lift, build, test against conformance, expose as binary/library.

### 2.1 Rust `logline` engine

```txt
⬜  live repo location chosen
⬜  archive lifted to live working copy
⬜  cargo build --release
⬜  binary path published as LOGLINE_RUNTIME_BIN
⬜  slot crates: who
⬜  slot crates: did
⬜  slot crates: this
⬜  slot crates: when
⬜  slot crates: confirmed_by
⬜  slot crates: if_ok
⬜  slot crates: if_doubt
⬜  slot crates: if_not
⬜  slot crates: status
⬜  parse-logline command
⬜  check-canon command
⬜  run / walk command
⬜  evidence pivot returns ok/doubt/not
⬜  no-evidence claim routes to doubt
⬜  evidence-present claim routes to ok when witness matches
⬜  contradiction routes to not
⬜  unknown did verb routes to doubt, not guess
⬜  engine emits scoped receipt / verdict record
⬜  engine verified against current byte-vectors
```

### 2.2 Engine split decision

```txt
⬜  canonical core boundary: validate/hash/canonicalize/conformance
⬜  admission/runtime boundary: decide/walk/run
⬜  receipt preparation boundary
⬜  adapter protocol boundary
⬜  CLI boundary
⬜  library boundary for labd
⬜  N-API / WASM / shell-out decision for TypeScript callers
⬜  deterministic error taxonomy
```

### 2.3 Admission runtime / gate predecessor

```txt
⬜  admission checks slot completeness
⬜  admission checks source class / process class
⬜  admission checks operator/app identity
⬜  admission checks grants/policy
⬜  admission checks evidence state
⬜  admission checks secret safety
⬜  admission checks closure claim risk
⬜  admission can return allow / require / deny
⬜  admission can return blocked / needs evidence / needs human
⬜  admission does not execute
⬜  admission does not close receipt
```

### 2.4 Adapter protocol

```txt
⬜  adapter declaration schema
⬜  adapter capabilities
⬜  adapter input/output contract
⬜  adapter conformance vectors
⬜  MCP app adapter
⬜  CLI adapter
⬜  model middleware adapter
⬜  Manhattan adapter
⬜  worker/runtime adapter
⬜  external provider adapter
⬜  adapter cannot become de facto canon
```

---

## 3. LogLine Lab Kit — generic installable host

`where:` future `logline-lab-kit/` root  
`kind:` product / host / installable operational kit  
`language / SDK:` Rust + TypeScript where useful + SQL + Markdown + JSON Schema  
`code?` partial / archive / prototype  
`assemble:` core first, then outbox/spine, then projections, then clock, then surfaces.

### 3.1 Product identity

```txt
⬜  name: LogLine Lab Kit
⬜  product sentence: Study LogLine itself. Run it. Project it. Doubt it. Prove what can be proved. Preserve what cannot.
⬜  public promise documented
⬜  first session flow documented
⬜  installable product shape
⬜  Lab instance declaration flow
⬜  profile-capable
⬜  pack-capable
⬜  recoverable
⬜  upgrade-safe
⬜  audit-safe
```

### 3.2 The four working forms

```txt
⬜  Act — writes meaning
⬜  Envelope — carries run provenance
⬜  Projection — reads derived state
⬜  Bundle — transports conventions
⬜  Convention Table — names uses: receipt, grant, gate, worker, blocked Act, report, etc.
⬜  content hash + signature across forms
⬜  forms.md
⬜  conventions.md
⬜  forms conformance examples
⬜  no form modifies Act canon
```

### 3.3 Root repository structure

```txt
⬜  README.md
⬜  LICENSE
⬜  AUTHORSHIP.md
⬜  AI_NON_AUTHORITY.md
⬜  GHOSTS.md / BLOCKED.md naming decision
⬜  RECOVERY_RECEIPT.md
⬜  Cargo.toml
⬜  package.json only if actual Node/TS components exist
⬜  .env.example without secrets
⬜  install.sh or install docs
⬜  docs/
⬜  canon/
⬜  profiles/
⬜  packs/
⬜  manifests/
⬜  schemas/
⬜  supabase/migrations/
⬜  crates/
⬜  templates/
⬜  examples/
⬜  benches/
⬜  reports/templates/
⬜  hooks/default/
⬜  tests/
```

### 3.4 Rust crates

```txt
⬜  crates/logline-act
⬜  crates/logline-lab-core
⬜  crates/logline-lab-local
⬜  crates/logline-lab-supabase
⬜  crates/logline-lab-postgres
⬜  crates/logline-lab-projectors
⬜  crates/logline-lab-clock
⬜  crates/logline-lab-cli
⬜  crates/logline-lab-labd
⬜  crates/logline-lab-worker-boundary
⬜  crates/logline-lab-mcp
⬜  crates/logline-lab-pack
⬜  crates/logline-lab-conformance
⬜  crates/logline-lab-recovery
⬜  no logline-lab-artifacts crate
```

### 3.5 `logline-act` crate

```txt
⬜  Act model
⬜  nine-slot validator
⬜  canonicalization
⬜  hash computation
⬜  branch / route evaluation
⬜  status vocabulary as pack/profile input
⬜  ugly candidate support
⬜  JSON Schema generation
⬜  test vectors
⬜  no selected_branch in Act model
⬜  envelope metadata separate
```

### 3.6 `logline-lab-core`

```txt
⬜  Lab manifest model
⬜  practice/profile loading
⬜  pack loading
⬜  admission API
⬜  gate API
⬜  error taxonomy
⬜  convention table loader
⬜  registry abstractions
⬜  projection invalidation API
⬜  source-of-truth discipline helpers
```

### 3.7 `logline-lab-local`

```txt
⬜  SQLite provisional cache
⬜  local outbox
⬜  retry state
⬜  idempotency keys
⬜  no local truth language
⬜  no LocalLedger naming unless deprecated
⬜  no FileLabStore as official storage
⬜  local export/debug modes clearly labeled
⬜  offline queue behavior
⬜  infinite retry prevention
⬜  silent drop prevention
```

### 3.8 `logline-lab-supabase` / Postgres profile

```txt
⬜  Supabase client
⬜  direct Postgres path
⬜  PostgREST read path
⬜  ops.ingest_logline_act
⬜  idempotent insert/upsert
⬜  count 0→1 then repeat count stays 1 test
⬜  RLS safe read policies
⬜  schema reload / PostgREST reload path
⬜  connection pooling profile
⬜  profile-specific docs
```

### 3.9 Projectors

```txt
⬜  registry projector
⬜  runtime registry projector
⬜  evidence projector
⬜  receipt projector
⬜  blocked/ghost projector
⬜  audit projector
⬜  report projector
⬜  experiment projector
⬜  app/grant projector
⬜  health projector
⬜  capacity/band projector
⬜  staleness computation
⬜  rebuild from spine
⬜  projection invalidation on pack/profile/proof-rule change
⬜  projection is never truth guardrails
```

### 3.10 Lab clock

```txt
⬜  clock tick Act
⬜  due-check projection
⬜  NOW ruler semantics
⬜  scheduled Act resolution
⬜  no off-the-books cron semantics
⬜  cron may fire heartbeat but tick is Act
⬜  if_ok when confirmed
⬜  if_doubt when evidence missing / ask / retry / wait
⬜  if_not when contract cannot hold
⬜  reschedule as new Act
⬜  capacity band high edge
⬜  capacity band low edge
⬜  standing work queue
⬜  idle Lab is broken Lab signal
```

### 3.11 Hooks

```txt
⬜  hook registry
⬜  pre-admission hooks
⬜  post-admission hooks
⬜  pre-dispatch hooks
⬜  post-evidence hooks
⬜  projection hooks
⬜  report hooks
⬜  pack-provided hooks
⬜  hook secret policy
⬜  hook cannot bypass gate
⬜  hook cannot write semantic truth except through Acts
```

### 3.12 Worker boundary

```txt
⬜  worker receives allowed Acts / workorders, not wishes
⬜  worker cannot decide authority
⬜  worker returns evidence
⬜  worker does not close receipt
⬜  read-only mode
⬜  dry-run mode
⬜  apply mode
⬜  rollback / rectify path
⬜  stdout/stderr refs
⬜  file/output refs with hashes
⬜  secret redaction
⬜  execution report schema
⬜  worker failure → blocked Act, not fake success
```

### 3.13 CLI

```txt
⬜  `logline status`
⬜  `logline doctor`
⬜  `logline emit <file.json>`
⬜  `logline act list`
⬜  `logline act get`
⬜  `logline blocked`
⬜  `logline receipt prepare <scope>`
⬜  `logline add entity <kind> <name>`
⬜  `logline add experiment <name>`
⬜  `logline grant <app> <scope>`
⬜  `logline app connect <name>`
⬜  `logline app list`
⬜  `logline pack install`
⬜  `logline pack list`
⬜  `logline profile status`
⬜  `logline clock tick`
⬜  `logline projector run`
⬜  `logline report generate`
⬜  `logline conformance run`
⬜  plugin system decision: oclif vs Rust clap vs hybrid
```

### 3.14 `labd` API

```txt
⬜  `GET /health`
⬜  `GET /status`
⬜  `GET /acts`
⬜  `POST /acts`
⬜  `POST /gate/decide`
⬜  `GET /blocked`
⬜  `POST /blocked`
⬜  `GET /evidence`
⬜  `POST /evidence`
⬜  `GET /receipts`
⬜  `POST /receipts/prepare`
⬜  `GET /projections/*`
⬜  `POST /clock/tick`
⬜  `POST /dispatch/prepare`
⬜  `POST /worker/execute` only admitted work
⬜  MCP endpoint
⬜  no world-touch in initial gates.decide seam
```

### 3.15 Manifest and bundle system

```txt
⬜  lab manifest schema
⬜  pack manifest schema
⬜  profile manifest schema
⬜  app manifest schema
⬜  bundle manifest schema
⬜  convention table schema
⬜  runtime registry manifest
⬜  projector manifest
⬜  hook manifest
⬜  receipt profile manifest
⬜  evidence policy manifest
⬜  proof policy manifest
⬜  constraints / brakes manifest
```

### 3.16 Reports

```txt
⬜  Daily Lab State report
⬜  Expedition report
⬜  Conformance report
⬜  Audit report
⬜  Learning report
⬜  Maintenance report
⬜  Manhattan health report
⬜  Experiment report
⬜  Recovery receipt/report
⬜  report ≠ receipt guardrail
⬜  reports list claims, evidence states, blocked Acts, next probes
```

### 3.17 Study benches

```txt
⬜  acts bench
⬜  runtimes bench
⬜  engines bench
⬜  projections bench
⬜  blocked/ghosts bench
⬜  receipts bench
⬜  intervals bench
⬜  experiments bench
⬜  AI-to-Act translation bench
⬜  promotion-control bench
⬜  what-runs-natively bench
⬜  each bench README
⬜  each bench examples/
⬜  each bench invalid/
⬜  each bench fixtures/
⬜  each bench exercises/
```

### 3.18 Documentation surfaces

```txt
⬜  Start
⬜  Practice
⬜  Study
⬜  Build
⬜  Reference

⬜  docs/00-overview.md
⬜  docs/01-install.md
⬜  docs/02-spine-profiles.md
⬜  docs/03-cli.md
⬜  docs/04-labd.md
⬜  docs/05-manifest.md
⬜  docs/06-projectors.md
⬜  docs/07-blocked-acts.md
⬜  docs/08-evidence.md
⬜  docs/09-receipts.md
⬜  docs/10-clock.md
⬜  docs/11-hooks.md
⬜  docs/12-dispatch-and-worker.md
⬜  docs/13-study-benches.md
⬜  docs/14-frontends.md
```

---

## 4. Packs, profiles, Labs, overlays, frontends

### 4.1 Separation law

```txt
Canon loads → Pack interprets → Profile provides capability → Lab runs → Overlay adapts → Frontend exposes
```

```txt
⬜  Canon = small stable Foundation form
⬜  Pack = opinionated conventions, verbs, policies, projectors, golden configs
⬜  Profile = backend/storage/runtime/adapters/capabilities
⬜  Lab = declared instance by Act
⬜  Overlay = private deployment/infrastructure/policy/taste
⬜  Frontend = optional replaceable surface
```

### 4.2 Pack system

```txt
⬜  pack id = content hash of pack manifest/bundle
⬜  declare_pack Act
⬜  pack import
⬜  pack validation
⬜  pack signatures
⬜  pack convention table
⬜  pack verbs
⬜  pack projectors
⬜  pack hooks
⬜  pack policies
⬜  pack examples
⬜  pack benches
⬜  pack can be public or private
⬜  pack never modifies Foundation canon
```

### 4.3 Known / expected packs

```txt
⬜  Santo André Pack
⬜  Manhattan Pack
⬜  Personal Offline Pack
⬜  Classroom Lab Pack
⬜  Contract Bench Pack
⬜  Runtime Observatory Pack
⬜  Simulation Bench Pack
⬜  App Park Pack
⬜  Research Pack
⬜  Home/Casa Pack
⬜  Company/Huge Mini Company Pack
```

### 4.4 Profile system

```txt
⬜  supabase profile
⬜  postgres profile
⬜  local-only profile
⬜  filesystem-manual export/debug profile
⬜  hybrid private/local profile
⬜  profile capability matrix
⬜  profile secret policy
⬜  profile outbox policy
⬜  profile projection policy
⬜  profile install doctor
⬜  profile migration runner
```

### 4.5 Frontends / surfaces

```txt
⬜  CLI
⬜  labd API
⬜  minilab.work
⬜  cockpit UI
⬜  pitwall bench
⬜  ChatGPT cockpit
⬜  phone dashboard
⬜  MCP app consumers
⬜  direct SQL read/debug only
⬜  future web UI
⬜  no frontend is semantic source
```

---

## 5. Santo André Lab

`where:` LAB 8GB / minilab.work / Supabase profile  
`kind:` first reference Lab setup / recommended pack  
`language / SDK:` Acts + Rust + SQL + TypeScript + MCP  
`code?` partial / evolving  
`assemble:` install Kit, apply Santo André Pack, publish anchor, connect spine, project minilab.work.

### 5.1 Identity and anchor

```txt
⬜  Santo André Lab declared by Act
⬜  Santo André did:key generated
⬜  genesis Act
⬜  self-confirmed genesis only
⬜  content_hash root
⬜  signature over content_hash
⬜  public anchor triple published
⬜  anchor verification documented
⬜  anchor key stored outside repo
⬜  anchor rotation/revocation convention
```

### 5.2 Spine / storage profile

```txt
⬜  Supabase project chosen
⬜  Postgres spine profile declared
⬜  ops.logline_acts table
⬜  ingest function
⬜  idempotency test
⬜  direct Postgres read
⬜  REST/PostgREST read
⬜  RLS safe reads
⬜  evidence_records
⬜  receipt_index
⬜  registry projections
⬜  blocked Acts projection
⬜  health projections
```

### 5.3 Santo André Pack contents

```txt
⬜  pack manifest
⬜  convention table
⬜  `ping` dispatcher convention
⬜  provider call convention
⬜  model claim convention
⬜  lookup/evidence convention
⬜  proof rules
⬜  projectors
⬜  hooks
⬜  reports
⬜  study benches
⬜  daily status template
⬜  minilab.work projection policy
⬜  pack docs
```

### 5.4 minilab.work as digital face

```txt
⬜  minilab.work declares itself as frontend/overlay, not product root
⬜  reads projections
⬜  does not write semantic truth directly
⬜  shows Lab health
⬜  shows Register / what exists
⬜  shows blocked Acts
⬜  shows receipts
⬜  shows evidence refs
⬜  shows machine health from Manhattan federation
⬜  shows daily status
⬜  offers propose-Act flow
⬜  supports phone view
```

### 5.5 First Santo André proof path

```txt
⬜  labd installed on LAB 8GB
⬜  Lab manifest loaded
⬜  Santo André Pack loaded
⬜  anchor loaded
⬜  spine adapter connected
⬜  first Act emitted through labd
⬜  first projection rebuilt
⬜  first blocked Act named
⬜  first evidence record attached
⬜  first receipt candidate prepared
⬜  first scoped receipt closed
⬜  first Daily Lab State generated
```

---

## 6. Manhattan Lab / Manhattan Pack

`where:` LAB_8GB, LAB_512, LAB_256, Cloudflare tunnel, Ethernet inference bus  
`kind:` physical fleet Lab / peer Lab / Pack / worker domain  
`language / SDK:` Python/Rust/launchd/HTTP/MCP/Ansible-osquery option  
`code?` substantial design/code exists; fleet proof pending  
`assemble:` treat Manhattan as Lab/Pack/App, not hidden module inside Santo André.

### 6.1 Manhattan identity

```txt
⬜  Manhattan Lab declared by Act
⬜  Manhattan anchor generated
⬜  Manhattan own spine decision
⬜  co-located temporary spine decision if needed
⬜  Manhattan Pack declared
⬜  Manhattan Pack content hash
⬜  Manhattan <-> Santo André inter-Lab contract
⬜  Manhattan state exposed as projections, not source truth
```

### 6.2 Fleet roles

```txt
⬜  LAB_8GB = Capital / supervisor / clock / Hermes / MCP / spine sync
⬜  LAB_512 = engine room / inference / local model / OpenAI-compatible endpoint
⬜  LAB_256 = workbench / human bench / pitwall / cockpit / hands-on work
⬜  one installable package for all three
⬜  per-machine identity config
⬜  shared operating rules
```

### 6.3 Network topology

```txt
⬜  WiFi = internet route
⬜  Cloudflare tunnel = reachability
⬜  Ethernet 8GB↔512 = private inference bus
⬜  Ethernet /30 static profile
⬜  LAB_8GB IP 10.88.0.9
⬜  LAB_512 IP 10.88.0.10
⬜  no gateway on Ethernet
⬜  no DNS on Ethernet
⬜  cable does not carry internet
⬜  ping proof over Ethernet
⬜  latency proof
⬜  throughput/cable upgrade note
⬜  cable interface names verified per machine
```

### 6.4 Manhattan runtime processes

```txt
⬜  system daemon loop
⬜  user agent loop
⬜  observe → repair drift → observe again
⬜  daemon owns root/system checks
⬜  user agent owns user-session checks
⬜  launchd admitted services respected
⬜  no helper plist sprawl
⬜  no parallel authority
⬜  receipts emitted for meaningful action
⬜  secret-free receipts
⬜  HTTP endpoints: /health
⬜  HTTP endpoints: /status
⬜  HTTP endpoints: /items
⬜  HTTP endpoints: /metrics
⬜  HTTP endpoints: /receipts
⬜  HTTP endpoints: /dashboard
⬜  MCP wrapper for "check and repair my machines"
```

### 6.5 Manhattan L-items

#### Identity and boot

```txt
⬜  L-01 Identity
⬜  L-02 Auto-login
⬜  L-03 FileVault Off
⬜  L-04 Keychain usable
```

#### Network and reachability

```txt
⬜  L-05 Wi-Fi internet route
⬜  L-06 Ethernet peer /30
⬜  L-07 Peer Witness / WOL
⬜  L-08 SSH port 22
```

#### Power and local posture

```txt
⬜  L-09 pmset no-sleep policy
⬜  L-10 macOS firewall off
⬜  L-11 pf disabled
⬜  L-12 screensaver / lock disabled
```

#### Remote surfaces

```txt
⬜  L-13 TeamViewer system service
⬜  L-14 TeamViewer user session
⬜  L-15 cloudflared process
⬜  L-16 Cloudflare tunnel health
⬜  L-17 login survival
```

#### Daily cycle

```txt
⬜  L-18 daily rejuvenation pre-restart phases
⬜  L-19 daily restart
⬜  L-20 post-reboot survival
```

#### Governance / receipts / drift

```txt
⬜  L-21 filesystem gate
⬜  L-22 no parallel authority
⬜  L-23 receipts
⬜  L-24 drift detection
```

#### Apple / session controls

```txt
⬜  L-25 Remote Management
⬜  L-26 Bluetooth
⬜  L-27 AirDrop
⬜  L-28 Handoff / Universal Control
⬜  L-29 TCC / manual MDM
⬜  L-30 LAB Runtime process
```

### 6.6 User-agent owned items

```txt
⬜  L-04 Keychain usable
⬜  L-12 screensaver / lock disabled
⬜  L-14 TeamViewer user session
⬜  L-16 Cloudflare tunnel health if session-context needed
⬜  L-17 login survival
⬜  L-27 AirDrop
⬜  L-28 Handoff / Universal Control
⬜  com.project-manhattan.agent admitted
⬜  user agent bootstrap
⬜  user agent receipts
```

### 6.7 Manhattan ghosts / blocked Acts

```txt
⬜  G-01 Cloudflare tunnel health endpoint/parse pattern
⬜  G-02 peer MAC addresses for WOL
⬜  G-03 WOL retry/cooldown policy
⬜  G-04 Bluetooth/AirDrop CLI reliability
⬜  G-05 MDM enrollment status
⬜  G-06 headless-repair/headless-watchdog replacement timeline
⬜  G-07 Remote Management install/enable state
⬜  G-08 Ethernet interface name per LAB
```

### 6.8 Manhattan known bugs queued

```txt
⬜  BUG-01 gate probe classified candidate; should be admitted
⬜  BUG-02 mode hardcoded audit-only; read from manhattan.json
⬜  BUG-03 gate paths hardcoded; read gate_paths config
⬜  BUG-04 /Library/LaunchAgents not scanned
⬜  BUG-05 cloudflared plist/XML parse error
⬜  BUG-06 receipt retention missing
⬜  BUG-07 ttyskeepawake missing from pmset check/repair
```

### 6.9 Manhattan phases

```txt
⬜  PH-00 baseline read-only evidence
⬜  PH-01 no-mutation ghost/blocked resolution
⬜  PH-02 safe code patches BUG-01..BUG-07
⬜  PH-03 registry normalization
⬜  PH-04 probe-only audit expansion
⬜  PH-05 user agent design/admission
⬜  PH-06 system-side auto-repair
⬜  PH-07 user-side auto-repair
⬜  PH-08 daily rejuvenation scheduler
⬜  PH-09 peer witness / WOL
⬜  PH-10 final audit + reboot survival
⬜  PH-11 porting kit for LAB_512 / LAB_256
```

### 6.10 Manhattan safety decisions

```txt
⬜  LAB_8GB first
⬜  LAB_512/LAB_256 untouched until LAB_8GB Phase 10 receipts
⬜  identity mismatch halts mutation
⬜  no quarantine state
⬜  no helper plists
⬜  no scheduled shutdown/wake
⬜  no StartCalendarInterval restart
⬜  daily restart via daemon loop shutdown -r now
⬜  restart stagger: LAB_8GB 06:40
⬜  restart stagger: LAB_512 06:50
⬜  restart stagger: LAB_256 06:59
⬜  Remote Management, not Screen Sharing, as canonical surface
⬜  Cloudflare is not internet gateway
⬜  tunnel health ≠ cloudflared process health
⬜  TeamViewer admitted
⬜  SSH port 22 only
⬜  FileVault off
⬜  Universal Control / Handoff / AirDrop / Bluetooth admitted
```

### 6.11 Manhattan as MCP / Lab Connection app

```txt
⬜  register_entity Act for Manhattan app
⬜  issue_grant Act for Manhattan scope
⬜  MCP tool: check_machine_health
⬜  MCP tool: list_items
⬜  MCP tool: get_receipts
⬜  MCP tool: propose_repair
⬜  MCP tool: run_readonly_probe
⬜  MCP tool: request_protected_repair
⬜  app can only propose Acts
⬜  gate decides allow/require/deny
⬜  worker executes admitted work
⬜  evidence returns to spine
```

### 6.12 Manhattan Done-v1 target

```txt
⬜  labd on LAB_8GB takes L-06 contract
⬜  admits L-06 contract
⬜  runs real ping over Ethernet bus
⬜  captures evidence
⬜  closes scoped receipt signed under Santo André anchor
⬜  writes receipt/Act to spine
⬜  minilab.work projects machine healthy
```

---

## 7. Federation

`where:` between Labs  
`kind:` inter-Lab verification and contracts  
`language / SDK:` Acts + signatures + anchor verification  
`code?` designed, not implemented  
`assemble:` signed content-addressed Acts; no shared DB required.

### 7.1 Federation principles

```txt
⬜  no center
⬜  no shared database required
⬜  Labs publish anchors
⬜  Labs verify remote Acts offline
⬜  inter-Lab contract is Act-shaped
⬜  both anchors sign where needed
⬜  remote projection is local reading, not remote truth
⬜  minimal data disclosure
⬜  verifiability without surveillance
```

### 7.2 Santo André ↔ Manhattan federation

```txt
⬜  Santo André anchor published
⬜  Manhattan anchor published
⬜  inter_lab_contract Act
⬜  Manhattan emits health Act
⬜  Manhattan signs health Act
⬜  Santo André verifies signature/hash/anchor
⬜  Santo André projects remote health
⬜  minilab.work displays federated health
⬜  blocked path when remote proof missing
```

### 7.3 Future federations

```txt
⬜  Personal Offline Lab ↔ Santo André selective export
⬜  Company Lab ↔ Santo André contract
⬜  Community Lab ↔ Foundation conformance
⬜  external standard adapter federation
⬜  publication/external-recognition proof links
```

---

## 8. Personal Offline Lab

`where:` future private/offline deployment  
`kind:` full Lab with private pack/profile  
`language / SDK:` local profile + batch signing + adapters  
`code?` designed conceptually  
`assemble:` not second-category; same Foundation, different pack/profile.

### 8.1 Identity and purpose

```txt
⬜  Personal Offline Lab declared by Act
⬜  own anchor
⬜  local/private profile
⬜  offline-first or local-only mode
⬜  no external sharing by default
⬜  selective export contracts
⬜  longitudinal memory purpose
⬜  proof of legitimate human existence/events purpose
```

### 8.2 Ingestion adapters

```txt
⬜  email adapter
⬜  calendar adapter
⬜  location adapter
⬜  app usage adapter
⬜  file/system event adapter
⬜  photo/media reference adapter
⬜  manual observation adapter
⬜  health/fitness adapter only if explicitly chosen
⬜  finance adapter only if explicitly chosen
⬜  social/public presence adapter
⬜  not everything is recorded; valuable streams only
```

### 8.3 Capture and signing model

```txt
⬜  automatic candidate capture
⬜  local queue
⬜  batch grouping
⬜  human presence/passkey checkpoint
⬜  batch signature
⬜  private evidence refs
⬜  no built-in LLM UI required
⬜  universal endpoint for LLMs/tools/adapters to submit candidates
⬜  ugly candidate capture allowed
⬜  promotion/closure remains strict
```

### 8.4 Privacy and projection

```txt
⬜  local projections
⬜  redaction profiles
⬜  export profiles
⬜  evidence minimization
⬜  right to delete local projections
⬜  preserve Act spine according to profile
⬜  no accidental public receipts
⬜  no personal sensitive claim promotion without explicit act
```

---

## 9. App Park / application execution zone

`where:` downstream of Santo André/minilab; LAB-hosted apps  
`kind:` managed app execution zone  
`language / SDK:` apps vary; Acts + packs + profiles  
`code?` concept/package stage  
`assemble:` apps become members only when known, admitted, bounded, observable, recoverable, receiptable.

### 9.1 App membership

```txt
⬜  app registered by Act
⬜  app identity
⬜  app pack/profile declared
⬜  app capability boundary
⬜  app grant
⬜  app runtime environment
⬜  app health probe
⬜  app evidence path
⬜  app receipt path
⬜  app rollback / rectify path
⬜  app projection
⬜  app does not become member merely by running in container
```

### 9.2 Shared app infrastructure

```txt
⬜  app registry
⬜  app runtime registry
⬜  app capability matrix
⬜  app health projector
⬜  app evidence collector
⬜  app receipt profile
⬜  app deployment profile
⬜  app secret policy
⬜  app logs redacted before model
⬜  app incident blocked Acts
```

### 9.3 Candidate platform apps

```txt
⬜  Intelligence App
⬜  Constitutional Runtime as platform
⬜  HUGE Company as platform
⬜  Personal Engine as platform
⬜  Voulezvous.tv as possible media/app domain
⬜  App Park default demo app
⬜  contract bench app
⬜  runtime observatory app
```

### 9.4 Intelligence App

```txt
⬜  local macOS binary
⬜  Phase 2 app, not Tower/Foundation dependency
⬜  cron-beehive internal model
⬜  local mini-LLM calls
⬜  steward jobs
⬜  confidence score in every report
⬜  model-position reliability tracking
⬜  peer/meta-evaluations
⬜  calibration scoring
⬜  information emergence analysis
⬜  writes advisory candidates/reports only
⬜  no protected execution
⬜  no secret reading unless scoped
⬜  Tower/gate remains authority
```

---

## 10. Apps, CLI, MCP connection model

### 10.1 MCP apps

```txt
⬜  Lab MCP server
⬜  MCP SDK TypeScript app template
⬜  app connect flow
⬜  app register_entity Act
⬜  app issue_grant Act
⬜  app revoke_grant Act
⬜  app scopes: verbs/resources/time
⬜  app tool call → draft Act
⬜  app reads projections
⬜  app cannot write spine directly
⬜  app cannot close receipts
⬜  app cannot widen grant
⬜  app tool metadata treated as untrusted input
```

### 10.2 App candidates

```txt
⬜  web search app
⬜  code runner app
⬜  calendar app
⬜  Gmail/email app
⬜  Drive/file app
⬜  Supabase/Postgres app
⬜  Cloudflare app
⬜  Doppler app
⬜  Manhattan app
⬜  pitwall app
⬜  model/tool app
⬜  research capture app
⬜  standards adapter app
```

### 10.3 CLI growth model

```txt
⬜  small core
⬜  plugin commands
⬜  pack-provided commands
⬜  Manhattan CLI plugin
⬜  Santo André CLI plugin
⬜  Personal Offline CLI plugin
⬜  no command bypasses Act path
⬜  all commands emit/read Acts
⬜  friendly output maps engine to human words
```

### 10.4 Plain-language output

```txt
⬜  allowed and closed → Worked ✓
⬜  denied → Didn't ✗ + reason
⬜  needs human → Waiting on you ⏸
⬜  missing evidence → Missing proof ⚠
⬜  missing rights → Not allowed yet ⚠
⬜  stale heartbeat → Offline for X minutes
⬜  unknown allowed as honest state
```

---

## 11. Bench / cockpit / co-scientist

`where:` `~/pitwall`, `~/vibe-codin-.but.-real`, ChatGPT cockpit, future minilab.work surface  
`kind:` daily work surface and evidence-aware LLM companion  
`language / SDK:` TypeScript, Electron, Next.js, Vercel AI SDK, terminal PTY  
`code?` built/runnable per context; needs binding to Acts  
`assemble:` bind events/claims/probes to Act spine.

### 11.1 Pitwall bench

```txt
⬜  real PTY terminal
⬜  xterm.js surface
⬜  append-only event ledger
⬜  claim-vs-evidence tracking
⬜  watcher lanes
⬜  scout watcher
⬜  reviewer watcher
⬜  test monitor
⬜  risk monitor
⬜  secret redaction before model
⬜  model adapters: GPT
⬜  model adapters: Claude
⬜  model adapters: stub/local
⬜  PitwallEvent → Act adapter
⬜  Claim transition → route adapter
⬜  local SQLite event ledger → Lab outbox bridge
⬜  bench projections read from spine
```

### 11.2 Co-scientist rules

```txt
⬜  claim ≠ evidence
⬜  test ran ≠ test passed
⬜  file changed ≠ bug fixed
⬜  local pass ≠ production works
⬜  done with no test ≠ verified
⬜  only speak about screen/evidence actually present
⬜  name claims lacking evidence
⬜  propose probes
⬜  never decide authority
⬜  never close receipt
```

### 11.3 Experiment bench

```txt
⬜  intuition Act
⬜  thesis Act
⬜  invariant Act
⬜  probe Act
⬜  observation Act/evidence
⬜  scoped receipt
⬜  blocked Acts for what probe did not reach
⬜  "Never let beauty outrun evidence" rule
⬜  hats/roles: structure-finder
⬜  hats/roles: falsifier
⬜  hats/roles: probe-runner
⬜  hats/roles: missing-proof namer
```

### 11.4 Cockpit

```txt
⬜  daily notebook
⬜  control: machines / registry / policies
⬜  work: code bench / experiments
⬜  "+ New" tell/show/photograph flow
⬜  co-scientist drafts Act proposal
⬜  Refine / Register confirmation
⬜  preview rail with actual evidence
⬜  no raw machinery as primary UI
⬜  no UI as source truth
⬜  phone-compatible view
```

### 11.5 Vibe-codin / current UI

```txt
⬜  current repo located
⬜  Project-Manhattan-Official connected
⬜  control-plane-home screen
⬜  code-lab screen
⬜  Hermes WebUI exposure plan reviewed
⬜  plaintext secrets rotated
⬜  Cloudflare tunnel route checked
⬜  cockpit reads projections
⬜  cockpit proposes Acts
```

---

## 12. Models / AI boundary

### 12.1 Model principles

```txt
⬜  model output is claim/candidate
⬜  model output is not truth
⬜  model output is not evidence by itself
⬜  model output is not receipt
⬜  model output is not authority
⬜  model cannot widen scope
⬜  model cannot run protected action alone
⬜  model cannot write spine directly
⬜  model must preserve ghosts/blocked states
```

### 12.2 Vercel AI SDK middleware path

```txt
⬜  openai-compatible provider
⬜  local Mistral provider
⬜  Hermes-as-model provider
⬜  cloud GPT provider
⬜  cloud Claude provider where applicable
⬜  redaction middleware
⬜  Act framing middleware
⬜  ledger/capture middleware
⬜  tool parser middleware for weak local models
⬜  canon verdict middleware
⬜  receipt/verdict emission
⬜  streaming path
⬜  non-streaming path
```

### 12.3 Model tool use

```txt
⬜  tool call becomes draft Act
⬜  tool result can produce evidence ref
⬜  tool's own witness receipt consumed by canon engine
⬜  non-native function calling via parser middleware
⬜  do not apply tool parser to models with native tool calling
⬜  top-level tool call reporting normalized
⬜  tool metadata threat model
```

### 12.4 LLM offices / plain version

```txt
⬜  role/session mandate
⬜  scope
⬜  sources
⬜  may/may-not
⬜  evidence state language
⬜  handoff format
⬜  Morning Scribe / daily status role
⬜  Ghost Keeper / blocked Act namer role
⬜  Research Registrar role
⬜  Authority Clerk / gate packet role
⬜  LAB Reporter role
⬜  Home Ops Planner role
⬜  Receipt Reviewer role
⬜  Canon Librarian role
⬜  Routing Clerk role
⬜  Archive Keeper role
```

---

## 13. Runtime / worker layer

### 13.1 Hermes

```txt
⬜  installed binary verified
⬜  version/license verified
⬜  skills directory inventoried
⬜  cron directory mapped to Lab Clock
⬜  hooks directory mapped to Lab hooks
⬜  sessions/logs/memories mapped as observations, not truth
⬜  skill run → Act adapter
⬜  cron tick → Act adapter
⬜  Hermes WebUI exposure
⬜  Hermes secrets policy
⬜  Hermes not product identity
⬜  Hermes as worker, not authority
```

### 13.2 OpenClaw

```txt
⬜  installed binary verified
⬜  version/license verified
⬜  devices include LABs
⬜  agents/defaults reviewed
⬜  cron/tasks mapped
⬜  exec-approvals mapped to require path
⬜  credentials redaction
⬜  local provider updated to standard OpenAI-compatible path
⬜  OpenClaw task → Act adapter
⬜  choose primary runtime vs Hermes
```

### 13.3 MCP host-runtime

```txt
⬜  host-runtime on lab512 verified
⬜  port 8788 verified
⬜  Cloudflare route verified
⬜  MCP tools inventory
⬜  MCP calls → draft Acts
⬜  grants enforced
⬜  projections returned
⬜  no direct protected execution
```

### 13.4 Worker alternatives

```txt
⬜  shell runner worker
⬜  job queue worker
⬜  Ansible worker
⬜  osquery worker
⬜  local inference worker
⬜  media processing worker
⬜  build/test worker
⬜  browser/research worker
⬜  provider API worker
⬜  every worker returns evidence, not receipt
```

---

## 14. Spine / database / projections

### 14.1 Semantic spine

```txt
⬜  all official semantic writes are Acts
⬜  no direct CRUD as semantic truth
⬜  ops.logline_acts
⬜  content_hash primary identity
⬜  tuple_hash support
⬜  canonical bytes stored or reconstructable
⬜  envelope refs stored separately
⬜  idempotent ingest
⬜  schema versioning
⬜  profile-specific storage semantics
```

### 14.2 Supabase migrations

```txt
⬜  0001_ops_logline_acts.sql
⬜  0002_registry.sql
⬜  0003_audit_views.sql
⬜  0004_lab_observability.sql
⬜  0005_evidence.sql
⬜  0006_receipts.sql
⬜  0007_workorders.sql
⬜  0008_authz.sql
⬜  0009_functions_projectors.sql
⬜  0010_rls_safe_reads.sql
```

### 14.3 Projection schemas

```txt
⬜  registry.entities
⬜  registry.links
⬜  registry.runtimes
⬜  registry.projectors
⬜  registry.apps
⬜  registry.grants
⬜  evidence.records
⬜  receipt.index
⬜  blocked.current
⬜  audit.v_mobile_today
⬜  audit.acts_by_runtime
⬜  lab_observability.current_state
⬜  lab_observability.blocked_open
⬜  capacity.band_projection
⬜  experiment.current
```

### 14.4 Evidence records

```txt
⬜  evidence_id
⬜  kind
⬜  produced_by
⬜  observed_at
⬜  runtime
⬜  claim_ref
⬜  act_ref
⬜  payload_ref
⬜  hash
⬜  redaction_status
⬜  secret_redacted
⬜  scope
⬜  limits
⬜  evidence ≠ receipt enforcement
```

### 14.5 Receipt index

```txt
⬜  receipt_id / receipt_hash
⬜  scope
⬜  closed_acts
⬜  evidence_refs
⬜  blocked_remaining
⬜  limits
⬜  produced_by
⬜  produced_at
⬜  status
⬜  conformance state
⬜  receipt profile
⬜  overclaim rejection
```

### 14.6 Blocked Acts / ghost projection

```txt
⬜  root blocked Act ref
⬜  current status
⬜  missing proof/permission/runtime/source/signer/context/schema
⬜  why it blocks
⬜  owner
⬜  next Act
⬜  closure condition
⬜  carried_by refs
⬜  closed_by ref
⬜  retired/superseded refs
⬜  no direct semantic writes to projection
```

---

## 15. Authority / gate / security

### 15.1 Gate

```txt
⬜  gate reads policy JSON
⬜  gate reads Act
⬜  gate answers allow
⬜  gate answers require
⬜  gate answers deny
⬜  no warn-and-continue
⬜  missing info defaults deny / blocked
⬜  gate does not execute
⬜  gate does not close receipt
⬜  gate emits decision Act / projection
```

### 15.2 Policy JSON

```txt
⬜  command classes
⬜  resource scopes
⬜  app grants
⬜  human-required rules
⬜  second-signer rules
⬜  protected action rules
⬜  risk classes
⬜  evidence requirements
⬜  receipt paths
⬜  rollback/rectify requirements
⬜  denial reasons
```

### 15.3 Brakes

```txt
⬜  SAFE_MODE
⬜  DRY_RUN
⬜  ALLOW_DB_WRITE
⬜  ALLOW_PROVIDER_MUTATION
⬜  ALLOW_LAB_EXEC
⬜  ALLOW_PROTECTED_COMMANDS
⬜  released brake is still not permission
⬜  brake state projected
⬜  brake changes by Act
```

### 15.4 Secrets / Doppler / keychain

```txt
⬜  Doppler project/profile
⬜  secret names only in Acts
⬜  no secret values in Acts
⬜  no secret values in receipts
⬜  no secret values in logs
⬜  no secret values in reports
⬜  redaction before model
⬜  rotate leaked tokens
⬜  keychain storage for local keys
⬜  anchor private key policy
⬜  secrets manager is not authority
```

### 15.5 Protected commands

```txt
⬜  filesystem destructive
⬜  provider mutation
⬜  deploy/public route
⬜  database migration
⬜  remote machine repair
⬜  restart/shutdown
⬜  access/grant changes
⬜  receipt closure
⬜  public publication
⬜  external communication high-risk
⬜  requires Dan/second signer/passkey/window as policy says
```

---

## 16. Source-of-truth classes / current state

### 16.1 Truth classes

```txt
⬜  constitutional/protocol truth
⬜  current operational state
⬜  identity/registry truth
⬜  authority/gate truth
⬜  evidence truth
⬜  receipt truth
⬜  research truth
⬜  home/casa truth
⬜  LAB operations truth
⬜  config/secret truth
⬜  work/request truth
⬜  publication/external recognition truth
⬜  cockpit/conversation truth
⬜  archive/historical truth
```

### 16.2 Evidence states

```txt
⬜  unknown
⬜  planned
⬜  declared
⬜  observed
⬜  evidence_captured
⬜  verified
⬜  blocked / ghost
⬜  retired
⬜  superseded
```

### 16.3 Current State

```txt
⬜  CURRENT_STATE source initialized
⬜  Current State is not canon
⬜  Current State is not receipt
⬜  Current State can be updated from Dan observation
⬜  Current State can be updated from evidence
⬜  Current State can be updated from receipt
⬜  Current State can be updated from registry projection
⬜  Current State can be updated from explicit blocked Act
⬜  Current State must not be updated from model confidence alone
⬜  Current State must not be updated from memory alone
```

---

## 17. Daily operation / status report

### 17.1 Daily status / Morning State plain version

```txt
⬜  date
⬜  office/role
⬜  overall state
⬜  Casa
⬜  LAB
⬜  Research / Pesquisa
⬜  State / Evidence
⬜  Work / Requests
⬜  Authority / Needs Dan
⬜  Blocked Acts
⬜  Next Acts
⬜  Promote
```

### 17.2 Casa / home operations

```txt
⬜  home routine
⬜  cleaning plan
⬜  maintenance plan
⬜  supplies
⬜  service provider confirmations
⬜  execution confirmations
⬜  calendar integration
⬜  household blocked Acts
⬜  plan ≠ execution
⬜  reminder ≠ completion
```

### 17.3 LAB daily status

```txt
⬜  machines listed
⬜  alive/stale/offline
⬜  heartbeat
⬜  power state
⬜  sleep state
⬜  runtime status
⬜  worker readiness
⬜  physical attention needed
⬜  blocked Acts
⬜  next check
```

### 17.4 Work / requests

```txt
⬜  request captured
⬜  routed to correct Lab/pack/surface
⬜  owner
⬜  next Act
⬜  evidence needed
⬜  closure requirement
⬜  blocked reason
⬜  request ≠ execution
```

---

## 18. Research / science / benches

### 18.1 Santo André research method

```txt
⬜  intuition
⬜  thesis
⬜  invariant
⬜  probe
⬜  runtime observation
⬜  evidence
⬜  scoped claim
⬜  receipt
⬜  doctrine/system proposal
⬜  no beauty outruns evidence
```

### 18.2 Experiment registry

```txt
⬜  research track
⬜  question
⬜  thesis
⬜  invariant candidate
⬜  probe candidate
⬜  evidence state
⬜  blocked Acts
⬜  next research Act
⬜  promotion candidate
⬜  external recognition state
```

### 18.3 External recognition

```txt
⬜  DOI
⬜  conference acceptance
⬜  journal acceptance
⬜  standards body record
⬜  institutional correspondence
⬜  public citation
⬜  review record
⬜  collaboration agreement
⬜  published URL
⬜  paper draft ≠ accepted work
```

### 18.4 LLA-001 / reasoning conformance agent

```txt
⬜  traps.md
⬜  25 traps
⬜  rubric
⬜  9-field output format
⬜  answer key
⬜  baseline generic LLM run
⬜  specialist prompt v0.1
⬜  RAG
⬜  typed GraphRAG
⬜  deterministic verifiers
⬜  synthetic data
⬜  DPO
⬜  LoRA last
⬜  no_tenth_slot verifier
⬜  foundation_leak verifier
⬜  projection_truth verifier
```

---

## 19. Recovery / corpus hygiene

### 19.1 Lab Kit recovery

```txt
⬜  Archive(1).zip source retained
⬜  do not use prior assistant-generated zips as base
⬜  extract archive
⬜  classify corpus
⬜  keep product candidates
⬜  remove debris
⬜  merge lab-kit + _inbox/LAB-KIT-main
⬜  correct false authority
⬜  correct storage language
⬜  mark unimplemented features as blocked
⬜  verify product shape
⬜  verify first experience
⬜  generate recovery receipt
⬜  recovery is not release
```

### 19.2 Remove / demote

```txt
⛔  .claude/ in final product
⛔  target/ in final product
⛔  __MACOSX/
⛔  .DS_Store
⛔  sources/ as authority
⛔  KIT_MANIFEST.json as authority
⛔  docs/inventory/ as authority
⛔  docs/decisions/ as authority
⛔  assistant-generated repair files as product authority
⛔  generated ADRs as decisions
⛔  transcripts as product
⛔  artifact as semantic category
⛔  SQLite ledger/spine wording
⛔  file/JSON/JSONL spine wording
```

### 19.3 Implementation repair

```txt
⬜  remove logline-lab-artifacts crate
⬜  remove ArtifactSpool
⬜  remove artifact cleanup/TTL queues
⬜  remove FileLabStore as official store
⬜  correct LocalLedger naming
⬜  correct claim limit messages
⬜  disable receipt file output as official storage
⬜  mark ADR-0002 deprecated
⬜  scan for "primitive system"
⬜  scan for "artifact"
⬜  scan for "ledger" misuse
⬜  scan for "Supabase universal"
⬜  scan for "official pack"
⬜  scan for "runtime envelope as canon"
```

---

## 20. Existing implementations and live assets

### 20.1 Already-read / known paths

```txt
⬜  `_archive/logline-foundation-canonical-repos 2/engine-main/`
⬜  `_archive/logline-foundation-canonical-repos 2/conformance-main/`
⬜  `_archive/logline-foundation-canonical-repos 2/Ethics-is-Efficient-main/`
⬜  `_archive/manhattan 3/project-manhattan-v2/`
⬜  `_archive/manhattan 3/project-manhattan-v2/upstream-replacement/`
⬜  `~/pitwall`
⬜  `~/vibe-codin-.but.-real`
⬜  `~/logline-engine`
⬜  `~/logline-model-middleware`
⬜  `~/.hermes`
⬜  `~/.openclaw`
⬜  MCP host-runtime on lab512
⬜  Supabase project / Postgres spine
⬜  Doppler `logline-trust` project
⬜  Cloudflare tunnel `lab512`
```

### 20.2 Probes reportedly run

```txt
⬜  engine built clean
⬜  check-canon ok
⬜  sample.ok.logline no evidence → doubt
⬜  sample.ok.logline with evidence → ok
⬜  model middleware bypassed poisoned gateway
⬜  redaction stripped secret pre-model
⬜  model claim captured as evidence_captured, not verified
⬜  canon verdict routes raw model claim to doubt
⬜  governed tool lookup_capital produced witness receipt
⬜  witness receipt moved claim to ok
⬜  fresh Act written to live Supabase spine
⬜  Act read back via direct Postgres
⬜  Act read back via REST/PostgREST
```

### 20.3 Gaps from build doc

```txt
⬜  streaming model path
⬜  Hermes-as-model path
⬜  Base gateway route if still wanted
⬜  launchd outbox worker repair
⬜  Hermes vs OpenClaw runtime choice
⬜  external validation of versions/licenses
⬜  plaintext secret rotation
⬜  adapter spec close read
```

---

## 21. Generator / automation inputs

### 21.1 Catalogs

```txt
⬜  product.yaml
⬜  commands.yaml
⬜  schemas.yaml
⬜  migrations.yaml
⬜  projectors.yaml
⬜  benches.yaml
⬜  acceptance.yaml
```

### 21.2 Generator outputs

```txt
⬜  Rust crate skeletons
⬜  JSON schemas
⬜  SQL migrations
⬜  projector functions
⬜  bench examples
⬜  test templates
⬜  doc templates
⬜  CLI command skeletons
⬜  labd route skeletons
```

### 21.3 Generator constraints

```txt
⬜  no product invention
⬜  no canon invention
⬜  no architecture decisions
⬜  no fake receipts
⬜  no Dan authority replacement
⬜  no commands beyond catalog unless Dan approves
⬜  no schemas beyond catalog unless Dan approves
⬜  must pass acceptance criteria
```

---

## 22. Acceptance / proof ladder

### 22.1 Lab Kit product acceptance

```txt
⬜  installable
⬜  first session completes
⬜  emits Act to chosen spine profile
⬜  projections work
⬜  blocked Acts work
⬜  evidence records work
⬜  receipts prepare correctly
⬜  reports generate
⬜  benches available
⬜  canon conformance passes
⬜  schema conformance passes
⬜  Supabase profile spine works
⬜  local buffer/outbox works
⬜  no file spine exists
```

### 22.2 First end-to-end proof

```txt
⬜  hand-written Act accepted by engine
⬜  Act emitted via CLI/labd
⬜  local cache/outbox entry created
⬜  synced to spine
⬜  projection rebuilt
⬜  evidence attached
⬜  receipt candidate prepared
⬜  receipt closed narrowly
⬜  report generated
⬜  all hashes recomputable
```

### 22.3 Done v1 sentence

```txt
⬜  labd runs on LAB_8GB
⬜  L-06 contract exists
⬜  L-06 contract admitted
⬜  real ping over Ethernet runs
⬜  evidence captured
⬜  receipt signed under Santo André anchor
⬜  receipt written to spine
⬜  minilab.work shows healthy machine
```

---

## 23. Open decisions for Dan

### 23.1 Vocabulary

```txt
⬜  public term: blocked Act vs ghost
⬜  public term: gate vs Tower / Authority
⬜  public term: worker vs Hermes
⬜  keep Foundation / Lab Kit / Pack / Profile / Anchor in public docs?
⬜  how much of old constitutional language remains internal only?
```

### 23.2 Architecture

```txt
⬜  engine/admission split
⬜  labd primary implementation path
⬜  CLI framework: Rust clap vs oclif vs both
⬜  Hermes vs OpenClaw primary runtime
⬜  Supabase default in v0 vs file/local default for generic Kit
⬜  Manhattan own spine timing
⬜  Foundation anchor timing
⬜  Personal Offline storage profile
⬜  minilab.work role: Lab surface vs overlay vs product demo
```

### 23.3 Operational

```txt
⬜  Which secrets rotate now
⬜  Which LAB receives first install
⬜  Whether Manhattan PH-00/PH-01 precede Lab Kit binding
⬜  Whether L-06 proof is first public demo
⬜  Which docs are public first
⬜  Which archive pieces are frozen
⬜  Which components become repos
```

---

## 24. Forbidden / do-not-rebuild list

```txt
⛔  custom database
⛔  custom crypto
⛔  custom serializer when RFC 8785 suffices
⛔  custom plugin API instead of MCP where MCP fits
⛔  custom model gateway if OpenAI-compatible API + middleware suffices
⛔  dashboard as source of truth
⛔  UI direct CRUD as semantic write
⛔  file/JSON spine
⛔  SQLite as official truth in Supabase profile
⛔  assistant-generated ADR as architecture decision
⛔  transcript as product authority
⛔  report as receipt
⛔  evidence as closure
⛔  vendor success as proof
⛔  model confidence as state
⛔  hidden execution outside Act path
⛔  silent drop
⛔  infinite retry
⛔  fake success
⛔  package embedding secrets
⛔  runtime claim without runtime evidence
```

---

## 25. Minimal build order

### 25.1 Phase A — stabilize the form

```txt
⬜  reconcile latest Act public docs with Foundation conformance
⬜  verify nine-slot schema
⬜  verify canonicalization
⬜  verify hash profiles
⬜  verify receipt profile
⬜  publish/mark current Foundation rev
```

### 25.2 Phase B — live engine

```txt
⬜  lift engine
⬜  build engine
⬜  run conformance
⬜  expose LOGLINE_RUNTIME_BIN
⬜  document adapter contract
```

### 25.3 Phase C — Act spine circuit

```txt
⬜  local cache/outbox
⬜  ingest to Supabase/Postgres profile
⬜  idempotent insert
⬜  read back
⬜  first projection
⬜  first status report
```

### 25.4 Phase D — labd/gate

```txt
⬜  labd shell
⬜  /gate/decide
⬜  policy JSON
⬜  allow/require/deny
⬜  no world-touch seam
⬜  receipt/report of gate boundary
```

### 25.5 Phase E — worker boundary

```txt
⬜  dispatch packet
⬜  admitted workorder
⬜  read-only probe worker
⬜  evidence packet
⬜  receipt candidate
⬜  blocked path
```

### 25.6 Phase F — Manhattan L-06 demo

```txt
⬜  Manhattan Pack minimal
⬜  L-06 contract
⬜  LAB_8GB labd
⬜  Ethernet ping
⬜  evidence
⬜  receipt
⬜  spine write
⬜  projection
⬜  minilab.work display
```

### 25.7 Phase G — product experience

```txt
⬜  install docs
⬜  first session
⬜  CLI commands
⬜  docs 00..14
⬜  examples
⬜  benches
⬜  reports
⬜  recovery receipt
```

---

## 26. Final proof target

> ⬜ **`labd`, on LAB_8GB, takes the L-06 contract, admits it, runs the real `ping` over the Ethernet bus, captures evidence, closes a scoped receipt signed under Santo André's anchor, writes it to the spine, and `minilab.work` projects the machine healthy.**

When this flips to ✅, the protocol has been demonstrated end to end:

```txt
admitted
→ probed
→ evidenced
→ receipted
→ signed
→ stored
→ projected
```

```txt
The Act records.
The hash addresses.
The signature answers.
The projection reads.
The pack interprets.
The Foundation holds the form — and nothing more.

We trust and build with LogLine.
```
