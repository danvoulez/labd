# 02 — Repo Structure

The repository should be shaped as one project:

```txt
logline-lab-kit/
  README.md
  Cargo.toml
  LICENSE
  CHANGELOG.md

  build-pack/
  foundation/
  crates/
  schemas/
  conventions/
  profiles/
  packs/
  manifests/
  projectors/
  hooks/
  benches/
  reports/
  examples/
  tests/
  generator/
  recovery/
  packages/
  apps/
  runtimes/
  deploy/
  install/
  release/
  docs/
```

## Core crates

```txt
crates/logline-act
crates/logline-lab-core
crates/logline-lab-local
crates/logline-lab-spine
crates/logline-lab-supabase
crates/logline-lab-projectors
crates/logline-lab-clock
crates/logline-lab-hooks
crates/logline-lab-dispatch
crates/logline-lab-reports
crates/logline-lab-cli
crates/logline-lab-labd
```

## Non-core but same repo

```txt
packs/demo
packs/santo-andre
packs/manhattan
packages/mcp-server
packages/model-middleware
packages/ts-spine-client
apps/pitwall-adapter
apps/cockpit-adapter
runtimes/shell-worker
runtimes/hermes-adapter
runtimes/openclaw-adapter
runtimes/manhattan
deploy/local
deploy/supabase
deploy/cloudflare
deploy/lab8gb
deploy/lab512
deploy/lab256
```

## Naming rule

Do not create additional project roots.
Every new concern must fit into one of:

```txt
core crate
schema
convention
profile
pack
app/adapter
runtime
deployment
doc
bench/test
recovery scan
```
