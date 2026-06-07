#!/usr/bin/env bash
# Generate SHA-256 checksums for release binaries. Real checksums only — never
# fake checksums (Operator §13 / Policy 07).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="$ROOT/release/checksums/SHA256SUMS"

( cd "$ROOT" && cargo build --release -p logline-lab-cli -p logline-lab-recovery-scanner )

: > "$OUT"
for b in labkit recovery-scan; do
  bin="$ROOT/target/release/$b"
  if [ -f "$bin" ]; then
    ( cd "$(dirname "$bin")" && sha256sum "$b" ) >> "$OUT"
  fi
done
echo "wrote $OUT:"
cat "$OUT"
