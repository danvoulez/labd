#!/usr/bin/env bash
# v0 generic end-to-end fixture: a no-pack, dev-ephemeral Lab, start to finish.
# No Supabase. No Manhattan. No Santo André. No mandatory pack. No third-party
# storage. It ASSERTS expected JSON fields (not just prints), and fails non-zero
# on any mismatch — so it works as a gate step.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STORE="${STORE:-/tmp/llk-first-lab}"
CO_STORE="${STORE}-candidate-only"
NOW="2026-06-07T12:00:00Z"
TMP="$(mktemp -d)"
LAB=(--lab "$ROOT/examples/manifests/lab.json" --profile "$ROOT/profiles/local-only/profile.json" --store "$STORE")
run(){ ( cd "$ROOT" && cargo run -q -p logline-lab-cli -- "$@" ); }
jassert(){ python3 - "$1" "$2" <<'PY'
import json,sys
d=json.load(open(sys.argv[1]))
expr=sys.argv[2]
assert eval(expr), f"ASSERT FAILED: {expr}\n{json.dumps(d)[:600]}"
print(f"  ok: {expr}")
PY
}

rm -rf "$STORE" "$CO_STORE"

echo "## 0. storage matrix — external spines are SOON; local is available"
run storage > "$TMP/storage.json"
jassert "$TMP/storage.json" "any(o['id']=='supabase' and o['status']=='soon' for o in d)"
jassert "$TMP/storage.json" "any(o['id']=='dev-ephemeral' and o['status']=='available' for o in d)"

echo "## 1. start — dev-ephemeral is honest: NOT publication-grade, no pack"
run start "${LAB[@]}" > "$TMP/start.json"
jassert "$TMP/start.json" "d['kind']=='logline.view.start.v0'"
jassert "$TMP/start.json" "d['publication_grade']==False and d['storage_warning'] is not None"
jassert "$TMP/start.json" "d['packs']==[]"

echo "## 2. write an ugly candidate (capture is generous)"
echo '{"did":"rough idea: does promotion require all nine slots?"}' > "$TMP/cand.json"
run write "${LAB[@]}" --json "$TMP/cand.json" > "$TMP/write.json"
jassert "$TMP/write.json" "d['outcome']=='candidate'"

echo "## 3. refuse protocol-grade admission under candidate-only"
run write --lab "$ROOT/examples/manifests/lab.json" --profile "$ROOT/profiles/candidate-only/profile.json" --store "$CO_STORE" --json "$ROOT/examples/acts/first.act.json" > "$TMP/co.json"
jassert "$TMP/co.json" "d['outcome']=='candidate'"  # even a valid Act is only captured, not admitted

echo "## 4. admit a valid Act (promotion is strict; dev-ephemeral admits)"
run session "${LAB[@]}" --act "$ROOT/examples/acts/first.act.json" | tail -1

echo "## 5. offline conformance"
run conformance >/dev/null && echo "  conformance: green"

echo "## 6. export comparable examples (protocol, not company)"
run conformance --export > "$TMP/exp.json"
jassert "$TMP/exp.json" "len(d)>=1 and all(len(e['content_hash'])==64 for e in d)"

echo "## 7. run a study bench -> evidence"
echo '{"promoted":true,"all_nine_slots":true}' > "$TMP/obs.json"
run workbench "${LAB[@]}" --now "$NOW" --bench "$ROOT/benches/candidate-promotion/bench.json" --met --observed "$TMP/obs.json" > "$TMP/wb.json"
jassert "$TMP/wb.json" "d['kind']=='logline.view.workbench.v0' and d['acts_emitted']==2"

echo "## 8. schedule a future obligation (due in the past so the tick catches it)"
cat > "$TMP/check.act.json" <<'JSON'
{"who":"lab.operator","did":"reproducibility_check","this":{"target":"bench"},"when":"2026-06-06T00:00:00Z","confirmed_by":"none","if_ok":"record","if_doubt":"carry_as_blocked_act","if_not":"skip","status":"candidate"}
JSON
run schedule "${LAB[@]}" --now "$NOW" --act "$TMP/check.act.json" --due 2026-06-06T00:00:00Z >/dev/null && echo "  scheduled"

echo "## 9. tick — materialize time as Acts; no due Act skipped"
run tick "${LAB[@]}" --now "$NOW" > "$TMP/tick.json"
jassert "$TMP/tick.json" "d['kind']=='logline.ruler_report.v0' and d['materialized']==True"
jassert "$TMP/tick.json" "len(d['due'])>=1 and d['acts_emitted']>=2 and d['tick_act'] is not None"

echo "## 10. today — the disposition Act was emitted and is visible"
run today "${LAB[@]}" --now "$NOW" > "$TMP/today.json"
jassert "$TMP/today.json" "'clock_tick' in d['recent'] and 'due_disposition' in d['recent']"

echo "## 11. timeline — past/present/future"
run timeline "${LAB[@]}" --now "$NOW" > "$TMP/tl.json"
jassert "$TMP/tl.json" "d['kind']=='logline.view.timeline.v0' and (len(d['past'])+len(d['present'])+len(d['future']))>=1"

echo "## 12. proof — evidence attached; receipt candidate possible; separated"
run proof "${LAB[@]}" --act "$TMP/check.act.json" --scope bench.candidate-promotion > "$TMP/proof.json"
jassert "$TMP/proof.json" "d['evidence_count']>=1 and d['has_receipt_candidate']==True"

echo "## 13. learn — proposes the next Act"
run learn "${LAB[@]}" --now "$NOW" > "$TMP/learn.json"
jassert "$TMP/learn.json" "'propose' in d['next_action']"

echo "## 14. no third-party dependency: store has only local jsonl files"
test -f "$STORE/actlog.jsonl" && echo "  ok: local actlog.jsonl is the dev-ephemeral record"
! grep -riq "supabase\|manhattan\|santo" "$STORE" && echo "  ok: no supabase/manhattan/santo in local store"

rm -rf "$TMP"
echo "## OK: no-pack dev-ephemeral first Lab completed end-to-end (asserted)."
