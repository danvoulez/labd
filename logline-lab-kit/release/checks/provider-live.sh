#!/usr/bin/env bash
# OPTIONAL provider live check. Runs ONLY if LOGLINE_PROVIDER_BASE_URL + LOGLINE_PROVIDER_MODEL
# are set (LOGLINE_PROVIDER_API_KEY optional — local/EMPTY-bearer endpoints need none).
# Otherwise prints "SKIPPED: no provider configured" and exits 0. The DEFAULT gate stays
# provider-free; this never depends on any private endpoint.
#
# Prints to STDOUT only; the release gate captures it to release/checks/provider-live.txt
# the same way it captures every other check (one convention; the script writes no file).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

if [ -z "${LOGLINE_PROVIDER_BASE_URL:-}" ] || [ -z "${LOGLINE_PROVIDER_MODEL:-}" ]; then
  echo "SKIPPED: no provider configured (set LOGLINE_PROVIDER_BASE_URL + LOGLINE_PROVIDER_MODEL)"
  exit 0
fi

S="$(mktemp -d)/lab"
LAB=(--lab "$ROOT/examples/manifests/lab.json" --profile "$ROOT/profiles/local-only/profile.json" --store "$S")
run(){ ( cd "$ROOT" && cargo run -q -p logline-lab-cli -- "$@" ); }

echo "# provider live check"
echo "# base_url=$LOGLINE_PROVIDER_BASE_URL model=$LOGLINE_PROVIDER_MODEL"

# Provider registration is a LogLine Act (no registry-of-record file).
run settings providers add livecheck --kind openai-compatible \
  --base-url "$LOGLINE_PROVIDER_BASE_URL" --model "$LOGLINE_PROVIDER_MODEL" \
  --api-key-env LOGLINE_PROVIDER_API_KEY --now 2026-01-01T00:00:00Z "${LAB[@]}"

run session suggest "${LAB[@]}" --provider livecheck --now 2026-01-01T00:00:01Z \
  --text "Reply with one short line." > "$S/suggest.json"

python3 - "$S/suggest.json" <<'PY'
import json,sys
d=json.load(open(sys.argv[1]))
assert d["candidate"]["status"]=="candidate", "provider output must be a candidate, not admitted"
assert d["candidate"]["confirmed_by"]=="", "model output must NOT satisfy confirmation"
assert d["candidate"]["did"]=="provider_suggested_candidate"
assert d["provenance"]["model"]["response_hash"], "provenance must carry a response hash"
print("OK: live provider produced a candidate with provenance; confirmed_by empty; not admitted")
PY

echo "PASS — provider live check (provider_call recorded as an Act; candidate captured, not admitted)"
