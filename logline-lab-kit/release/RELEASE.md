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

```sh
cargo test          # default generic set (A01-A52 + 6.5 hardening)
cargo clippy --workspace --all-targets
bash install/doctor.sh
```
