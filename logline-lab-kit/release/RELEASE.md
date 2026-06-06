# Release — LogLine Lab Kit v0.1.0

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

```sh
cargo test          # A1-A30, 43 tests
cargo clippy --workspace --all-targets
bash install/doctor.sh
```
