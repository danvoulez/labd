#!/usr/bin/env bash
# A human (developer/scientist) working a Lab entirely headless.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STORE="${STORE:-/tmp/llk-demo-lab}"
NOW="2026-06-07T12:00:00Z"
L=(--lab "$ROOT/examples/manifests/lab.json" --profile "$ROOT/profiles/local-only/profile.json" --store "$STORE")
run(){ ( cd "$ROOT" && cargo run -q -p logline-lab-cli -- "$@" ); }

rm -rf "$STORE"

echo "# 1. Start the Lab (doctor + conformance + next actions)"
run start "${L[@]}"

echo "# 2. Write an ugly candidate (capture is generous)"
echo '{"did":"rough idea: study link health"}' > /tmp/cand.json
run write "${L[@]}" --json /tmp/cand.json

echo "# 3. Schedule an experiment as a future obligation"
cat > /tmp/l06.act.json <<'JSON'
{"who":"manhattan.runtime","did":"ethernet_ping_check","this":{"target":"peer"},"when":"2026-06-06T00:00:00Z","confirmed_by":"none","if_ok":"record","if_doubt":"carry_as_blocked_act","if_not":"skip","status":"candidate"}
JSON
run schedule "${L[@]}" --now "$NOW" --act /tmp/l06.act.json --due 2026-06-06T00:00:00Z

echo "# 4. Today: what is due / overdue / blocked"
run today "${L[@]}" --now "$NOW"

echo "# 5. Run the study bench (observe -> evidence)"
echo '{"stdout":"l06_link_probe_ran"}' > /tmp/obs.json
run workbench "${L[@]}" --now "$NOW" --bench "$ROOT/benches/manhattan-l06/bench.json" --met --observed /tmp/obs.json

echo "# 6. Proof: claim / evidence / receipt / ghost for the scope"
run proof "${L[@]}" --act /tmp/l06.act.json --scope manhattan.L-06

echo "# 7. Learn: what closed/failed/ghosted and the next Act"
run learn "${L[@]}" --now "$NOW"
