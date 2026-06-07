#!/usr/bin/env bash
# Pinned-canon drift check (C3 gate).
#
# Two layers, both required by default:
#   1. OFFLINE integrity — every vendored upstream file matches canon/.manifest.sha256.
#      Catches local tampering with no network. Always enforced.
#   2. ONLINE drift — every vendored file matches LogLine-Foundation/conformance@<SHA>.
#      Catches the vendored copy diverging from the pinned upstream. Requires `gh` +
#      network. Hard-fails on drift or if unreachable, UNLESS LLK_DRIFT_OFFLINE_ONLY=1
#      is set as an explicit, documented decision (e.g. air-gapped CI).
#
# "If drift exists, the gate fails unless there is an explicit update receipt/decision":
# the decision mechanism is a deliberate re-vendor (regenerate .manifest.sha256 + update
# PROVENANCE.md / CANON_ERRATA.md in a reviewed commit).
set -euo pipefail

REPO="LogLine-Foundation/conformance"
SHA="389a6b676af30bf5e344f9287ef51472b7f7a53f"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CANON="$HERE/canon"
MANIFEST="$CANON/.manifest.sha256"

echo "# drift check — pinned $REPO@$SHA"

# --- Layer 1: offline integrity against the committed manifest -----------------
[ -f "$MANIFEST" ] || { echo "FAIL: missing manifest $MANIFEST"; exit 1; }
offline_fail=0
# 1a. every manifest entry matches the file on disk
while read -r want rel; do
  [ -n "$rel" ] || continue
  if [ ! -f "$CANON/$rel" ]; then
    echo "OFFLINE DRIFT: vendored file missing: $rel"; offline_fail=$((offline_fail+1)); continue
  fi
  got=$(shasum -a 256 "$CANON/$rel" | cut -d' ' -f1)
  if [ "$got" != "$want" ]; then
    echo "OFFLINE DRIFT: $rel (disk=${got:0:12} manifest=${want:0:12})"; offline_fail=$((offline_fail+1))
  fi
done < "$MANIFEST"
# 1b. no untracked extra files (besides our PROVENANCE.md + the manifest)
manifest_count=$(grep -c . "$MANIFEST")
disk_count=$(find "$CANON" -type f ! -name 'PROVENANCE.md' ! -name '.manifest.sha256' | wc -l | tr -d ' ')
if [ "$manifest_count" != "$disk_count" ]; then
  echo "OFFLINE DRIFT: file count mismatch (manifest=$manifest_count disk=$disk_count)"; offline_fail=$((offline_fail+1))
fi
if [ "$offline_fail" -ne 0 ]; then
  echo "FAIL: $offline_fail offline integrity error(s) — vendored canon tampered locally"; exit 1
fi
echo "OK offline: $manifest_count vendored files match the manifest"

# --- Layer 2: online drift against the pinned upstream commit ------------------
if [ "${LLK_DRIFT_OFFLINE_ONLY:-0}" = "1" ]; then
  echo "WARN online drift check SKIPPED by explicit decision (LLK_DRIFT_OFFLINE_ONLY=1)"
  echo "OK (offline-only, by decision)"
  exit 0
fi
if ! command -v gh >/dev/null 2>&1; then
  echo "FAIL: gh not available; online drift check is required (set LLK_DRIFT_OFFLINE_ONLY=1 only with an explicit decision)"
  exit 1
fi
online_fail=0
checked=0
while read -r _want rel; do
  [ -n "$rel" ] || continue
  checked=$((checked+1))
  up=$(gh api "repos/$REPO/contents/$rel?ref=$SHA" --jq '.content' 2>/dev/null | base64 -d 2>/dev/null | shasum -a 256 | cut -d' ' -f1 || true)
  lo=$(shasum -a 256 "$CANON/$rel" | cut -d' ' -f1)
  if [ "$up" != "$lo" ]; then
    echo "ONLINE DRIFT: $rel (local=${lo:0:12} upstream=${up:0:12})"; online_fail=$((online_fail+1))
  fi
done < "$MANIFEST"
if [ "$online_fail" -ne 0 ]; then
  echo "FAIL: $online_fail file(s) drifted from $REPO@$SHA"; exit 1
fi
echo "OK online: $checked vendored files match $REPO@$SHA"
echo "OK: no drift"
