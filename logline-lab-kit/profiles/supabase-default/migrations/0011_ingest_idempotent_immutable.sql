-- 0011: content-addressed idempotency + append-only immutability + audited ingest.
-- Aligns the remote spine with the local outbox (idempotency_key = content_hash)
-- and ADR-0002: "no silent drop", "no double send", immutable ledger.
-- Portable DDL — verified on stock Postgres 16.

-- Idempotent ingest key: the canonical content hash dedupes re-delivered acts.
-- A plain (non-partial) unique index so `ON CONFLICT (content_hash)` can target it;
-- NULL content_hash rows remain allowed (NULLs are distinct in a unique index).
create unique index if not exists logline_acts_content_hash_uk
  on ops.logline_acts (content_hash);

-- Append-only ledger: an act is immutable once written.
create or replace function ops.forbid_act_mutation()
returns trigger language plpgsql as $$
begin
  raise exception 'ops.logline_acts is append-only (attempted %)', tg_op
    using errcode = 'check_violation';
end;
$$;

drop trigger if exists logline_acts_immutable on ops.logline_acts;
create trigger logline_acts_immutable
  before update or delete on ops.logline_acts
  for each row execute function ops.forbid_act_mutation();

-- Audited write boundary. With RLS deny-all (0010), this SECURITY DEFINER function
-- is the only sanctioned ingest path (matches the DATABASE_ALLOW_OPS_LOGINE_ACT_INSERT
-- governance flag). Idempotent on content_hash; never updates, so immutability holds.
--
-- Slots arrive as the engine models them: who/did/status are text; this/confirmed_by/
-- if_ok/if_doubt/if_not are jsonb (a bare string scalar is valid jsonb, so the engine's
-- flat-string slots round-trip without being forced into objects).
create or replace function ops.ingest_logline_act(payload jsonb)
returns ops.logline_acts
language plpgsql
security definer
set search_path = ops, public
as $$
declare
  result        ops.logline_acts;
  v_content_hash text := payload->>'content_hash';
begin
  if coalesce(trim(payload->>'who'), '') = '' then
    raise exception 'ingest_logline_act: who is required';
  end if;
  if coalesce(trim(payload->>'did'), '') = '' then
    raise exception 'ingest_logline_act: did is required';
  end if;
  if coalesce(trim(payload->>'status'), '') = '' then
    raise exception 'ingest_logline_act: status is required';
  end if;

  -- Fast path: already ingested (content-addressed dedupe).
  if v_content_hash is not null then
    select * into result from ops.logline_acts where content_hash = v_content_hash;
    if found then
      return result;
    end if;
  end if;

  insert into ops.logline_acts
    (who, did, this, "when", confirmed_by, if_ok, if_doubt, if_not, status,
     runtime_envelope, tuple_hash, content_hash)
  values (
    payload->>'who',
    payload->>'did',
    coalesce(payload->'this', '""'::jsonb),
    coalesce((payload->>'when')::timestamptz, now()),
    coalesce(payload->'confirmed_by', '{}'::jsonb),
    coalesce(payload->'if_ok', '{}'::jsonb),
    coalesce(payload->'if_doubt', '{}'::jsonb),
    coalesce(payload->'if_not', '{}'::jsonb),
    payload->>'status',
    coalesce(payload->'runtime_envelope', '{}'::jsonb),
    payload->>'tuple_hash',
    v_content_hash
  )
  on conflict (content_hash) do nothing
  returning * into result;

  -- Lost the race (concurrent ingest of the same content_hash): read the winner.
  if result.id is null and v_content_hash is not null then
    select * into result from ops.logline_acts where content_hash = v_content_hash;
  end if;

  return result;
end;
$$;
