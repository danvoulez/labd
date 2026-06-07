-- 0013: consequence queues (pgmq) for the ADR-0002 flow.
--
-- EXTENSION-DEPENDENT — requires the Supabase `pgmq` (and later `pg_cron`)
-- extensions. NOT verified against stock Postgres (those extensions are not
-- bundled there). Apply on the real Supabase instance.
-- Ghost: queue-layer-runtime-unverified.
--
-- ADR-0002 flow:
--   local outbox -> q_lab_outbox -> worker -> ops.ingest_logline_act (0011)
--   consequences -> q_workorders / q_receipts / q_projection_rebuild
--   temp artifacts -> q_artifact_cleanup
-- Failures that exhaust bounded retries -> ops.dead_letters (0012) + ghost.

create extension if not exists pgmq;

do $$
declare
  q text;
begin
  foreach q in array array[
    'q_lab_outbox',
    'q_workorders',
    'q_receipts',
    'q_projection_rebuild',
    'q_artifact_cleanup'
  ] loop
    if not exists (select 1 from pgmq.list_queues() where queue_name = q) then
      perform pgmq.create(q);
    end if;
  end loop;
end $$;

-- pg_cron drives queue maintenance (e.g. draining q_artifact_cleanup to enforce
-- the 24h artifact TTL remotely; the local spool is enforced by
-- logline-lab-artifacts::gc). The schedules are intentionally NOT created here:
-- scheduling a no-op before the worker exists would be plastic. They attach when
-- the queue workers land.
-- Ghost: queue-workers-unimplemented.
create extension if not exists pg_cron;
