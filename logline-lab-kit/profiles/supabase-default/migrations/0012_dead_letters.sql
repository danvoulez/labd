-- 0012: dead-letter store. ADR-0002: "no infinite retry" — a delivery that
-- exhausts its bounded retries lands here together with the ghost that explains
-- why, instead of being silently dropped.
-- Portable DDL — verified on stock Postgres 16.

create table if not exists ops.dead_letters (
  id              uuid primary key default gen_random_uuid(),
  source_queue    text not null,
  idempotency_key text,
  payload         jsonb not null,
  error           text,
  retries         integer not null default 0,
  ghost_id        text,
  created_at      timestamptz not null default now()
);

create index if not exists dead_letters_queue_idx on ops.dead_letters (source_queue);
create index if not exists dead_letters_key_idx on ops.dead_letters (idempotency_key);

-- A dead letter is a settled failure record; keep it append-only like the spine.
create or replace function ops.forbid_dead_letter_mutation()
returns trigger language plpgsql as $$
begin
  raise exception 'ops.dead_letters is append-only (attempted %)', tg_op
    using errcode = 'check_violation';
end;
$$;

drop trigger if exists dead_letters_immutable on ops.dead_letters;
create trigger dead_letters_immutable
  before update or delete on ops.dead_letters
  for each row execute function ops.forbid_dead_letter_mutation();
