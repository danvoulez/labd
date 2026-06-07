#!/usr/bin/env bash
# C3 — conformance HARD gate for the v0 generic kit. Fails non-zero if ANY required
# check fails (set -euo pipefail + pipefail-propagating tees). No known-red exceptions,
# no best-effort conformance, no local-only greenwash: the same gate runs in CI.
#
# Scope: the DEFAULT generic build only — no Supabase, no Manhattan, no Santo André, no
# required pack, no third-party storage. Optional adapter evidence is captured separately
# by `release/checks/optional-supabase-profile.sh` and is NOT part of this gate.
#
# Required checks (each writes an evidence file under release/checks/):
#   1 build        2 test (incl. canon suite + adversarial probe)   3 clippy -D
#   4 canon harness (Rust 21/21) + Node reference verifier (21 passed)
#   5 adversarial JCS probe (0/7 diverge)   6 drift check (pinned SHA)
#   7 doctor       8 no-pack fixture
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="$ROOT/release/checks"
cd "$ROOT"

stamp(){ echo "# $(date -u +%Y-%m-%dT%H:%M:%SZ) — $*"; }

{ stamp "toolchain"; rustc --version; cargo --version; node --version; } > "$OUT/toolchain.txt" 2>&1

stamp "clean build (default generic members)" | tee "$OUT/build.txt"
rm -rf target
cargo build 2>&1 | tee -a "$OUT/build.txt"

stamp "cargo test --workspace (incl. canon suite + adversarial probe + envelope)" | tee "$OUT/test.txt"
cargo test --workspace 2>&1 | tee -a "$OUT/test.txt"

stamp "clippy -D warnings (workspace + all targets)" | tee "$OUT/clippy.txt"
cargo clippy --workspace --all-targets -- -D warnings 2>&1 | tee -a "$OUT/clippy.txt"

stamp "canon conformance — Rust harness (REQUIRED: 21/21 conform · 0 divergence(s))" | tee "$OUT/conformance-baseline.txt"
cargo run -q -p logline-lab-conformance --bin conformance_baseline 2>&1 | tee -a "$OUT/conformance-baseline.txt"
{ echo ""; stamp "canon conformance — Node reference verifier (REQUIRED: 21 passed, 0 failed)"; } | tee -a "$OUT/conformance-baseline.txt"
node foundation/conformance/canon/tools/verify-receipt.mjs --suite foundation/conformance/canon/vectors 2>&1 | tee -a "$OUT/conformance-baseline.txt"

stamp "adversarial JCS probe (REQUIRED: 0/7 cases diverge)" | tee "$OUT/jcs-adversarial-probe.txt"
cargo run -q -p logline-lab-conformance --bin jcs_probe 2>&1 | tee -a "$OUT/jcs-adversarial-probe.txt"

stamp "canon drift check (pinned SHA; offline manifest + online upstream)" | tee "$OUT/drift-check.txt"
bash foundation/conformance/check-drift.sh 2>&1 | tee -a "$OUT/drift-check.txt"

stamp "install doctor" | tee "$OUT/doctor.txt"
bash install/doctor.sh 2>&1 | tee -a "$OUT/doctor.txt"

stamp "no-pack first-lab fixture (asserts JSON fields)" | tee "$OUT/fixture.txt"
STORE=/tmp/llk-checks-lab bash release/examples/local-only-first-lab.sh 2>&1 | tee -a "$OUT/fixture.txt"

echo "C3 GATE PASSED — all required conformance + build checks succeeded -> $OUT"
