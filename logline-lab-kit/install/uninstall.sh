#!/usr/bin/env bash
# LogLine Lab Kit — uninstaller. Removes the installed binaries. Never touches a
# Lab's Acts, evidence, or outbox (those are not owned by the installer).
set -euo pipefail
PREFIX="${PREFIX:-$HOME/.local}"
BIN="$PREFIX/bin"
for b in labkit recovery-scan; do
  if [ -f "$BIN/$b" ]; then
    rm -f "$BIN/$b"
    echo "removed $BIN/$b"
  fi
done
echo "uninstall complete"
