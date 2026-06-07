#!/usr/bin/env bash
# Step C fixture: a PROVIDER-FREE resident session, start to finish, proving a Lab
# session is NOT "chat completions plus tools". No provider attached. It ASSERTS JSON
# fields and fails non-zero on mismatch, and it proves resume-after-restart from the
# Lab's Acts (no separate session-truth store).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STORE="${STORE:-/tmp/llk-noprovider-session}"
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

rm -rf "$STORE"

echo "## 1. session start — no provider attached"
run session start "${LAB[@]}" --now "$NOW" > "$TMP/start.json"
jassert "$TMP/start.json" "d['kind']=='logline.view.start.v0'"
jassert "$TMP/start.json" "d['publication_grade']==False"

echo "## 2. session view — the same read surfaces a human/LLM would see"
for s in today timeline schedule learn settings storage; do
  run session view "${LAB[@]}" --now "$NOW" --surface "$s" > "$TMP/$s.json"
  jassert "$TMP/$s.json" "isinstance(d, (dict, list))"
done

echo "## 3. session write — capture human text as a candidate (NOT admitted)"
run session write "${LAB[@]}" --text "does promotion require all nine slots?" > "$TMP/write.json"
jassert "$TMP/write.json" "d['did']=='note' and d['status']=='candidate'"

echo "## 3b. transcript shows a candidate, but NOTHING admitted yet"
run session transcript "${LAB[@]}" > "$TMP/t1.json"
jassert "$TMP/t1.json" "d['kind']=='logline.view.session_transcript.v0'"
jassert "$TMP/t1.json" "any(e['event']=='candidate_drafted' for e in d['events'])"
jassert "$TMP/t1.json" "not any(e['event']=='act_admitted' for e in d['events'])"

echo "## 4. session approve — authorization candidate + the LAB admits the target"
run session approve "${LAB[@]}" --now "$NOW" --json "$ROOT/examples/acts/first.act.json" > "$TMP/approve.json"
jassert "$TMP/approve.json" "d['authorization']['did']=='authorize_promotion'"
jassert "$TMP/approve.json" "d['authorization']['status']=='candidate'"
jassert "$TMP/approve.json" "d['admitted_content_hash'] is not None"

echo "## 5. session tick — confront time"
run session tick "${LAB[@]}" --now "$NOW" > "$TMP/tick.json"
jassert "$TMP/tick.json" "isinstance(d, dict)"

echo "## 6. RESUME after restart — a fresh process reopens the SAME store"
test ! -f "$STORE/session.json" && echo "  ok: no separate session-truth store"
run session transcript "${LAB[@]}" > "$TMP/t2.json"
jassert "$TMP/t2.json" "any(e['event']=='act_admitted' for e in d['events'])"
jassert "$TMP/t2.json" "any(e['event']=='candidate_drafted' for e in d['events'])"

echo "## 7. session close"
run session close "${LAB[@]}" | tail -1

echo "PASS — provider-free resident session: presence over Acts, resumed from the Lab."
