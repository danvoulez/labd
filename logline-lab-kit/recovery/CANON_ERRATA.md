# Canon errata / interpretation notes

Durable record of places where the LogLine canon documentation and its authoritative
behavior (reference verifier + RFC 8785 + vectors) disagree, and how labd resolves them.
**Rule: reference bytes and vectors win over prose.**

---

## E-001 — JCS exponent sign: prose says "no `+`", reference emits `1e+21`

**Found:** 2026-06-07 (during P1 JCS adoption).

[`foundation/conformance/canon/hash-profiles/jcs-rfc8785.md`](../foundation/conformance/canon/hash-profiles/jcs-rfc8785.md)
rule 4 prose states:

> no `+` sign on exponent, lowercase `e`

But the authoritative behavior is the opposite on the sign:

- The reference verifier (`tools/verify-receipt.mjs`) formats numbers with ECMAScript
  `String(value)` / `Number.prototype.toString`, which yields **`1e+21`** — *with* the `+`.
- RFC 8785 §3 mandates exactly that ECMAScript `Number::toString` algorithm.
- The vendored vectors and both implementations (the old hand-roll and the adopted
  `serde_json_canonicalizer`) emit `1e+21`.

**Resolution:** labd follows the **reference bytes and the vectors**, i.e. `1e+21`. The
prose summary in `jcs-rfc8785.md` is treated as an editorial error for the exponent sign.
The lowercase-`e` part of the prose is correct.

**Implication for implementation choice:** a canonicalizer must be accepted by *behavior*
(canon vectors + adversarial probe + Node agreement), never by following the prose
summary. A crate that emitted `1e21` (no `+`) to match the prose would be **non-conformant**
to the actual vectors. `serde_json_canonicalizer` 0.3.2 (via `ryu-js`) emits `1e+21` and is
therefore correct.

Evidence: `release/checks/jcs-adversarial-probe.txt`,
`release/checks/conformance-baseline.txt`.
