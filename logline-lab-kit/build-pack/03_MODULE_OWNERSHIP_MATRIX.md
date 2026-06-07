# 03 — Module Ownership Matrix

| Thing | Lives in | Ships with Lab Kit core? | Specific? | Notes |
|---|---|---:|---|---|
| Act model | `crates/logline-act` | yes | no | exact nine-slot model |
| Canon refs | `foundation/` | referenced/vendor | no | source of conformance, not runtime truth |
| labd | `crates/logline-lab-labd` | yes | no | generic Lab host |
| CLI | `crates/logline-lab-cli` | yes | no | generic command surface |
| Local outbox | `crates/logline-lab-local` | yes | no | provisional cache only |
| Spine trait | `crates/logline-lab-spine` | yes | no | generic ingest/sync/idempotency |
| Supabase adapter | `crates/logline-lab-supabase` + `profiles/supabase` | yes | profile | optional profile, not universal canon |
| Projectors | `crates/logline-lab-projectors` + `projectors/` | yes | no | read models only |
| Clock | `crates/logline-lab-clock` | yes | no | tick/due/capacity |
| Worker contract | `crates/logline-lab-dispatch` | yes | no | contract, not implementation monopoly |
| MCP server | `packages/mcp-server` | optional | generic | app connection surface |
| Model middleware | `packages/model-middleware` | optional | generic | AI SDK wrapper, not truth |
| Pitwall adapter | `apps/pitwall-adapter` | no | app | bench integration |
| Cockpit adapter | `apps/cockpit-adapter` | no | app | UI/projection integration |
| Santo André Pack | `packs/santo-andre` | optional | yes | reference pack, not product root |
| Manhattan Pack | `packs/manhattan` | optional | yes | physical fleet conventions |
| Manhattan runtime | `runtimes/manhattan` | no | yes | macOS implementation |
| LAB_8GB config | `deploy/lab8gb` | no | yes | deployment |
| LAB_512 config | `deploy/lab512` | no | yes | deployment |
| LAB_256 config | `deploy/lab256` | no | yes | deployment |
| Cloudflare config | `deploy/cloudflare` | no | yes | deployment |
| Supabase project values | `deploy/supabase` | no | yes | deployment secrets/config refs only |
