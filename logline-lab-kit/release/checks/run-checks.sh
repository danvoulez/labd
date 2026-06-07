#!/usr/bin/env bash
# Release GATE for the v0 generic kit. Fails non-zero if ANY step fails
# (set -euo pipefail + pipefail-propagating tees). Scope: the DEFAULT generic
# build only — no Supabase, no Manhattan, no Santo André, no required pack, no
# third-party storage. Optional adapter evidence is captured separately by
# `release/checks/optional-supabase-profile.sh` and is NOT part of this gate.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="$ROOT/release/checks"
cd "$ROOT"

stamp(){ echo "# $(date -u +%Y-%m-%dT%H:%M:%SZ) — $*"; }

{ stamp "toolchain"; rustc --version; cargo --version; } > "$OUT/00_toolchain.txt" 2>&1

stamp "clean build (default generic members)" | tee "$OUT/01_build.txt"
rm -rf target
cargo build 2>&1 | tee -a "$OUT/01_build.txt"

stamp "default generic tests (cargo test == default-members, no Supabase)" | tee "$OUT/02_test.txt"
cargo test 2>&1 | tee -a "$OUT/02_test.txt"

stamp "clippy -D warnings (default members + all targets)" | tee "$OUT/03_clippy.txt"
cargo clippy --all-targets -- -D warnings 2>&1 | tee -a "$OUT/03_clippy.txt"

stamp "install doctor" | tee "$OUT/04_doctor.txt"
bash install/doctor.sh 2>&1 | tee -a "$OUT/04_doctor.txt"

stamp "no-pack first-lab fixture (asserts JSON fields)" | tee "$OUT/05_first_lab.txt"
STORE=/tmp/llk-checks-lab bash release/examples/local-only-first-lab.sh 2>&1 | tee -a "$OUT/05_first_lab.txt"

echo "GATE PASSED — all generic v0 checks succeeded -> $OUT"
