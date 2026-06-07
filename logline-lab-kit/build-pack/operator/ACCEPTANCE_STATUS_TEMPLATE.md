# ACCEPTANCE_STATUS.md Template

| ID | Acceptance test | Status | Evidence | Test command | Responsible module | Notes |
|---|---|---|---|---|---|---|
| A1 | Act with exactly nine slots validates. | unknown |  |  | `crates/logline-act` |  |
| A2 | Act with a missing slot fails. | unknown |  |  | `crates/logline-act` |  |
| A3 | Act with a tenth canonical slot fails. | unknown |  |  | `crates/logline-act` |  |
| A4 | Same Act produces same canonical hash. | unknown |  |  | `crates/logline-act` |  |
| A5 | Ugly candidate can be preserved. | unknown |  |  | `crates/logline-act` |  |
| A6 | Local emit stores Act in local cache/outbox. | unknown |  |  | `crates/logline-lab-local` |  |
| A7 | Re-emitting same Act is idempotent. | unknown |  |  | `crates/logline-lab-local` |  |
| A8 | Sync writes Act to configured spine. | unknown |  |  | `crates/logline-lab-spine` |  |
| A9 | Projection reads Act from spine. | unknown |  |  | `crates/logline-lab-projectors` |  |
| A10 | Blocked Act appears when evidence or permission is missing. | unknown |  |  | `crates/logline-lab-projectors` |  |
| A11 | Evidence can be attached to scope. | unknown |  |  | `crates/logline-lab-core` |  |
| A12 | Receipt candidate names exact scope. | unknown |  |  | `crates/logline-lab-core` |  |
| A13 | Report does not pretend to be receipt. | unknown |  |  | `crates/logline-lab-reports` |  |
| A14 | Pack loads without changing core. | unknown |  |  | `crates/logline-lab-core` |  |
| A15 | Profile loads without changing core. | unknown |  |  | `crates/logline-lab-core` |  |
| A16 | App/MCP call becomes draft Act. | unknown |  |  | `packages/mcp-server` |  |
| A17 | Unauthorized app action is blocked. | unknown |  |  | `packages/mcp-server` |  |
| A18 | Worker cannot execute without allow. | unknown |  |  | `crates/logline-lab-dispatch` |  |
| A19 | Worker returns evidence, not closure. | unknown |  |  | `crates/logline-lab-dispatch` |  |
| A20 | Clock tick emits or evaluates due Acts. | unknown |  |  | `crates/logline-lab-clock` |  |
| A21 | Due Act resolves or creates reschedule Act. | unknown |  |  | `crates/logline-lab-clock` |  |
| A22 | Lab report renders state from projections. | unknown |  |  | `crates/logline-lab-reports` |  |
| A23 | Demo pack completes first session. | unknown |  |  | `packs/demo` |  |
| A24 | Santo André pack loads without becoming core. | unknown |  |  | `packs/santo-andre` |  |
| A25 | Manhattan pack loads without becoming core. | unknown |  |  | `packs/manhattan` |  |
| A26 | Manhattan L-06 emits evidence. | unknown |  |  | `packs/manhattan` |  |
| A27 | L-06 receipt closes only L-06. | unknown |  |  | `packs/manhattan` |  |
| A28 | Projection shows L-06 health from Acts. | unknown |  |  | `packs/manhattan` |  |
| A29 | Recovery scan catches false authority. | unknown |  |  | `recovery/` |  |
| A30 | Storage scan rejects file/SQLite truth language. | unknown |  |  | `recovery/` |  |
