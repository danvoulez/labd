# Storage — ontology before infrastructure

`LogLine Act is the internal truth grammar.` Storage is part of the protocol's
dignity, not a detail (RELEASE_SCOPE §1).

## The rule

A local file / outbox / cache is **capture, transport, scratchpad, or dev
fallback**. It is **never** the protocol-grade place where *admitted* Acts live.
Admitted Acts require a **declared Spine Profile**. The onboarding (`labkit start`
/ `labkit storage`) surfaces that decision honestly.

## Grades

| Grade | Meaning | v0 status |
|---|---|---|
| `candidate-only` | capture candidates only; **cannot admit** protocol-grade Acts | available |
| `dev-ephemeral` | local append-only Act-log; **dev-only / unregistered / non-publication-grade** | available |
| `publication` | a declared external Spine Profile (postgres/neon/supabase/byo) | **SOON** |

A Lab without a publication-grade spine labels its runs `dev-only`. The Start and
Doctor surfaces always report `grade`, `publication_grade`, and a `storage_warning`.

## Why `dev-ephemeral` is honest, not a contradiction

Under `dev-ephemeral` the durable local record is an **append-only log of hashed
Acts** — a legitimate *local spine for development*, explicitly non-publication.
That is different from "any file is truth": the file is a cache/transport of Acts,
and the grade tells the truth about what it is and isn't.

## Onboarding matrix (`labkit storage`)

```
candidate-only   available     capture only
dev-ephemeral    available     local Act-log; dev-only
postgres         soon          external Postgres (--features supabase-profile)
neon             soon          Neon Postgres
supabase         soon          Supabase (--features supabase-profile)
bring-your-own   soon          implement the Spine trait
```

## Supabase/Postgres are optional, not core

The Supabase adapter (`crates/logline-lab-supabase`) is **excluded from the
default build** (`default-members`) and is an **optional, off-by-default feature**
of `labd` (`--features supabase-profile`). The generic kit does not depend on it.

## The Santo André note (the source of the storage noise)

Santo André (Dan's own Lab) needs high-frequency local SQL + an outbox + a
Supabase queue + async ingestion into registered Acts. **That is a Santo-André
deployment/pack, LATER** — it must not define generic Lab Kit storage. The generic
kit only knows the `Spine` trait and the grade ladder above.
