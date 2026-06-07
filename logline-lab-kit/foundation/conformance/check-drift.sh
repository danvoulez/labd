#!/usr/bin/env bash
# Drift check: verify the vendored canon conformance suite still matches the pinned
# upstream commit. Fails non-zero on any mismatch. Requires `gh` (authenticated) +
# network; intended for CI and deliberate re-vendor review, not the offline runtime path.
set -euo pipefail

REPO="LogLine-Foundation/conformance"
SHA="389a6b676af30bf5e344f9287ef51472b7f7a53f"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CANON="$HERE/canon"

echo "# drift check: $REPO@$SHA  vs  $CANON"

drift=0
checked=0
while IFS= read -r f; do
  rel="${f#"$CANON"/}"
  checked=$((checked + 1))
  up=$(gh api "repos/$REPO/contents/$rel?ref=$SHA" --jq '.content' 2>/dev/null | base64 -d 2>/dev/null | shasum -a 256 | cut -d' ' -f1 || true)
  lo=$(shasum -a 256 "$f" | cut -d' ' -f1)
  if [ "$up" != "$lo" ]; then
    echo "DRIFT: $rel (local=${lo:0:12} upstream=${up:0:12})"
    drift=$((drift + 1))
  fi
done < <(find "$CANON" -type f ! -name 'PROVENANCE.md')

if [ "$drift" -ne 0 ]; then
  echo "FAIL: $drift file(s) drifted from pinned upstream (checked $checked)"
  exit 1
fi
echo "OK: $checked vendored file(s) match upstream@$SHA"
