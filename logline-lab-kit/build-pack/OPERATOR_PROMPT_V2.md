# OPERATOR PROMPT V2 — LogLine Lab Kit

You are working on one project only:

```txt
logline-lab-kit
```

You have been given a package named something like:

```txt
LOG_LINE_LAB_KIT_BUILD_PACKAGE_V2.zip
```

Your job is to recover, assemble, and build **LogLine Lab Kit** from this package.

Do not rename the project.
Do not create a new meta-project.
Do not ask the user which phase to implement.
Do not treat acceptance-test numbers as menu choices.
Do not reopen the architecture.

---

## 0. Immediate rule

Your first output must not be a question.

Your first output must be an execution report that says you have begun:

```txt
source-fruits classification
repo assembly plan
acceptance status
```

If you ask “Should I continue with Phase X?” before producing those documents, you failed the task.

---

## 1. Mission

Build **LogLine Lab Kit**.

LogLine Lab Kit is an installable Act machine.

It lets a user initialize a Lab, emit Acts, validate them, store them, project them, block them, attach evidence, prepare scoped receipts, load packs/profiles, run workers, expose CLI/API/MCP surfaces, and generate reports.

The product is not minimal.
The product is right-sized and complete for v0.

---

## 2. Closed boundary

There is one project:

```txt
logline-lab-kit
```

There is one build-control folder:

```txt
logline-lab-kit/build-pack/
```

Use this boundary:

```txt
Foundation     = protocol reference / canon / conformance
Lab Kit        = installable generic machinery
Pack           = opinionated conventions
Profile        = infrastructure choice
Lab instance   = identity + config + runtime state
App / adapter  = optional interface or integration
Deployment     = real-world machine/cloud configuration
```

Santo André, Manhattan, Minilab, minilab.work, Pitwall, Cockpit, Hermes, OpenClaw, Supabase, Cloudflare, and the LAB machines are not the product root.

---

## 3. Read order

Read these first:

```txt
00_START_HERE.md
OPERATOR_PROMPT_V2.md
operator/DO_NOT_NEGOTIATE.md
operator/FIRST_RUN_PROTOCOL.md
operator/ACCEPTANCE_DEPENDENCY_RULES.md
01_PROJECT_CHARTER.md
build-pack/00_BUILD_CONTEXT.md
build-pack/01_FINAL_ARCHITECTURE_TREE.md
build-pack/02_REPO_STRUCTURE.md
build-pack/03_MODULE_OWNERSHIP_MATRIX.md
build-pack/04_BUILD_ORDER.md
build-pack/05_ACCEPTANCE_TESTS.md
build-pack/06_GHOSTS_AND_OPEN_DECISIONS.md
build-pack/07_ARCHIVE_AND_DEBRIS_POLICY.md
atlas/ATLAS_TABELA.md
```

Then inspect:

```txt
simple-docs/
labkit-docs/
source-fruits/
repo-skeleton/
```

---

## 4. First required deliverables

Before implementing any feature, create:

```txt
SOURCE_FRUITS_CLASSIFICATION.md
REPO_ASSEMBLY_PLAN.md
ACCEPTANCE_STATUS.md
```

Use the templates in:

```txt
operator/SOURCE_FRUITS_CLASSIFICATION_TEMPLATE.md
operator/REPO_ASSEMBLY_PLAN_TEMPLATE.md
operator/ACCEPTANCE_STATUS_TEMPLATE.md
```

Only after those three documents exist may you implement code.

---

## 5. Core Act rule

Everything consequential starts as a LogLine Act.

The canonical Act has exactly nine slots:

```txt
who
did
this
when
confirmed_by
if_ok
if_doubt
if_not
status
```

Hashes, signatures, runtime data, selected branch, storage timestamps, and envelopes live around the Act, not inside it.

Do not add a tenth slot.

Do not treat files, JSON exports, SQLite rows, UI state, generated reports, app records, or provider responses as semantic truth.

---

## 6. Source fruits policy

Everything under `source-fruits/` is raw material, not authority.

Classify each source fruit as:

```txt
core_candidate
pack_candidate
profile_candidate
runtime_candidate
adapter_candidate
deployment_candidate
documentation_candidate
recovery_only
debris
```

Do not blindly merge old code.

Reject or repair:

```txt
artifact as semantic category
primitive system framing
SQLite as truth/spine
files as official semantic storage
LLM-generated authority
receipt without evidence
fake closure
package embedding secrets
runtime claim without runtime evidence
```

---

## 7. Build order

Follow dependency order.

Do not skip ahead because a later acceptance test was mentioned.

```txt
Phase 0  boundary + classification + assembly plan + acceptance status
Phase 1  Act core
Phase 2  local Lab/outbox
Phase 3  spine profile
Phase 4  projections
Phase 5  Lab host and CLI
Phase 6  evidence / blocked Acts / receipts
Phase 7  clock and hooks
Phase 8  worker boundary
Phase 9  MCP and model boundary
Phase 10 packs
Phase 11 Manhattan L-06 proof
Phase 12 install/release
```

---

## 8. Acceptance dependency rule

Acceptance tests are gates, not tasks to cherry-pick.

```txt
A1-A5    Act core boundary
A6-A9    local/spine/projection boundary
A10-A13 proof discipline boundary
A14-A15 pack/profile extension boundary
A16-A19 app/worker authority boundary
A20-A22 clock/report boundary
A23-A25 pack loading boundary
A26-A28 Manhattan serious proof boundary
A29-A30 recovery/debris boundary
```

If A1-A9 are not green, do not implement A10-A14.

If A1-A13 are not green, do not implement A14-A15.

If pack/profile boundaries are not green, do not implement Manhattan proof.

---

## 9. What ships as Lab Kit core

Build these as generic Lab Kit machinery:

```txt
Act model
exact nine-slot validation
candidate mode
canonical JSON
hashing
branch / verdict behavior
local cache / outbox
spine adapter interface
Supabase / Postgres profile adapter
projection runner
blocked Act machinery
evidence machinery
receipt candidate machinery
clock / due checks
hook runner
generic CLI
labd host
generic MCP server
worker contract
report generator
schemas
conformance hooks
acceptance tests
recovery scans
```

---

## 10. What may live in the repo but is not core

```txt
packs/demo
packs/santo-andre
packs/manhattan
packages/model-middleware
packages/mcp-server
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

These must not define core semantics.

---

## 11. Required repository shape

Build toward:

```txt
logline-lab-kit/
  build-pack/
  foundation/
  crates/
    logline-act/
    logline-lab-core/
    logline-lab-local/
    logline-lab-spine/
    logline-lab-supabase/
    logline-lab-projectors/
    logline-lab-clock/
    logline-lab-hooks/
    logline-lab-dispatch/
    logline-lab-reports/
    logline-lab-cli/
    logline-lab-labd/
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

---

## 12. Output required from implementation runs

Each implementation run must end with:

```txt
files changed
tests run
acceptance tests now green
acceptance tests still red/unknown/blocked
ghosts added/closed
next earliest failing acceptance test
```

Do not say “done” without this.

---

## 13. Strict prohibitions

Do not:

```txt
rename the project
create a new root product
make Santo André the product
make Manhattan the product
make Minilab the product
hardcode Dan-specific details into Lab Kit core
hardcode LAB_8GB/LAB_512/LAB_256 into Lab Kit core
treat source-fruits as authority
call disposable junk archive
treat files as semantic truth
treat SQLite as spine/truth
create receipts without evidence
claim runtime proof from code existence
claim install success without running an install check
claim pack loading if core was modified to fit the pack
hide ghosts
delete source material without noting why
ask which phase to implement before classification/status exists
```

---

## 14. Final sentence to preserve

```txt
LogLine Lab Kit is the instrument.
Packs are the music.
Profiles are the stage.
Workers are the hands.
Apps are the doors.
Labs are the performances.
Acts are the record.
Receipts close only what was proven.
```
