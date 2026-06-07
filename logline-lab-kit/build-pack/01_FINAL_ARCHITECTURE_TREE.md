# 01 — Final Architecture Tree

```txt
LogLine Foundation
  protocol reference
  Act form
  canonicalization
  hash / signature
  conformance
  governance

LogLine Lab Kit
  installable generic Lab machinery
  Act core
  local outbox
  spine adapters
  projectors
  blocked Acts
  evidence
  receipt candidates
  clock
  hooks
  CLI
  labd
  MCP server
  worker contract
  reports
  benches
  recovery

Profiles
  local-only
  postgres
  supabase
  filesystem-manual export/debug

Packs
  demo
  santo-andre
  manhattan

Apps / Adapters
  MCP server
  model middleware
  pitwall adapter
  cockpit adapter
  Hermes adapter
  OpenClaw adapter
  Manhattan MCP wrapper

Runtimes
  shell worker
  Hermes adapter
  OpenClaw adapter
  Manhattan runtime

Deployments
  local
  Supabase
  Cloudflare
  LAB_8GB
  LAB_512
  LAB_256

Live Labs
  Santo André Lab
  Manhattan Lab
  future Personal Offline Lab
```

## Dependency direction

```txt
Foundation refs
  ↓
Lab Kit core
  ↓
Profiles + Packs
  ↓
Lab instances
  ↓
Apps / Runtimes / Deployments
```

Never upward.

## Boundary examples

```txt
Lab Kit may load Manhattan Pack.
Lab Kit must not import Manhattan code.

Lab Kit may ship a Supabase profile.
Foundation must not require Supabase.

A cockpit may read Lab Kit projections.
Lab Kit must not depend on the cockpit.

A worker may execute admitted workorders.
The worker must not define semantic truth.
```
