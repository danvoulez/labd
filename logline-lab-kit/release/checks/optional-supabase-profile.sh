#!/usr/bin/env bash
# OPTIONAL adapter evidence — NOT part of the generic v0 gate.
#
# Proves the `supabase-profile` feature COMPILES and its staging tests pass. It
# does NOT prove publication-grade storage: the adapter only stages
# content-addressed payloads. Live admit + write + read-back + hash-preserved +
# reproducible export against a real external spine is a SOON doctor check.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="$ROOT/release/checks/optional-supabase-profile.txt"
cd "$ROOT"

{
  echo "# $(date -u +%Y-%m-%dT%H:%M:%SZ) — OPTIONAL supabase-profile (staging only; NOT publication-grade)"
  echo "## build adapter crate + labd with feature"
  cargo build --workspace --features logline-lab-labd/supabase-profile 2>&1 || cargo build -p logline-lab-supabase -p logline-lab-labd --features logline-lab-labd/supabase-profile 2>&1
  echo "## test adapter + labd feature"
  cargo test -p logline-lab-supabase 2>&1
  cargo test -p logline-lab-labd --features supabase-profile 2>&1
  echo "## NOTE: staged only. publication_grade remains false until a real external"
  echo "##       spine doctor proves admit+write+readback+hash+reproducible export."
} 2>&1 | tee "$OUT"

echo "optional supabase-profile evidence -> $OUT"
