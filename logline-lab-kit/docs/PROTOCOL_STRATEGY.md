# Protocol Strategy

Make protocol adoption easier than platform captivity (FINAL §16). The center is
**LogLine Act + conformance + independent Labs**, not a company.

Minimum viable independence: install locally, load reference, write/validate
Acts, run conformance, run one study bench, produce reports, export comparable
examples — all with **no central hosted service** (A17).

- Conformance: `crates/logline-lab-conformance` + `foundation/conformance/`.
- Exportable examples carry a `content_hash` so another Lab can compare behavior
  (A15/A16) — see `release/conformance-vectors/`.
- Labs are the distribution unit; the shared unit across all of them is the Act.
