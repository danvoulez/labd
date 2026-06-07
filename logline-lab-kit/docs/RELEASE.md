# Release

Artifacts: `labkit` (CLI) and `recovery-scan`. Build with `install/install.sh`.

- Real SHA-256 checksums: `release/checksums/SHA256SUMS`
  (regenerate via `release/checksums/generate.sh`).
- Conformance vectors for sharing: `release/conformance-vectors/`.
- Verify: `cargo test` (A01–A52), `cargo clippy --workspace --all-targets`,
  `bash install/doctor.sh`.

GHOST `first release distribution channel` (FINAL §23): Homebrew / GitHub
Actions scaffolding exists; the channel is an open decision.
