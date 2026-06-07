#!/usr/bin/env bash
# v0 generic end-to-end fixture: a no-pack, dev-ephemeral Lab, start to finish.
# No Supabase. No Manhattan. No Santo André. No mandatory pack.
#
# Proves the generic kit: open a Lab -> write candidate -> admit valid Act ->
# conformance -> study bench (evidence) -> schedule -> tick -> proof -> learn ->
# export examples.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STORE="${STORE:-/tmp/llk-first-lab}"
NOW="2026-06-07T12:00:00Z"
LAB=(--lab "$ROOT/examples/manifests/lab.json" --profile "$ROOT/profiles/local-only/profile.json" --store "$STORE")
run(){ ( cd "$ROOT" && cargo run -q -p logline-lab-cli -- "$@" ); }

rm -rf "$STORE"

echo "## 0. storage matrix (where admitted Acts can register; external = SOON)"
run storage

echo "## 1. start (dev-ephemeral grade is honest about non-publication)"
run start "${LAB[@]}"

echo "## 2. write an ugly candidate (capture is generous)"
echo '{"did":"rough idea: does promotion require all nine slots?"}' > /tmp/cand.json
run write "${LAB[@]}" --json /tmp/cand.json

echo "## 3. admit a valid Act (promotion is strict)"
run session "${LAB[@]}" --act "$ROOT/examples/acts/first.act.json" | tail -1

echo "## 4. offline conformance"
run conformance >/dev/null && echo "conformance: green"

echo "## 5. run a study bench -> evidence"
echo '{"promoted":true,"all_nine_slots":true}' > /tmp/obs.json
run workbench "${LAB[@]}" --now "$NOW" --bench "$ROOT/benches/candidate-promotion/bench.json" --met --observed /tmp/obs.json | tail -3

echo "## 6. schedule a future obligation (due in the past so the tick catches it)"
cat > /tmp/check.act.json <<'JSON'
{"who":"lab.operator","did":"reproducibility_check","this":{"target":"bench"},"when":"2026-06-06T00:00:00Z","confirmed_by":"none","if_ok":"record","if_doubt":"carry_as_blocked_act","if_not":"skip","status":"candidate"}
JSON
run schedule "${LAB[@]}" --now "$NOW" --act /tmp/check.act.json --due 2026-06-06T00:00:00Z >/dev/null && echo "scheduled"

echo "## 7. tick: confront time (emits tick + disposition + reschedule Acts)"
run tick "${LAB[@]}" --now "$NOW" | tail -8

echo "## 8. proof for the bench scope (claim/evidence/receipt/ghost separated)"
run proof "${LAB[@]}" --act /tmp/check.act.json --scope bench.candidate-promotion

echo "## 9. learn (proposes the next Act)"
run learn "${LAB[@]}" --now "$NOW" | tail -3

echo "## 10. export comparable examples (protocol, not company)"
run conformance --export | head -6

echo "## OK: no-pack dev-ephemeral first Lab completed end-to-end."
