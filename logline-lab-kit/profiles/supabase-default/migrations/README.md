# Supabase / Postgres spine migrations

Recovered from source fruit `LogLine-Lab-Kit-online-spine` (recovery-order #1),
which carried the spine circuit most directly. These are **promoted SQL**, not
authority by mere existence — each was inspected during recovery.

## The spine table

`0001_ops_logline_acts.sql` declares `ops.logline_acts`. The nine canonical slots
are columns (`who`/`did`/`status` text; `this`/`confirmed_by`/`if_ok`/`if_doubt`/
`if_not` jsonb). Everything else (`runtime_envelope`, `tuple_hash`, `content_hash`,
`previous_act_refs`, `evidence_state`, `promotion_state`, timestamps) is envelope
**around** the Act — there is no tenth slot (Operator §5).

Two constraints enforce proof discipline at the storage edge:

- `logline_receipt_candidate_requires_evidence` — a `prepare_receipt_candidate`
  Act must carry a non-empty `this.evidence_refs` (no receipt without evidence).
- `logline_evidence_redaction_required` — a `report_execution_result` must mark
  `secret_redacted = true`.

## Idempotency & immutability

`0011_ingest_idempotent_immutable.sql` is the audited write boundary
`ops.ingest_logline_act(payload jsonb)`: content-addressed dedupe on
`content_hash`, append-only (update/delete forbidden by trigger). The
`logline-lab-supabase` adapter builds exactly this payload.

## Status

| Migration | Status | Notes |
|---|---|---|
| 0001–0012 | portable | Verified shape against stock Postgres 16 per source notes. |
| 0013_queues_pgmq | **GHOST** `pgmq/pg_cron queue runtime` | Extension-dependent; NOT in stock Postgres. Apply on real Supabase only; queue workers unimplemented. |

GHOST `supabase-live-ingest`: applying these against a real project and capturing
an ingest row as evidence is the closure condition (see
`recovery/REPO_ASSEMBLY_PLAN.md`).
