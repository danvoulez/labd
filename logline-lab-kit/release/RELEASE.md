# Release — LogLine Lab Kit v0.4.0-rc1

## Artifacts

- `labkit` — the Lab Kit CLI
- `recovery-scan` — false-authority / storage-as-truth scanner

Build them with `bash install/install.sh`. Real SHA-256 checksums of the release
binaries are in `release/checksums/SHA256SUMS` (regenerate with
`release/checksums/generate.sh`).

## Distribution channel

GHOST `first release distribution channel` (build-pack 06). Scaffolding for
Homebrew (`release/homebrew/`) and GitHub Actions (`release/github-actions/`) is
present; the chosen channel is an open decision.

## Verify

The hard conformance + build gate (C3) — the same script CI runs:

```sh
bash release/checks/run-checks.sh
```

It fails non-zero unless ALL required checks pass, and writes evidence to
`release/checks/*.txt`:

| Check | Required outcome | Evidence |
|---|---|---|
| clean build | ok | `build.txt` |
| `cargo test --workspace` (incl. canon suite, adversarial probe, envelope) | all pass, 0 ignored | `test.txt` |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean | `clippy.txt` |
| Rust canon harness over vendored canon | `21/21 conform · 0 divergence(s)` | `conformance-baseline.txt` |
| Node reference verifier (`verify-receipt.mjs --suite`) | `21 passed, 0 failed` | `conformance-baseline.txt` |
| adversarial JCS probe | `0/7 cases diverge` | `jcs-adversarial-probe.txt` |
| drift check vs pinned `conformance@389a6b6` | no drift | `drift-check.txt` |
| `install/doctor.sh` | pass | `doctor.txt` |
| `release/examples/local-only-first-lab.sh` (no pack) | pass | `fixture.txt` |

Canon conformance means: vendored canon vectors + Rust harness + Node verifier +
drift check (see `recovery/CONFORMANCE_PLAN.md`). The kit-examples tier is **not**
canon conformance. CI runs the same gate via `.github/workflows/conformance-gate.yml`.
