# Conformance

Conformance makes protocol compatibility visible (A13–A17).

- Vectors: `foundation/conformance/{valid,invalid,ambiguous}/*.json`.
- Runner: `crates/logline-lab-conformance` (`run`, `builtin_vectors`,
  `export_examples`).
- CLI: `labkit conformance` (exits non-zero if not green).
- Expectations: `valid` admits as nine slots; `invalid` is rejected; `ambiguous`
  is preservable as a candidate but not strictly valid.
- Exported examples carry a deterministic `content_hash`; another Lab recomputes
  it to confirm compatibility. Runs fully offline.
