# MAP — the living build map

A working tracker, not prose. Edit it: flip the marker on the left as things move. Narrative version is `00_vision.md`; this is the checklist that guides us.

**Legend (left marker):**
```
✅  done / exists & verified
🟡  partial / discussed / exists but needs work
🔴  not started / needed
❓  unknown — verify before trusting
```

**Per-item fields:** `where` · `language / SDK` · `code?` · `how to assemble`.
Statuses below are my honest best guess as of 2026-06-06 — correct them freely.

---

## The map

```
LogLine Foundation        publishes, does not operate — the protocol, frozen
  Canon & Conformance · engine · constitutional-runtime
        │
LogLine Lab Kit           the generic installable host (opinion-free)
  labd · anchor/crypto · the four forms (spec) · spine adapter · translator · cockpit
        │  + a pack
a Lab (instance)
  Santo André Lab   (Kit + Santo André Pack · minilab.work)
  Manhattan Lab     (Kit + Manhattan Pack · own spine)
        ↔  Federation (inter-Lab contracts, anchor verification)

Tooling / Science:  traps.md + LLA-001 agent · the durable docs
```

---

## 1. LogLine Foundation — the protocol, frozen

### 1.1 Canon & Conformance
`where:` GitHub **LogLine-Foundation/canon** + **/conformance**; vendored in `_archive/logline-foundation-canonical-repos 2/{canon-main,conformance-main}` · `lang:` JSON / Markdown spec · `code?:` ✅ exists · `how:` it IS the source of truth — implementations conform to it, never the reverse.
```
✅  the mold logline.receipt.v0 (9 slots + hashes{tuple,content} + jcs-rfc8785)
✅  two hashes confirmed (envelope_hash removed from mold)
✅  tuple_hash profile = logline-length-prefixed-v0
✅  JCS RFC 8785 hash-profile documented
✅  conformance vectors (canon/receipt/adapter/if-doubt × valid/invalid/ambiguous)
✅  schemas (canon.v0, receipt.v0, adapter-declaration.v0)
🔴  governance / LIP process written down (how the canon changes, rarely)
🔴  Foundation's own anchor (the key that stamps the canon version)
❓  is the GitHub canon the same rev as the vendored one? (reconcile)
```

### 1.2 engine (`logline`)
`where:` `_archive/.../engine-main` and the merge impl `merging-codex-update-labd.../engine` · `lang:` **Rust** · `code?:` 🟡 exists, builds (per memory) · `how:` cargo lib + `logline` CLI; the canonical judge — certify / hash / receipt.
```
🟡  9 slot crates + status + receipt.v0 (exists; verify against current canon)
🟡  hashing aligned to canon: tuple=length-prefixed, content=JCS(minus hashes)  ← CHECK
🔴  re-verify: feed Foundation byte-vectors, confirm hashes match (be the 3rd impl, in-engine)
🔴  split decision: "canonical core" (hash/validate/conformance) vs "admission runtime" (run/walk)
```

### 1.3 constitutional-runtime
`where:` `_archive/.../constitutional-runtime-main` · `lang:` **Rust** · `code?:` 🟡 exists (lib) · `how:` admission / policy / IR / lowering / evidence; delegates receipts to engine.
```
🟡  admission + IR + lowering + evidence (exists; verify)
🟡  co-equal with engine, independently usable (confirm packaging)
🔴  conformance: does it pass the if-doubt vectors?
```

---

## 2. LogLine Lab Kit — the generic host

### 2.1 labd (the host binary)
`where:` `merging-codex-update-labd-to-comply-with-core-laws` · `lang:` **Rust** (CLI: clap) · `code?:` 🟡 exists (Codex merge, compiles green) · `how:` marries engine + constitutional-runtime, adds clock + CLI + empty registries + pack import; ships opinion-free.
```
🟡  merges the two engines, compiles
🔴  ALIGN to current canon (the flattened-form hashing is a bug — must hash JCS of readable Act)
🔴  empty registries (dispatcher / evidence-store / substrate / projector) + pack-import mechanism
🔴  the clock (tick → due-check Acts)
🔴  Gate 0 / Gate 1 (admission gates)
🔴  CLI: certify · admit · run · close · status
```

### 2.2 anchor / crypto
`where:` spec in `templates/anchor.md`; prototype built today (Python, ephemeral) · `lang:` Rust (target) / proto in Python · `code?:` 🟡 prototype only · `how:` Ed25519 → did:key, self-signed genesis Act, sign content_hash, trustless verify.
```
✅  design + a real, verified Santo André anchor (templates/examples/santo-andre.genesis.json)
✅  did:key (Ed25519 multicodec) + signature over content_hash + 3-way verify
🔴  real implementation inside labd/Kit (Rust): keygen, sign, verify
🔴  key persistence: secret key in Doppler / keychain (never in repo/receipt)
🔴  publish triple (did:key + genesis + signature) as the Lab's public anchor
🔴  anchor rotation / revoke-compromised-anchor convention
```

### 2.3 the four forms (the spec the Kit implements)
`where:` `templates/` · `lang:` Markdown spec · `code?:` ✅ written · `how:` Kit conventions on top of the Foundation Act.
```
✅  Act (canon) · Envelope · Projection · Bundle — forms.md
✅  Convention Table (where names live) — conventions.md
✅  hash + signature transversal; verifiable ≠ true — README.md
🟡  Projection declares derives_from + staleness (noted; needs enforcement in labd)
🟡  Authorization Act vs Runtime Envelope split (noted; needs a template)
🔴  Bundle manifest (pack.json) + app manifest schemas
```

### 2.4 spine (evidence/Act store — swappable adapter)
`where:` Supabase/Postgres (Santo André) · `lang:` SQL · `code?:` 🟡 partial · `how:` adapter; File default in the Kit, Supabase/SQLite optional. Acts as bytea (round-trip via content_hash).
```
🟡  one verified Act written to the spine (done this session)
🟡  Register projection with content-hash identity (exists)
🔴  schemas: ops.logline_acts · registry.* · evidence_records · receipt_index
🔴  File-store default adapter in the Kit (so it runs offline, no Supabase required)
🔴  projection rebuild + staleness on pack/proof-rule change
```

### 2.5 translator / control plane
`where:` ❓ (Hermes? verify) · `lang:` **TypeScript** + **Vercel AI SDK** · `code?:` 🟡 partial · `how:` NL → candidate Act; human confirms; never crosses execution boundary.
```
❓  locate existing code (Hermes / control plane)
🔴  NL → candidate Act (if_doubt by default; admission decides)
🔴  client of labd (over MCP / HTTP)
```

### 2.6 cockpit / bench
`where:` pitwall / vibe-codin (per memory) · `lang:` **TS** (Next / Electron) · `code?:` 🟡 exists (pitwall) · `how:` reads projections honestly (Worked / Didn't / Waiting / Missing proof), proposes Acts.
```
🟡  pitwall exists (verify what it shows today)
🔴  honest projection view bound to the spine
🔴  propose-Act flow (candidate → admission)
```

---

## 3. Santo André Lab  (Kit + Santo André Pack)

`where:` Capital = LAB 8GB; digital face **minilab.work** · `lang:` Acts (pack) + Rust dispatchers + TS face · `code?:` 🟡 partial · `how:` install Kit + apply Santo André Pack + its anchor + its spine.
```
✅  Santo André anchor (the real one, today)
🔴  Santo André Pack: dispatchers (ping, provider), spine config, substrates
🔴  declare_pack Act (pack_id = content_hash)
🔴  minilab.work site = projection of the Register / health
🔴  runtime_profile Acts (accepted_dids/types, proof_rules, projectors)
```

## 4. Manhattan Lab  (Kit + Manhattan Pack — peer Lab)

`where:` the 3 machines; own spine · `lang:` Acts (pack) + Rust · `code?:` 🟡 fleet scripts exist · `how:` derive from the Kit, own anchor + own spine, federate with Santo André by contract. NOT a module inside Santo André.
```
🟡  fleet / health scripts exist (per memory; verify)
🔴  Manhattan anchor (its own did:key)
🔴  Manhattan Pack: health checks L-01..L-30 as probe.health conventions
🔴  L-06 peer-link-check contract (the "done v1" target)
🔴  repair workorder + maintenance report conventions
🔴  own spine (can co-locate at first; separate gradually)
```

## 5. Federation  (inter-Lab)

`where:` between Santo André ↔ Manhattan · `lang:` Acts · `code?:` 🔴 designed only · `how:` signed, content-addressed Acts + an inter_lab_contract; verify by published anchor — no shared DB.
```
🔴  inter_lab_contract Act (signed by BOTH anchors)
🔴  remote verification (resolve → recompute hash → check signature → anchor match)
🔴  projection policy + data minimization (verifiability without surveillance)
🔴  first real federation demo: Manhattan emits health → Santo André consumes
```

---

## 6. Tooling / Science

### 6.1 traps + LLA-001 agent
`where:` `templates/traps.md` · `lang:` spec now; agent later · `code?:` 🟡 traps written, agent not built · `how:` traps + rubric = conformance for reasoning; build the agent on top.
```
✅  25 traps + rubric + 9-field output format (traps.md)
🔴  answer key + baseline run (generic LLM) → measure
🔴  specialist prompt v0.1 (full version is in ~/Downloads/Agent-LogLine.md)
🔴  build ladder: prompt+key → RAG → typed GraphRAG → verifiers → synthetic → DPO → LoRA last
🔴  deterministic verifiers (no_tenth_slot, foundation_leak, projection_truth)
🔴  MCP/Connection-App threat model (tool metadata = untrusted input)
```

### 6.2 the durable docs
`where:` `refazer-docs/` root + `templates/` · `lang:` Markdown · `code?:` ✅ written · `how:` keep washing; English; little and distilled.
```
✅  README · 00_vision · 01–08 · templates/ (README, forms, conventions, anchor, traps) · examples/
🟡  open tension: vocabulary rule vs protocol-era names (canon/Foundation/Lab Kit/Pack/anchor) — DAN'S CALL
🔴  fold identity law (content_hash, JCS, 2 hashes) into 01_the_act.md (gradually)
🔴  this MAP kept current
```

---

## Done — v1, the one sentence

> 🔴 **labd, on LAB 8GB, takes L-06, admits it, runs the real `ping` over Ethernet, closes a receipt signed under Santo André's anchor, writes it to the spine, and minilab.work shows the machine healthy.**

When that flips to ✅ — admitted · probed · receipted · signed · projected — the protocol is proven end to end.

```
The Act records. The hash addresses. The signature answers.
The projection reads. The pack interprets. The Foundation holds the form — and nothing more.
We trust and build with LogLine.
```
