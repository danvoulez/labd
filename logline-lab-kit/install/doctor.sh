#!/usr/bin/env bash
# LogLine Lab Kit — install doctor. Verifies the toolchain and a buildable kit,
# then runs a real first session on THE BASICS (identity + profile, no pack) as
# an install check (Operator §13: do not claim install success without running an
# install check).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> rust toolchain"
command -v cargo >/dev/null || { echo "FAIL: cargo not found"; exit 1; }
cargo --version

echo "==> build check"
( cd "$ROOT" && cargo build -q -p logline-lab-cli )

echo "==> protocol conformance (offline)"
( cd "$ROOT" && cargo run -q -p logline-lab-cli --bin labkit -- conformance >/dev/null && echo "conformance: green" )

echo "==> first Lab session (basics: local-only profile, no pack)"
( cd "$ROOT" && cargo run -q -p logline-lab-cli --bin labkit -- session \
    --lab examples/manifests/lab.json \
    --profile profiles/local-only/profile.json \
    --act examples/acts/first.act.json )

echo "==> doctor OK"
