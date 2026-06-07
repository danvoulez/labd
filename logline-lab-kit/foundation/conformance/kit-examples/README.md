# Kit examples — NOT canon conformance

These are labd's own **lab-formation / candidate examples**. They are a *kit* tier, not
the canon contract. The authoritative conformance suite is the vendored, pinned canon in
[`../canon/`](../canon/) (see its `PROVENANCE.md`).

Refurbished here from labd's original home-grown vectors (the wrong / non-conforming one,
`i02_tenth_slot`, was quarantined outside the repo — see the conformance plan).

| File | What it demonstrates | Tier |
|---|---|---|
| `v01_declare_lab.json` | A "declare lab" Act with a **structured** `this` slot — a valid labd Act, **not** a conformant `logline.receipt.v0` (canon slots are strings). Illustrates divergence #3. |
| `v02_observe.json` | An "observe" Act, structured `this`. Same divergence. |
| `i01_missing_slot.json` | Missing `confirmed_by` → correctly rejected by strict validation. (The canon suite covers this too via `missing-confirmed-by` / `missing-who`.) |
| `a01_ugly_candidate.json` | Candidate-generous capture (empty `who`, slots missing) — labd's candidate layer, **not** canon parse-ambiguity. |

These are checked by the *structural* (slot-validity) tier of the conformance harness and
the A13–A17 acceptance tests. They are deliberately **not** part of the canon
hash-conformance gate.
