#!/usr/bin/env bash
# Capture clean build/test/lint/doctor/fixture evidence for the v0 generic kit.
# Writes outputs into release/checks/. This is the "I know it builds" proof.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="$ROOT/release/checks"
cd "$ROOT"

stamp(){ echo "# $(date -u +%Y-%m-%dT%H:%M:%SZ) — $*"; }

{
  stamp "rustc/cargo versions"; rustc --version; cargo --version
} > "$OUT/00_toolchain.txt" 2>&1

stamp "clean build" | tee "$OUT/01_build.txt"
rm -rf target
cargo build --workspace >> "$OUT/01_build.txt" 2>&1
echo "build exit: $?" | tee -a "$OUT/01_build.txt"

stamp "tests (default generic set)" | tee "$OUT/02_test.txt"
cargo test >> "$OUT/02_test.txt" 2>&1
echo "test exit: $?" | tee -a "$OUT/02_test.txt"

stamp "clippy -D warnings" | tee "$OUT/03_clippy.txt"
cargo clippy --workspace --all-targets -- -D warnings >> "$OUT/03_clippy.txt" 2>&1
echo "clippy exit: $?" | tee -a "$OUT/03_clippy.txt"

stamp "install doctor" | tee "$OUT/04_doctor.txt"
bash install/doctor.sh >> "$OUT/04_doctor.txt" 2>&1
echo "doctor exit: $?" | tee -a "$OUT/04_doctor.txt"

stamp "no-pack first-lab fixture" | tee "$OUT/05_first_lab.txt"
STORE=/tmp/llk-checks-lab bash release/examples/local-only-first-lab.sh >> "$OUT/05_first_lab.txt" 2>&1
echo "fixture exit: $?" | tee -a "$OUT/05_first_lab.txt"

echo "checks complete -> $OUT"
