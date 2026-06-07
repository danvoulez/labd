# LOG_LINE_LAB_KIT_FINAL_REAL_PROJECT_DOC.md

## LogLine Lab Kit — final real project document

**Project root:** `logline-lab-kit`  
**Document status:** final project definition / controlling build document  
**Date:** 2026-06-07  
**Preserves:** current lean package boundary, Act-centered canon, build-pack structure, pack/profile/app/runtime/deployment separation  

---

## 0. Central thesis

```txt
LogLine Lab Kit is an installable kit for forming Labs that study,
practice, test, prove, and transmit LogLine through LogLine Acts.
```

The product definition is not primarily a temporal product. It is primarily:

```txt
a Lab formation kit for studying LogLine
and making LogLine behave like a protocol.
```

The Lab is not a passive API. Time matters because real study, work, proof, repetition, and learning happen across time, so the Lab must have clocks, schedules, ticks, deadlines, capacity, and future obligations. But time is infrastructure. It is not the mission.

The mission is to form Labs that study LogLine.

The strategic undercurrent is to establish LogLine as a protocol, not as a company.

---

## 1. One-line version

```txt
Build the installable Lab Kit that lets independent Labs study LogLine,
run LogLine Acts, compare results, and spread LogLine as a protocol.
```

---

## 2. What this document does

This document collapses the current lean package, the scientific method, the human experience layer, and the protocol strategy into one final project authority.

It preserves the solved package decisions:

```txt
one project root:       logline-lab-kit
one build pack:         logline-lab-kit/build-pack
one canonical unit:     LogLine Act
one package grammar:    core / pack / profile / app / runtime / deployment
one serious proof:      Manhattan L-06
```

It corrects the product definition:

```txt
not merely an Act machine
not merely a temporal machine
not merely an API
not merely a CLI
not merely a repository skeleton
not merely a scheduler
not merely a dashboard
not Dan's private Minilab
not Santo Andre as universal canon
not Manhattan as the product
not a company workspace
not a SaaS trap
```

The final project is:

```txt
an installable kit for forming operational Labs
where LogLine is studied through Acts,
where scientific practice is encoded as Lab work,
where human operators can write, schedule, run, prove, and learn,
and where independent Labs can interoperate because LogLine is protocol-shaped.
```

---

## 3. Public objective and strategic objective

### 3.1 Public objective

The public objective is to let a capable person or group instantiate a Lab that can study LogLine seriously.

A Lab formed by the Kit must be able to:

```txt
load LogLine canon/reference material
load packs and profiles
create or inspect a Lab manifest
write candidate Acts
admit valid Acts
preserve ugly candidate Acts without pretending they are final
run study benches
schedule future work
execute allowed work
attach evidence
prepare scoped receipts
leave ghosts where proof is missing
produce reports
learn from completed, failed, blocked, and unresolved work
compare behavior with conformance examples
```

The product is not just about recording. It is about forming a place where LogLine can be practiced.

### 3.2 Strategic objective

The quiet strategic objective is:

```txt
make LogLine a protocol, not a company.
```

That means:

```txt
many Labs can exist
Labs can be independently installed
Labs can load different packs
Labs can use different profiles and deployments
Labs can produce comparable Acts
Labs can run shared conformance tests
Labs can publish examples without depending on one central service
Labs can fork practice without forking canon
Labs can disagree locally while remaining protocol-compatible
```

A company may support, host, fund, distribute, or provide services around LogLine. But the project must not make the company the semantic center.

The center is:

```txt
LogLine Act + conformance + independent Labs.
```

### 3.3 Protocol over product captivity

A wrong build tries to win by capturing users inside a proprietary workspace.

A correct build wins by making LogLine portable, inspectable, installable, testable, and transmissible.

The project should feel less like:

```txt
Sign up for our platform.
```

And more like:

```txt
Install a Lab.
Load the canon.
Run the tests.
Study through Acts.
Compare with other Labs.
Publish your practice.
```

---

## 4. Project identity

### 4.1 Name

```txt
logline-lab-kit
```

Do not rename the project.

Do not introduce a second root such as:

```txt
logline-temporal-lab
logline-ruler
logline-science-kit
santo-andre-kit
minilab-kit
logline-protocol-company
```

Internal crates and modules may have focused names. The project remains `logline-lab-kit`.

### 4.2 Product name

```txt
LogLine Lab Kit
```

### 4.3 Product class

```txt
installable Lab formation kit
```

Secondary class:

```txt
protocol study and conformance environment for LogLine Acts
```

### 4.4 Product promise

A capable operator can install LogLine Lab Kit and instantiate a Lab that studies LogLine by living in Acts.

A serious Lab can:

```txt
remember what it is studying
express study as Acts
turn questions into benches
turn benches into scheduled work
turn work into evidence
turn evidence into scoped receipts or ghosts
turn history into learning
turn practice into comparable protocol examples
```

### 4.5 Internal promise

The Lab Kit gives LogLine a place to be practiced.

It is the smallest credible installable system where LogLine is not only described, but exercised.

---

## 5. Non-goals

LogLine Lab Kit is not:

```txt
LogLine Foundation
Santo Andre Lab
Manhattan
minilab.work
Dan Control Plane
Pitwall
Cockpit
Hermes
OpenClaw
Supabase
Cloudflare
LAB_8GB
LAB_512
LAB_256
```

Those may appear as:

```txt
reference material
pack
profile
adapter
runtime
deployment
proof fixture
practice example
```

They do not define the generic product.

LogLine Lab Kit is also not:

```txt
a passive REST API
a generic dashboard
a task manager
a calendar app
a workflow toy
a custom database
a philosophical text
a pile of scripts
a generic agentic workspace
a company control plane
a hosted SaaS dependency
a franchise system for official Labs
```

It may expose APIs, dashboards, schedules, workflows, texts, scripts, and agents. None of those is the product.

The product is the installable Lab that lets people study and practice LogLine through Acts.

---

## 6. Constitutional architecture law

The core law remains:

```txt
Act is the canonical unit.
Everything else is envelope, convention, projection, pack, profile, app, runtime, or deployment.
```

The Lab law is:

```txt
A Lab is an operational institution for studying LogLine through Acts.
```

The protocol law is:

```txt
No Lab, pack, company, deployment, or UI owns LogLine.
Protocol compatibility is established through Act shape, conformance, reference behavior, and comparable proof.
```

Together:

```txt
Act is the unit.
Canon/reference defines the grammar.
Conformance tests make compatibility visible.
Lab is the study institution.
Pack is local practice.
Profile is infrastructure choice.
App is participation surface.
Runtime is execution capacity.
Projection is readable interpretation.
Evidence is the bridge to reality.
Receipt is scoped closure.
Ghost is named missing proof.
Learning is the return from history.
```

### 6.1 No parallel semantic model

The project must not create native semantic domains competing with Act.

Do not make authoritative native models such as:

```txt
Task
Project
Workflow
Experiment
Schedule
Evidence
Receipt
Ghost
Probe
ProtocolMessage
Study
LabReport
```

as semantic truth outside Acts.

Those concepts may exist as:

```txt
Act kinds
Act conventions
schemas over Act payloads
projection views
pack practices
runtime envelopes
report projections
```

The rule is simple:

```txt
If it matters semantically, it must be expressible as LogLine Acts.
```

### 6.2 No independent artifact category

Do not introduce `Artifact` as a semantically authoritative layer.

Files, outputs, blobs, receipts, traces, screenshots, logs, and generated materials only matter through:

```txt
Acts
references
paths
evidence links
hashes
runtime observations
projections
```

A file does not become truth because it exists. It becomes relevant because an Act refers to it and a Lab can reason about that reference.

### 6.3 Candidate generosity, promotion strictness

The Lab must be generous at capture.

Humans and models will produce imperfect Acts. That is acceptable. Candidate capture should be permissive.

The Lab must be strict at promotion.

Promotion into admitted Acts, proof, receipts, conformance, and protocol claims requires explicit checks.

This distinction is essential:

```txt
capture ugly candidates
improve later
never pretend unproven material is closed truth
```

---

## 7. LogLine Act

The current package law remains valid.

A LogLine Act is the canonical semantic unit.

The final shape remains nine canonical slots:

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

No Lab Kit implementation may replace this with a richer native object model.

### 7.1 Slot meanings

```txt
who            actor, author, authority, agent, person, system, or role
did            act verb / action / declaration / event / claim
this           object or subject of the act
when           time or declared temporal position
confirmed_by   signature, witness, rule, model, receipt, validator, or absent proof
if_ok          route when support is sufficient
if_doubt       route when proof, permission, or clarity is missing
if_not         route when the claim fails or is contradicted
status         state of the Act under the declared practice
```

The slots are stable. Packs and profiles may add conventions. They do not add new canonical slots.

### 7.2 Status vocabulary

Status may be represented by Act conventions and projections such as:

```txt
candidate
admitted
scheduled
due
running
blocked
evidence_attached
receipt_candidate
closed
ghosted
rescheduled
failed
learned
```

These are Lab practice states, not new semantic primitives.

---

## 8. Labs as protocol study cells

The Lab is the main unit of adoption.

A Lab is not merely a deployment. It is a place where a person, group, machine, course, project, institution, or research effort studies LogLine in practice.

A Lab has:

```txt
manifest
identity
canon/reference links
profile
packs
local policy
clock behavior
study benches
runtime grants
projection surfaces
reports
conformance tests
history
```

A Lab does not own LogLine.

A Lab demonstrates LogLine.

### 8.1 Why Labs matter

Protocols spread through independent practice.

A protocol is weak if it only exists inside one hosted product.

A protocol becomes serious when multiple independent places can:

```txt
install it
interpret it
produce valid records
run the same tests
compare behavior
share examples
extend practice locally
survive without central permission
```

That is why the product is a Lab Kit.

The Kit should create more Labs, not more dependency on a company.

### 8.2 Lab formation cycle

A new Lab should pass through this cycle:

```txt
1. Start Lab
2. Load canon/reference
3. Select profile
4. Select pack
5. Write first candidate Act
6. Admit first valid Act
7. Run first study bench
8. Produce first evidence path
9. Produce first report
10. Learn first next action
11. Run conformance tests
12. Export/share a comparable example
```

This cycle is more important than a feature list.

A Lab that cannot teach LogLine through use is not a successful Lab.

### 8.3 Labs are not official branches

There should be no concept of a single official Lab hierarchy.

Santo Andre may be Dan's recommended/default practice pack.

Manhattan may be a serious proof fixture.

A university course may create its own teaching pack.

A company may create an internal operations pack.

A local machine may create a personal offline Lab profile.

None of those becomes canon merely by existing.

The canon remains reference-level. Packs remain practice-level.

---

## 9. Scientific method as Lab bureaucracy

The Lab is not a generic automation runner.

The Lab studies.

Scientific method is not decorative. It is the core bureaucracy of serious Lab work.

A scientific cycle in the Lab is:

```txt
observe
question
hypothesize
design probe
schedule or run probe
collect evidence
compare result against expectation
produce verdict
close scoped receipt or preserve ghost
learn next action
```

Every step can be represented as Acts.

### 9.1 Research Acts

A research Act may express:

```txt
observation
question
hypothesis
probe design
experiment order
measurement
result
interpretation
verdict
learning
next action
```

Example pattern:

```txt
who: lab
 did: proposed
 this: hypothesis H-001 — explain observed mismatch in Act promotion behavior
 when: 2026-06-07T00:00:00Z
 confirmed_by: pending
 if_ok: schedule study-bench://candidate-promotion
 if_doubt: open blocked Act for missing conformance vectors
 if_not: record contradicted hypothesis
 status: candidate
```

This is not a new object type. It is an Act with a research convention.

### 9.2 Study benches

A study bench is a repeatable practice for testing a thesis.

A bench may include:

```txt
question
hypothesis
inputs
procedure
expected observations
allowed tools
success criteria
failure criteria
receipt scope
ghost policy
learning output
```

A bench is not a semantic primitive. It is a pack/profile convention that produces Acts.

### 9.3 Ghosts

A ghost is named missing proof.

A ghost is created when the Lab cannot honestly close something.

Examples:

```txt
missing evidence
ambiguous result
blocked permission
failed runtime
unconfirmed witness
partial measurement
conflicting reports
```

Ghosts matter because a Lab must preserve doubt without pretending closure.

---

## 10. Time as Lab infrastructure

Time is essential, but it is not the product slogan.

A Lab needs time because study has duration:

```txt
questions wait
experiments run later
obligations become due
evidence arrives after action
failures require retries
learning depends on history
capacity matters
```

The Lab must therefore include clocks, schedules, ticks, timelines, due work, overdue work, reschedules, and reports.

The framing is: a Lab studies LogLine in time. Temporal machinery serves study; it is not the product identity.

### 10.1 Clock discipline

The Lab's clock behavior must be explicit.

It should produce visible Acts or Act-derived records for:

```txt
ticks
scheduled checks
due work discovered
overdue work
blocked work
reschedules
capacity reports
maintenance reports
```

No invisible cron magic.

### 10.2 Ruler behavior

`logline-lab-ruler` remains a valid internal module name.

Its job is to compare current time against the Lab's Acts and projections.

It answers:

```txt
What is due?
What is overdue?
What is blocked?
What can run now?
What must wait?
What needs evidence?
What requires human permission?
What study bench should run next?
Where is the Lab idle in a harmful way?
```

The ruler is important because Labs must not silently skip obligations.

But it is a component. It is not the project identity.

### 10.3 Capacity band

A Lab has a capacity band.

A Lab with no legitimate study or work for too long is failing its usefulness contract.

The correct response is not fake busywork. It is to surface the state:

```txt
no executable work
no pending study bench
no due obligation
under-capacity warning
next legitimate study proposal
```

---

## 11. Human experience

The human experience is how the Lab teaches and operates.

The user is not just calling APIs. The user is entering a Lab.

The core experience surfaces are:

```txt
Start
Today
Timeline
Write
Schedule
Workbench
Proof
Learn
Settings
```

They may be implemented through CLI, TUI, web, MCP, ChatGPT, local app, phone view, or cockpit. The surface is flexible. The experience grammar is not.

### 11.1 Start

Start declares or opens a Lab.

It should help the operator:

```txt
inspect Lab manifest
select profile
load pack
verify canon/reference links
run first conformance check
write first candidate Act
produce first report
```

Start is the first study moment: can this Lab exist and remember what happened?

### 11.2 Today

Today shows the current operational study state:

```txt
due Acts
overdue Acts
blocked Acts
running work
recent evidence
recent receipts
open ghosts
recommended next study
capacity state
```

### 11.3 Timeline

Timeline shows past, present, and future Acts.

It is not a calendar clone. It is a projection of the Lab's contract and study chain.

### 11.4 Write

Write captures candidate Acts.

It must support ugly capture.

It should help humans and models produce useful candidates without demanding hand-authored perfect JSON.

### 11.5 Schedule

Schedule places Acts into future obligation.

A scheduled Act is not just a reminder. It is a future contract the Lab must later confront.

### 11.6 Workbench

Workbench runs study benches.

It should expose:

```txt
questions
hypotheses
probes
inputs
procedure
expected result
actual result
evidence
ghost/receipt path
learning
```

Workbench is where LogLine becomes a method, not just a syntax.

### 11.7 Proof

Proof separates:

```txt
claim
evidence
witness
receipt candidate
ghost
```

The Lab must never allow reports or model text to masquerade as proof.

### 11.8 Learn

Learn reads history back into the Lab.

It summarizes:

```txt
what closed
what failed
what remained ghosted
what repeated
what improved
what should be studied next
```

### 11.9 Settings

Settings exposes configuration, not authority.

It may configure:

```txt
profile
pack
clock
ruler behavior
gate
runtime
MCP/app grants
spine
local paths
```

Settings must not let configuration bypass Act discipline, proof discipline, or gate policy.

---

## 12. System architecture

The final architecture has these layers:

```txt
Foundation/reference material
Lab Kit core
Lab host: labd
Engine + constitutional-runtime
Clock/ruler behavior
Local outbox/cache
Spine/profile layer
Projection layer
Packs
Apps/MCP/model middleware
Runtimes/workers
Deployments/live Labs
```

### 12.1 Foundation/reference material

Foundation/reference material supplies:

```txt
Act grammar
canonical examples
conformance vectors
hash/signature expectations
reference behavior
```

Foundation/reference is not the app.

Foundation/reference is not a Lab instance.

Foundation/reference is not the company.

### 12.2 Lab Kit core

Lab Kit core supplies:

```txt
Act types
validation
canonicalization
hashing
manifest loading
pack loading
profile loading
local storage boundary
report generation boundary
projection contracts
conformance runner
```

### 12.3 `labd`

`labd` is the resident Lab host.

It keeps a Lab alive.

It should:

```txt
load Lab manifest
load profile
load packs
maintain local outbox/cache
run clock/ruler loops
invoke gate
issue workorders
receive worker reports
record evidence refs
prepare reports
serve local APIs or MCP surfaces
```

`labd` is not a generic API server. It is the Lab host.

### 12.4 Engine + constitutional-runtime

The engine and constitutional-runtime are the canonical runtime brain.

They enforce:

```txt
Act shape
promotion rules
policy checks
receipt scope
proof discipline
conformance expectations
```

They do not replace the Lab. They are inside the Lab host.

### 12.5 Clock/ruler

Clock/ruler behavior ensures that the Lab confronts time.

It must be explicit, reportable, and Act-visible.

The internal module may remain:

```txt
logline-lab-ruler
```

but the product should not market itself as a ruler. The product forms Labs.

### 12.6 Local outbox/cache

The Lab must work locally before it works through external infrastructure.

Local outbox/cache stores:

```txt
candidate Acts
admitted Acts waiting for sync
pending evidence refs
pending reports
sync status
```

### 12.7 Spine/profile layer

A profile chooses infrastructure.

Possible profiles:

```txt
local-only
Postgres
Supabase
Cloudflare/D1
hybrid
course lab
company lab
personal offline lab
```

Profile is not meaning. Profile is operational placement.

### 12.8 Projection layer

Projections make Acts readable.

Examples:

```txt
Today
Timeline
Proof
Learn
reports
health
conformance state
capacity
open ghosts
```

Projection is never truth.

### 12.9 Packs

Packs add practice without changing core.

A pack may define:

```txt
study benches
conventions
examples
adapters
projectors
runtime hooks
acceptance tests
teaching flows
```

Packs do not mutate canon.

### 12.10 Apps, MCP, and models

Apps, MCP tools, and models participate by drafting, reading, proposing, invoking allowed routes, and submitting evidence under grants.

They do not own truth.

A model can help write candidates. It cannot become evidence by itself.

### 12.11 Runtimes and workers

Workers execute allowed work and return reports/evidence.

Workers do not close receipts.

Workers do not define truth.

### 12.12 Deployments/live Labs

Deployments place the generic Lab Kit in the real world.

Examples:

```txt
personal offline Lab
Santo Andre practice Lab
Manhattan proof Lab
course Lab
company internal Lab
machine-control Lab
research Lab
```

Each deployment is a Lab instance or practice context. None becomes universal meaning.

---

## 13. Final repository tree

The repository remains one package.

```txt
logline-lab-kit/
  README.md
  Cargo.toml
  build-pack/
    README.md
    package.manifest.yaml
    final-real-project-doc.md

  docs/
    PROJECT_DEFINITION.md
    ACT_CANON.md
    LAB_FORMATION.md
    PROTOCOL_STRATEGY.md
    SCIENTIFIC_METHOD.md
    HUMAN_EXPERIENCE.md
    TIME_AND_RULER.md
    PACKS_PROFILES_APPS.md
    CONFORMANCE.md
    MANHATTAN_L06.md
    RELEASE.md

  crates/
    logline-act/
    logline-lab-core/
    logline-lab-local/
    logline-lab-spine/
    logline-lab-projectors/
    logline-lab-clock/
    logline-lab-ruler/
    logline-lab-hooks/
    logline-lab-dispatch/
    logline-lab-reports/
    logline-lab-conformance/
    logline-lab-cli/
    logline-lab-labd/

  schemas/
    act.schema.json
    lab-manifest.schema.json
    pack-manifest.schema.json
    profile.schema.json
    study-bench.schema.json
    receipt-candidate.schema.json
    ghost.schema.json
    conformance-vector.schema.json
    ruler-report.schema.json
    learning-report.schema.json

  conventions/
    act-kinds.yaml
    research.conventions.yaml
    proof.conventions.yaml
    schedule.conventions.yaml
    receipt.conventions.yaml
    ghost.conventions.yaml
    conformance.conventions.yaml

  packs/
    demo/
    santo-andre/
    manhattan/
    course-starter/

  profiles/
    local-only/
    supabase-default/
    postgres-default/
    personal-offline/

  benches/
    candidate-promotion/
    proof-discipline/
    schedule-due-work/
    manhattan-l06/
    conformance/

  experience/
    start/
    today/
    timeline/
    write/
    schedule/
    workbench/
    proof/
    learn/
    settings/

  apps/
    mcp-server/
    model-middleware/
    chatgpt-bridge/

  runtimes/
    hermes/
    openclaw/
    local-worker/

  deployments/
    local-dev/
    santo-andre/
    manhattan/

  tests/
    act/
    lab-formation/
    protocol-conformance/
    pack-profile-app/
    study-benches/
    clock-ruler/
    proof-receipt-ghost/
    manhattan-l06/

  install/
    macos/
    linux/
    docker/

  release/
    checks/
    examples/
    conformance-vectors/
```

---

## 14. Core Rust crates

### 14.1 `logline-act`

Owns:

```txt
Act struct
slot validation
canonical serialization
hashing
signature hooks
```

### 14.2 `logline-lab-core`

Owns:

```txt
Lab manifest
pack/profile loading interfaces
candidate/admission workflow
core errors
core policy hooks
```

### 14.3 `logline-lab-local`

Owns:

```txt
local outbox
local cache
file paths
offline behavior
```

### 14.4 `logline-lab-spine`

Owns:

```txt
spine contract
sync protocol
profile-independent truth boundary
```

### 14.5 `logline-lab-projectors`

Owns read projections:

```txt
Today
Timeline
Proof
Learn
reports
conformance status
open ghosts
```

### 14.6 `logline-lab-clock`

Owns:

```txt
time source
tick generation
schedule scanning hooks
```

### 14.7 `logline-lab-ruler`

Owns:

```txt
due work detection
overdue detection
blocked/reschedule proposals
capacity reports
next study suggestions
```

It is a necessary component, not the identity of the project.

### 14.8 `logline-lab-dispatch`

Owns:

```txt
gate calls
workorder creation
worker dispatch
execution report intake
```

### 14.9 `logline-lab-reports`

Owns:

```txt
Lab reports
study reports
conformance reports
proof reports
learning reports
```

### 14.10 `logline-lab-conformance`

Owns:

```txt
reference vectors
pack conformance checks
Lab compatibility reports
examples that other Labs can compare against
```

This crate is important for protocol strategy.

### 14.11 `logline-lab-labd`

Owns the resident host binary.

It loads the Lab and keeps it alive.

---

## 15. Packs and profiles

### 15.1 Packs

Packs are practice.

They may define:

```txt
teaching material
study benches
examples
conventions
adapters
projectors
runtime hooks
acceptance tests
```

Packs are not canon.

### 15.2 Santo Andre pack

Santo Andre is Dan's recommended/default practice pack.

It may carry strong social weight because Dan invented and uses the system.

But it is not official canon.

Other Labs may use it, fork it, ignore it, or learn from it.

### 15.3 Manhattan pack

Manhattan is a proof pack.

It demonstrates that the Lab can carry a serious physical/network obligation through Act-based study and proof.

It is not the product.

### 15.4 Profiles

Profiles decide operational placement.

A profile may configure:

```txt
storage
sync
local paths
runtime access
clock policy
MCP/app grants
secrets boundary
```

Profile does not define LogLine meaning.

---

## 16. Protocol strategy

The project must make protocol adoption easier than platform captivity.

### 16.1 Protocol surface

LogLine-as-protocol requires:

```txt
canonical Act shape
canonical serialization
hashing/signature expectations
conformance vectors
reference examples
portable packs
portable profiles where possible
clear extension rules
clear non-authority rules
independent implementation possibility
```

### 16.2 Conformance before community mythology

The project should avoid vague claims like:

```txt
LogLine is a movement.
LogLine is a platform.
LogLine is a company.
```

The better claim is operational:

```txt
Here is the Act shape.
Here are the vectors.
Here is a Lab.
Here is a pack.
Here is a study bench.
Here is a proof.
Here is how another Lab can compare behavior.
```

### 16.3 Labs as distribution

Every successful Lab is a distribution unit for the protocol.

A course Lab teaches LogLine.

A research Lab tests LogLine.

A personal Lab lives with LogLine.

A company Lab applies LogLine.

A machine Lab proves LogLine against real infrastructure.

The shared unit across all of them is not the company account.

It is the Act.

### 16.4 No central service requirement

A hosted service may exist later.

But a Lab must be able to exist without requiring one central hosted authority.

Minimum viable independence:

```txt
install locally
load canon/reference
write and validate Acts
run conformance tests
run at least one study bench
produce reports
export comparable examples
```

---

## 17. Apps, models, and AI

Apps are participation surfaces.

Models are candidate generators, critics, translators, and helpers.

They are not authorities.

### 17.1 Model role

Models may:

```txt
translate human language into candidate Acts
suggest missing slots
compare Acts against conventions
summarize Lab history
propose study questions
propose next actions
criticize evidence gaps
```

Models may not:

```txt
count as evidence by themselves
close receipts by themselves
silently promote candidates
rewrite canon through convenience
```

### 17.2 MCP role

MCP is a good participation surface.

MCP tools can expose:

```txt
write candidate Act
inspect Today
inspect Timeline
run conformance
submit evidence ref
run allowed study bench
request report
```

MCP does not become the authority backbone.

The authority remains Act discipline plus Lab runtime policy.

---

## 18. Manhattan L-06 proof

Manhattan L-06 remains the serious proof fixture.

It proves that Lab Kit can:

```txt
represent a real future obligation as Acts
carry that obligation until due
route it through Lab policy
execute allowed real-world work
capture evidence
prepare scoped receipt
leave ghosts for missing proof
report and learn
```

The reason L-06 matters is not that every Lab is Manhattan.

The reason it matters is that protocol claims become serious when they survive real work.

L-06 is one proof of seriousness.

It should live as pack/proof material, not as the definition of the product.

---

## 19. Build order

### Phase 0 - Freeze authority

```txt
keep project name
preserve current package boundary
mark this document as controlling definition
remove stale primitive/object authority
remove company/workspace framing from core docs
```

### Phase 1 - Act core

```txt
implement Act type
validate nine slots
canonicalize
hash
load conformance vectors
```

### Phase 2 - Lab formation loop

```txt
Lab manifest
profile selection
pack loading
first candidate Act
first admitted Act
first report
```

### Phase 3 - Protocol conformance

```txt
reference vectors
conformance runner
conformance report
exportable examples
```

### Phase 4 - Study benches

```txt
study bench schema
candidate-promotion bench
proof-discipline bench
first learning report
```

### Phase 5 - Clock/ruler infrastructure

```txt
ticks
schedules
due work
overdue work
blocked/reschedule Acts
capacity reports
```

### Phase 6 - Experience surfaces

```txt
Start
Today
Timeline
Write
Schedule
Workbench
Proof
Learn
Settings
```

### Phase 7 - Gate, workers, evidence, receipts

```txt
gate policy
workorders
worker reports
evidence refs
receipt candidates
ghosts
```

### Phase 8 - Packs and deployments

```txt
demo pack
Santo Andre pack
Manhattan pack
local-only profile
Supabase/Postgres profile if chosen
```

### Phase 9 - Apps/MCP/models

```txt
MCP server
model middleware
candidate writing tools
report tools
bench invocation tools
```

### Phase 10 - Release

```txt
install scripts
examples
conformance vectors
first-session guide
Lab formation guide
```

---

## 20. Acceptance tests

### 20.1 Act/canon acceptance

```txt
A01  Act has exactly nine canonical slots.
A02  Unknown semantic domains do not become native authority.
A03  Candidate Acts can be ugly.
A04  Promotion requires validation.
A05  Hash/canonicalization is stable.
```

### 20.2 Lab formation acceptance

```txt
A06  User can initialize a Lab.
A07  Lab manifest loads.
A08  Profile loads.
A09  Pack loads without mutating core.
A10  First candidate Act is created.
A11  First admitted Act is stored.
A12  First Lab report renders.
```

### 20.3 Protocol acceptance

```txt
A13  Conformance vectors run.
A14  Conformance report is generated.
A15  Example Acts can be exported.
A16  Another Lab can compare against the examples.
A17  No central hosted service is required for minimum Lab operation.
```

### 20.4 Scientific method acceptance

```txt
A18  Study bench declares question/hypothesis/probe/evidence/receipt scope.
A19  Bench produces Acts.
A20  Observation becomes evidence or ghost.
A21  Learning report proposes next Act.
```

### 20.5 Time/ruler acceptance

```txt
A22  Future Act can be scheduled.
A23  Tick/check discovers due work.
A24  Due work is executed, blocked, or rescheduled visibly.
A25  No due Act is skipped silently.
A26  Capacity report surfaces harmful idleness without fake busywork.
```

### 20.6 Proof acceptance

```txt
A27  Claim is separate from evidence.
A28  Model text alone is not evidence.
A29  Worker returns evidence/report, not closure.
A30  Receipt candidate closes only declared scope.
A31  Missing proof creates ghost.
```

### 20.7 Experience acceptance

```txt
A32  Start initializes or inspects a Lab.
A33  Today shows due/overdue/blocked/running/recent state.
A34  Timeline shows past/present/future Acts.
A35  Write captures ugly candidate Acts.
A36  Schedule places future Acts.
A37  Workbench runs a study bench.
A38  Proof separates claim/evidence/receipt/ghost.
A39  Learn summarizes closed, failed, and ghosted work.
A40  Settings configures without bypassing authority.
```

### 20.8 Pack/profile/app acceptance

```txt
A41  Santo Andre pack loads as recommended practice, not canon.
A42  Manhattan pack loads as proof material, not product identity.
A43  App/MCP call becomes candidate/proposal/evidence submission under grants.
A44  Unauthorized app action is blocked.
```

### 20.9 Manhattan acceptance

```txt
A45  Manhattan L-06 exists as Acts in the Manhattan pack.
A46  L-06 can be scheduled as future obligation.
A47  Lab reaches L-06 when due.
A48  Gate admits or blocks L-06 with explicit reason.
A49  Worker runs real ping over Ethernet when allowed.
A50  Evidence captures ping result.
A51  Receipt candidate closes only L-06.
A52  Report records what changed and what remains ghosted.
```

---

## 21. Done definition

v0 is done when:

```txt
A user installs LogLine Lab Kit.
The user initializes a Lab.
The Lab loads canon/reference material.
The Lab loads a profile.
The Lab loads a pack.
The user writes a candidate Act.
The candidate validates as exactly nine canonical slots or remains visibly imperfect.
The candidate can be admitted after validation.
The Act is canonicalized and hashed.
The Act is stored locally.
The Act syncs when a spine/profile is configured.
The Act is visible through projections.
The Lab runs conformance vectors.
The Lab runs at least one study bench.
The Lab produces evidence or ghost for study work.
The Lab produces a learning report.
The Lab schedules a future Act.
The Lab reaches due work through explicit clock/ruler behavior.
The due work resolves, blocks, or reschedules visibly.
One worker dry-run runs.
One real worker command runs in an allowed proof fixture.
One MCP/app route is registered.
One unauthorized route is rejected.
The demo pack completes first session.
The Santo Andre pack loads without becoming canon.
The Manhattan pack loads without becoming product identity.
Manhattan L-06 proves a serious future obligation path.
The Lab can export examples/conformance output useful to another Lab.
```

---

## 22. Fatal wrong builds

The build is wrong if it does any of the following:

```txt
renames the project
creates a second root
turns temporal machinery into the product identity
turns Manhattan into the product
turns Santo Andre into universal canon
turns the company into semantic authority
requires a central hosted service for basic Lab existence
adds native semantic domains outside Acts
introduces an authoritative file/blob layer outside Acts
lets projection become truth
lets apps write truth directly
lets workers govern
lets model output count as evidence
lets reports count as receipts
builds a passive API and calls it a Lab
uses cron invisibly without visible tick/check behavior
allows due Acts to be skipped silently
implements generic dashboard without study/proof behavior
turns packs into core mutations
turns profiles into meaning
keeps stale primitive/object language as authority
```

---

## 23. Ghosts and open decisions

Known ghosts that must remain visible until proven:

```txt
final lab manifest schema
final pack manifest schema
final profile manifest schema
conformance vector schema
receipt candidate schema
blocked Act projection schema
ruler report schema
study bench schema
learning report schema
exact Rust CLI vs TypeScript/oClif boundary
Hermes vs OpenClaw primary runtime
Supabase/Postgres/local profile defaults
MCP server package boundary
model middleware package boundary
pitwall adapter boundary
cockpit adapter boundary
Manhattan runtime install status
LAB_8GB package installation
Manhattan L-06 receipt
capacity band formula
standing work policy
first release distribution channel
```

Rule:

```txt
A ghost is not failure.
A ghost is named missing proof.
```

---

## 24. Source material policy

### 24.1 Preserve

Preserve:

```txt
current build-pack
ATLAS/MAP material
Foundation/reference material
conformance vectors
working code
recovery notes
receipts
evidence
current lean package skeleton
Lab Kit docs aligned with Act-centered semantics
Manhattan docs as pack/proof material
Santo Andre docs as recommended practice material
```

### 24.2 Demote or remove

Demote or remove:

```txt
old transcripts as authority
assistant-generated authority not grounded in current package
stale architecture drafts
duplicated docs
old primitive framing
native object/domain language that competes with Act
company-as-center language
platform-captivity language
file/SQLite truth language when a spine is configured
```

Do not preserve disposable confusion under prestigious labels.

Preserve only material that teaches, proves, or supports recovery. Remove material that will confuse future builders.

If temporary isolation is needed, it must have:

```txt
owner
reason
expiry/review date
allowed actions
automatic transition to remove/delete/escalate
```

No permanent limbo.

---

## 25. Final project map

```txt
Foundation/reference
  supplies grammar, examples, vectors, and compatibility expectations.

LogLine Lab Kit
  supplies installable machinery for forming Labs.

Lab
  is the institution where LogLine is studied through Acts.

Act
  is the semantic unit.

Conformance
  makes protocol compatibility visible.

Pack
  adds local practice, teaching, examples, benches, and adapters.

Profile
  chooses operational infrastructure.

labd
  is the resident Lab host.

engine + constitutional-runtime
  are the canonical runtime brain.

clock/ruler
  keep obligations and study work from being silently skipped.

projectors
  make Acts readable as Today, Timeline, Proof, Learn, reports, and health.

apps/MCP/models
  participate by drafting, reading, proposing, and submitting evidence under grants.

workers/runtimes
  execute allowed work and return evidence.

deployments
  place independent Labs in the world.

Manhattan L-06
  proves one serious real-world obligation path.
```

---

## 26. One-page version

```txt
LogLine Lab Kit is one project: logline-lab-kit.

It is not renamed.
It is not a passive API.
It is not mainly a temporal product.
It is not Manhattan, Santo Andre, Minilab, Supabase, Cloudflare, Pitwall, or Cockpit.
It is not a company workspace or SaaS captivity system.

It is an installable kit for forming Labs that study, practice, test, prove,
and transmit LogLine through LogLine Acts.

The public goal is to help people instantiate real Labs.
The strategic goal is to make LogLine behave as protocol, not company.

Act remains the canonical semantic unit with exactly nine slots.
Everything else is envelope, convention, projection, pack, profile, app, runtime, or deployment.

A Lab loads canon/reference material, packs, and profiles.
A Lab writes candidate Acts, admits valid Acts, runs study benches, schedules future work,
collects evidence, prepares scoped receipts, preserves ghosts, reports, learns, and runs conformance.

Time remains important because real Labs have future obligations, due work, retries, capacity,
and learning over history. But time is infrastructure for study. It is not the whole identity.

The human experience is:
Start, Today, Timeline, Write, Schedule, Workbench, Proof, Learn, Settings.

The repository remains one package with core crates, schemas, conventions, profiles, packs,
projectors, benches, experience surfaces, apps, runtimes, deployments, install/release,
and conformance examples.

v0 is done only when a real Lab can be installed, load reference material, run conformance,
write and admit Acts, run a study bench, schedule and reach future work, execute allowed proof work,
attach evidence, prepare scoped receipts, preserve ghosts, learn next actions, and export examples
that another Lab can compare against.
```

---

## 27. Final controlling sentence

```txt
Build the Lab formation kit, not a company product.
Keep Act as the only semantic unit.
Make Labs the adoption unit.
Make conformance the proof of protocol.
Use time, proof, science, and experience to help Labs study LogLine seriously.
```
