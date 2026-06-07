# Conformance Plan — make labd provably obey the LogLine canon

> **Status:** Proposed (awaiting go to execute)
> **Sequenced BEFORE the SDK adoption plan.** The conformance suite is the contract;
> proving conformance defines what every later SDK swap must achieve and proves when
> it has. Validation-first: establish the real gate and a truthful RED baseline, then
> each SDK adoption is "turn N red vectors green," witnessed by the foundation's own
> verifier.

---

## 0. Why this precedes the SDK plan

labd already ships a `logline-lab-conformance` crate, but it tests the **wrong contract
against the wrong oracle**:

- It ships **5 home-grown vectors** that only check **slot validity** (parse as 9
  non-empty slots). Its `Expectation` enum is `Valid | Invalid | Ambiguous`; `check()`
  only calls `Act::from_json_strict`.
- It computes `content_hash` with labd's **hand-rolled, non-conformant** canonicalizer
  (`crates/logline-act/src/canonical.rs`) and certifies the result as an "exportable
  example." A second Lab recomputing with a *conformant* JCS engine would get a
  different hash.
- It never tests the canon's actual contract: JCS byte-exactness, the three-layer
  hash discipline, the envelope, forbidden fields, or the receipt shape.

So today's green conformance report is a **ghost** — green that does not mean what it
claims. This plan replaces it with the authoritative contract before we change any
implementation.

## 1. Authoritative source + pin

- **Upstream:** `LogLine-Foundation/conformance`, branch `main`.
- **Pinned commit:** `389a6b676af30bf5e344f9287ef51472b7f7a53f` (pushed 2026-05-21).
- **Provenance verified:** Dan's local snapshot (`~/Downloads/logline-foundation-canonical-repos 3/conformance-main`)
  is **byte-identical** to upstream@389a6b6 (58/58 files; receipt schema, canon schema,
  jcs profile, verifier, envelope vector all SHA-256 MATCH). No drift.
- **Decision (locked):** vendor a copy into labd + record the SHA + drift check.
  Keeps labd's "runs fully offline, no central service" property. Re-vendoring is a
  deliberate, reviewed act. (This is the canon README's own prescribed model:
  "pin a SHA, download schemas + vectors, verify SHA, cache.")

## 2. What the canon binds (the contract we must obey)

- **Canonicalization = JCS / RFC 8785**, frozen (`json_canonicalization:"jcs-rfc8785"`).
  Hash = **sha256**. Worked vector: `{"a":[1,0.5],"b":2,"c":"hi"}` →
  `54927d21fad3946f67210fdacd35b9eecc06842b4a0c4fcf33e382a301f7246f`.
- **Three hash layers (LIP-0007):**
  - `tuple_hash    = sha256(jcs( 9 slots only ))`             → pure act
  - `content_hash  = sha256(jcs( receipt minus {id,hashes} ))`→ interpreted act; the `id`
  - `envelope_hash = sha256(jcs( {content, transport} ))`     → transported package;
    computed by the **sender at every boundary**, lives **ONLY** on the Envelope wrapper,
    **never** inside the receipt; **receiver verifies before accepting**.
- **Receipt shape (`logline.receipt.v0`):** 9 **string** slots + `id` (=content_hash) +
  `hashes{tuple_hash,content_hash,algorithm}` + `receipt_version` +
  `json_canonicalization` + free **AUX** (additionalProperties true; AUX ∈ content_hash,
  ∉ tuple_hash). `result`/`evidence`/`transport` **FORBIDDEN** at receipt top level.
- **Envelope:** `{content:<receipt>, transport:{sent_by,sent_to,sent_at,channel?}, envelope_hash}`.

## 3. Where labd diverges (the gaps this plan exposes, the SDK plan closes)

1. **Canonicalization is hand-rolled, not JCS.** Rust `keys.sort()` (UTF-8 byte order,
   canon wants UTF-16 code units) + `Number::to_string()` (canon wants ES6 shortest
   round-trip). Agree on ASCII + integers; diverge on unicode keys / floats / -0 /
   exponents. (The engine's own `status/receipt.rs` has the identical debt.)
2. **No envelope/transport layer at all.** No `envelope_hash`, `TransportMeta`, or
   `Envelope`. This is the MCP-transport gap: every boundary crossing must wrap +
   envelope-hash at the sender and verify at the receiver.
3. **Slots are `Value`, canon slots are strings; `Act` is bare-9 + `deny_unknown_fields`,
   receipt is 9 reserved + free AUX.** A labd Act is stricter than a canon receipt and
   is not a conformant receipt as-is. On export/transport it must **project** into a
   `logline.receipt.v0`.

## 4. Triage of labd's 5 existing vectors (decision: wrong→quarantine, good→refurbish)

| Vector | Verdict | Action |
|---|---|---|
| `v01_declare_lab` | Refurbish | Object `this` + no meta → valid labd Act, not a conformant receipt. Keep as **kit lab-formation example** (Act-shape tier); receipt twin deferred until divergence #3 is resolved. |
| `v02_observe` | Refurbish | Same as above. Keep as kit example. |
| `i01_missing_slot` | Refurbish/redundant | Correct invalid under both; canon already ships `missing-confirmed-by` + `missing-who`. Keep as kit example. |
| `i02_tenth_slot` | **WRONG vs canon → quarantine** | Asserts a 10th top-level field is invalid. Canon receipt is `additionalProperties:true` → a non-reserved extra field is **valid AUX**. labd conflated "no tenth **slot**" (true) with "no tenth **field**" (false). Move out of `foundation/conformance/`. |
| `a01_ugly_candidate` | Refurbish | Legitimately labd's candidate-generous layer (≠ canon parse-ambiguity). Keep as candidate example. |

**Two clearly-labeled tiers afterward:**
- `foundation/conformance/canon/` — vendored, pinned, **authoritative** canon suite.
- `foundation/conformance/kit-examples/` — labd's own lab-formation / candidate examples
  (NOT canon conformance; clearly labeled).
- `recovery/superseded-vectors/i02_tenth_slot.json` + README explaining the AUX conflict.

## 5. Execution phases

### C0 — Vendor the canon suite (additive, reversible)
- Copy the 58-file suite to `foundation/conformance/canon/` from the verified local snapshot.
- Add `foundation/conformance/canon/PROVENANCE.md`: upstream repo, pinned SHA, branch,
  pushedAt, "regenerated by re-vendoring, never hand-edited", and the verify command.
- Add a drift-check script: re-hash vendored files vs upstream@SHA, fail on mismatch.

### C1 — Rebuild `logline-lab-conformance` against the canon
- New vector model with **layered expectations**: structural (valid/invalid/ambiguous,
  current) **plus** hash-conformance (`tuple_hash`/`content_hash` byte-match),
  receipt-schema acceptance, envelope verification.
- Loader reads the vendored `canon/vectors/{receipt,canon}/{valid,invalid}/` instead of
  hard-wired `include_str!` paths. Refurbished kit examples move to `kit-examples/` and
  the A13–A17 acceptance tests are repointed there (kept green for the kit tier).
- Add the JCS worked-vector test (`54927d21…`) and envelope-vector tests (expected RED
  until the envelope primitive exists).

### C2 — Produce the truthful baseline report (DONE 2026-06-07)
- Harness run + external Node cross-check recorded in
  `release/checks/conformance-baseline.txt`. **No false "conformant" claim.**
- **ACTUAL baseline (revises the earlier prediction):**
  - labd **20/21**; foundation reference verifier **21/21**.
  - **Sole failure: `envelope.json`** — labd fails it only because `logline-act` has
    no envelope primitive (divergence #2). The reference verifies it.
  - labd reproduces the canon **byte-for-byte on all 20 receipt-layer vectors**,
    *including* `aux-unicode`, `aux-number-decimal`, `aux-key-order`. The earlier
    "unicode/number receipts will fail" prediction was **wrong**.
  - **Caveat — not a clean bill of health:** the current vectors do not exercise the
    JCS edge cases where labd's hand-roll provably diverges (object KEYS with astral
    >U+FFFF code points: UTF-8 scalar order vs JCS UTF-16 code-unit order; numbers
    needing ES6 exponential form). The 20/20 pass by *coincidence of input*, not by
    spec guarantee. Per the canon ("regardless of how close"), **P1 real JCS
    (`serde_jcs`) remains required**, alongside the envelope primitive.

### C3 — Lock the gate into CI (also P6-lite)
- Add the conformance harness to `release/checks/run-checks.sh`.
- Add a GitHub Actions workflow (`.github/workflows/`) running fmt + clippy -D + tests +
  the release gate + the conformance harness + the **external `verify-receipt.mjs --suite`
  cross-check** (Node, zero-dep) as an independent witness over the same vectors.
- Pin the upstream SHA; the drift check runs in CI.

## 6. Acceptance-test impact (must not silently break)

`crates/logline-lab-conformance/src/lib.rs` hard-wires the 5 vectors via `include_str!`
and A13–A17 assert that set is green. C1 repoints these to `kit-examples/` so the kit
tier stays green, while the new canon tier reports honestly (RED at baseline). Both
tiers are distinct and labeled; neither masks the other.

## 7. Handoff to the SDK plan

After C3, the SDK adoptions become measurable, witnessed deltas:
- **P1 JCS (`serde_jcs` or vetted equiv)** → adversarial probe (`jcs_adversarial_matches_canon`,
  currently `#[ignore]` expected-red) flips green. Does NOT change the receipt vector score
  (already 20/20) — it guarantees conformance on inputs the vectors don't cover.
- **Envelope primitive in `logline-act`** → `envelope.json` flips green. **DONE.**
- **Receipt projection (`Act` → `logline.receipt.v0`)** → resolves divergence #3 (not yet
  exercised: the harness feeds canon receipts directly to the hash fns, bypassing `Act`).

Each is "make N red vectors green," proven by both the Rust harness and the foundation's
own Node verifier. Buy-don't-build and the Act/projection invariant hold throughout.

### Progress (2026-06-07)
- ✅ **Adversarial JCS probe** added (`src/bin/jcs_probe.rs` +
  `release/checks/jcs-adversarial-probe.txt` + test). Pre-P1: 4/7 diverge.
- ✅ **Envelope primitive** in `logline-act` (`Envelope`, `TransportMeta`, `envelope_hash`,
  `verify_envelope_value`). Harness verifies envelopes for real → **labd 21/21 ==
  reference 21/21**. Transport wrapper only: hash never inside content, transport never
  becomes Act semantics.
- ✅ **P1 JCS replacement** — `canonical_json` delegates to `serde_json_canonicalizer`
  =0.3.2 (vetted by behavior, not name; via `ryu-js` for ECMAScript numbers). Adversarial
  probe now **7/7 CONFORMANT** and un-ignored in the normal suite. Old hand-roll demoted to
  a `#[cfg(test)]` divergence fixture (no production path). Canon prose erratum E-001
  recorded (`recovery/CANON_ERRATA.md`: exponent sign `1e+21`, reference wins over prose).
  Hash stability: canon-valid vectors unchanged; only non-conformant edge-case bytes changed
  (nothing admitted relied on them). 118 tests pass, clippy clean, doctor + fixture green.
- ✅ **C3 hard gate** — `release/checks/run-checks.sh` wires all required checks and
  fails non-zero on any: Rust canon harness `21/21`, Node reference verifier `21 passed`,
  drift check (offline `.manifest.sha256` + online pinned SHA), adversarial JCS probe
  `0/7`, `cargo test --workspace` (120 pass, 0 ignored), clippy `-D warnings`, doctor,
  no-pack fixture. Mirrored in CI by `.github/workflows/conformance-gate.yml` (same gate,
  not a weaker duplicate). The harness binaries now exit non-zero on divergence; the canon
  sweep is also a `cargo test` (`tests/canon_suite.rs`). Evidence regenerated in
  `release/checks/*.txt`. **No known-red, no best-effort, no local-only greenwash.**

**C3 complete (2026-06-07). Conformance is now a hard gate. Do not start Postgres / MCP
server / packaging / science-core until this is committed.**
