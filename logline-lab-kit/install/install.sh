#!/usr/bin/env bash
# LogLine Lab Kit — installer.
#
# Builds the `labkit` CLI and the `recovery-scan` tool from source and installs
# them into PREFIX/bin. This is generic machinery — it hardcodes no Lab identity,
# pack, profile, or LAB machine (Operator §13).
set -euo pipefail

PREFIX="${PREFIX:-$HOME/.local}"
BIN="$PREFIX/bin"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> Building LogLine Lab Kit (release)"
( cd "$ROOT" && cargo build --release -p logline-lab-cli -p logline-lab-recovery-scanner )

echo "==> Installing to $BIN"
mkdir -p "$BIN"
install -m 0755 "$ROOT/target/release/labkit" "$BIN/labkit"
install -m 0755 "$ROOT/target/release/recovery-scan" "$BIN/recovery-scan"

echo "==> Installed:"
echo "    $BIN/labkit"
echo "    $BIN/recovery-scan"
echo
echo "Add $BIN to PATH if it is not already, then run: labkit slots"
