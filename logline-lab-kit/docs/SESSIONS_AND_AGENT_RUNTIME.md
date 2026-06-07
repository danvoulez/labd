# Sessions, Session Runtime, and Provider Adapters

> **Status:** design (step A of the session sequence). Contracts here are the spec the
> implementation steps (B–J) realize. Nothing here grants new semantic authority.
>
> **Naming:** there is no "the LogLine agent." The kit provides a **session runtime** —
> a generic substrate in which a human, an optional model provider, MCP clients, tools,
> and the Lab host participate under one authority model. Use: *session runtime, resident
> session, model provider adapter, remote MCP participation, Lab co-scientist mode*.
> Agents **participate in** the Lab; they do not **become** the Lab.

---

## 0. Foundational correction — everything is an Act

Everything semantically important is a **LogLine Act**. "Receipt", "ghost", "evidence",
"schedule", "session", "message", "learning", "proof", "bench", and "workorder" are
**not** separate truth objects — they are nicknames, conventions, roles, projections, or
experience views over the raw graph of Acts. A Lab may project Acts as Proof, Learn,
Timeline, Session, Chat, Receipt, Ghost, Workbench, Schedule, Evidence, or Culture
Ledger. Projections improve the experience of reading and operating the graph. They do
**not** create new semantic authority. The unit remains the LogLine Act.

A **session** is one more projection + runtime surface over Acts. It is not a new
authority.

---

## 1. Provider-agnostic principle

The Lab Kit stays provider-agnostic. It makes session and runtime possibilities
*possible*; it does not become one opinionated agent product. There is no "LogLine agent"
identity in core. Instead there is a generic session substrate where participants operate
under the same authority model:

- **Lab** owns state.
- **Act** is the semantic unit.
- **Projections** are readable surfaces.
- **LLMs** draft, propose, criticize, translate. They never decide.
- **Apps / MCP clients** participate under grants.
- **Workers** execute and return evidence, not closure.
- **Humans** authorize and carry consequence.
- Nothing outside the Lab silently becomes truth.

**Sessions are valid without a provider.** A human opens a resident session with no model
attached and works normally. Provider attachment is **optional** — for offline use,
privacy, deterministic testing, and so that no LLM provider becomes core meaning in
practice, not just in theory.

> A Lab session is **not** "chat completions plus tools." It is:
> **human + Lab + Act graph + surfaces + optional model provider + optional tools.**
> A provider can enhance the session; it must not define it.

---

## 2. The three modes — do not collapse them

| | Mode 1 — Resident Terminal | Mode 2 — Remote MCP Client | Mode 3 — Shared UI |
|---|---|---|---|
| **What** | Shared Lab presence in the terminal | Remote Lab participation from an external client | Shared Lab presence in a richer Lab-owned interface |
| **Human is** | At the terminal, watching the same Lab state as the model | Inside ChatGPT/Claude/another MCP client | Inside the Lab's own web shell |
| **Lab is** | Resident host (`labd` keeps the Lab open) | A server | A server with an event stream |
| **Model** | Attached via Settings; drafts/translates/criticizes/proposes/explains | External; calls tools and narrates Lab state back | Optional participant alongside the human |
| **Risk model** | Local | Grants, auth, session tokens, tunnel/deploy profile, audit, revocation, envelope discipline, human-confirmation boundaries | Same authority model; richer presence |

**Controlling sentence:** Resident session is shared Lab presence in the terminal. Shared
UI session is shared Lab presence in a richer Lab-owned interface. Remote MCP session is
remote Lab participation from an external client.

Do **not** describe Mode 2 as "just connecting the same experience" — narration from an
external LLM UI is not shared Lab presence.

**Mode 3 does not invent new surfaces.** The stable JSON surfaces already exist —
`start, today, timeline, write, schedule, workbench, proof, learn, settings, tick,
storage` (each `logline.view.*.v0`, see [`SURFACES.md`](SURFACES.md)). Mode 3 needs a
**client connection protocol over those surfaces + an event-stream subscription** — not a
redefinition of the surfaces. The UI consumes the same contracts the CLI, resident
session, and MCP boundary consume. That is how human and LLM see the same Lab for real.

The generic kit provides for Mode 3: surface read contracts, session event stream,
candidate submission, human approval boundaries, provider adapter hooks, grants, evidence
reference submission, projection refresh. It must **not** create a UI-specific truth layer.

---

## 3. Session as runtime, not semantic authority

A session is a convention, projection, and runtime surface over Acts. A session event that
matters must be expressible as an Act. A chat message that matters is captured as a
candidate or admitted Act. A UI action that matters becomes an Act or candidate. A model
suggestion that matters becomes a candidate Act. A human confirmation that matters becomes
an Act.

- The UI **reads** projections.
- The UI **writes** candidates.
- The Lab **admits** Acts.
- The Lab remains the authority boundary.

**Resumability derives from Acts, not from a separate session-truth store.** If the
terminal closes, the UI disconnects, or the server restarts, session state is recoverable
from the Lab's Act graph and projections. Sessions may have caches, transcripts, UI state,
and transport buffers — but never a parallel durable session store that becomes truth.
Continuity comes from reading the Lab's own surfaces and Acts.

---

## 4. No silent admission — a hard, non-configurable boundary

**A session cannot admit an Act.** Only the Lab admits Acts, and only after promotion
discipline passes. The session produces candidates. The human may trigger or authorize
promotion. The Lab performs admission.

`session.approve()` may create an approval/authorization Act or candidate. It must **not**
bypass Lab admission. This boundary is not configurable. (This is the existing app-boundary
rule: an app/MCP/model call becomes a *draft* Act for the Lab to admit — see
[`crates/logline-lab-core/src/app.rs`](../crates/logline-lab-core/src/app.rs) and
[`apps/mcp-server`](../apps/mcp-server). The session substrate reuses it; it does not widen it.)

---

## 5. Attribution and provenance

Every candidate and Act produced during a session carries explicit attribution. The
session distinguishes:

- **who drafted/proposed** something,
- **who confirmed/authorized** it,
- **what proof, witness, rule, signature, receipt, or validator** supports it.

A model-generated candidate carries model provenance. A human confirmation carries human
identity. **Model output alone never satisfies `confirmed_by`** — that slot is about
confirmation, witness, proof, validator, signature, or *acknowledged absence of proof*,
not mere authorship. If the model drafted it, record that through the session/candidate
convention. If the human confirmed it, that is explicit. If proof is missing, keep it as
doubt or ghost — not confirmation.

(Forward link: the ed25519 **witness profile** in P1 makes `confirmed_by` a verifiable
cryptographic witness over the canonical tuple. A session's human confirmation can later
attach such a witness. The transport **Envelope** — already in `logline-act` — wraps
session payloads crossing a boundary; `envelope_hash` is sender-computed, receiver-verified,
and never enters Act content.)

---

## 6. Session substrate types (contracts)

Runtime, projection, and convention types — **not** new semantic authorities. Sketch
(final field sets land in step B+):

```rust
/// A live session over a Lab. Holds no truth: state is derived from the Lab's Acts +
/// projections; everything below is cache/convention/runtime.
pub struct Session { /* id, lab ref, participants, optional provider, grants, transcript */ }

pub enum Participant { Human(HumanId), Model(ProviderId), App(AppId), Worker(WorkerId), LabHost }

pub struct ToolGrant { /* participant, capability (read:today, draft_*, run_bench, submit_evidence...), scope */ }

/// A session event — ALWAYS either an Act or a projection over Acts. Never a parallel
/// authority log. (opened, participant joined, surface viewed, candidate drafted, Act
/// admitted, evidence attached, ghost opened, receipt projection prepared, tick, learning
/// suggested, closed.)
pub enum SessionEvent { /* ... */ }

/// A candidate produced in-session (model, human, app, or tool authored). Carries
/// attribution; promotion/admission is the Lab's, never the session's.
pub struct SessionActCandidate { /* raw Act value, drafted_by, provenance, supporting refs */ }

pub struct SessionEvidenceRef { /* points at evidence; never embeds it in Act content */ }

pub struct SessionTranscript { /* cache/projection only; rebuildable from Acts */ }
```

`session.approve()` → produces an approval/authorization candidate or Act; **does not
admit** (see §4).

---

## 7. Provider adapter trait

Provider-agnostic; **carries no admission authority**:

```rust
pub trait ProviderAdapter {
    fn prepare_context(&self, session: &Session) -> ProviderContext;
    fn receive_user_input(&self, input: &str, ctx: &ProviderContext) -> ProviderTurn;
    fn produce_candidate(&self, ctx: &ProviderContext) -> SessionActCandidate; // draft only
    fn critique_evidence(&self, act: &Act, ctx: &ProviderContext) -> Critique;
    fn suggest_next_act(&self, ctx: &ProviderContext) -> SessionActCandidate;  // draft only
    fn summarize_surface(&self, surface_json: &serde_json::Value) -> String;   // read projection
}
```

Settings supports adapters: **Ollama, Anthropic, OpenAI, local model, future providers**.
No provider becomes core meaning. OpenAI-compatible chat-completion APIs are one adapter
*shape*, not the Lab session model. Do not hardcode any provider into core.

---

## 8. Settings — provider registry & secrets

The current Settings surface is **read-only** (`logline.view.settings.v0`; "authority is
always locked"). The provider registry is **runtime config, not core semantics** — adding
a provider must never mutate LogLine core. New commands:

```
labkit settings providers list
labkit settings providers add ollama    --url ...
labkit settings providers add anthropic --api-key-env ANTHROPIC_API_KEY
labkit settings providers add openai     --api-key-env OPENAI_API_KEY
labkit settings providers set-default ...
labkit settings providers test ...
```

Secrets must not be stored casually in plain config. Prefer env vars; OS keychain later;
explicit local-dev mode otherwise.

---

## 9. Resident session — CLI (Mode 1)

Implemented provider-free (step C). All operate on a `--store`-backed Lab so state resumes
across runs from the Lab's Acts:

```
labkit session start        # open/resume a resident session; show Start
labkit session view --surface today   # read a Lab read-surface (start/today/timeline/
                            #   schedule/learn/settings/storage)
labkit session write --text "…"       # capture human text as a candidate (NOT admitted)
labkit session write --json file.json # capture a structured candidate (NOT admitted)
labkit session approve --json act.json # mint authorization candidate; the LAB admits target
labkit session tick                    # confront time
labkit session transcript              # projection over the Lab's Acts (rebuilt, not stored)
labkit session close
```

(The old batch-admit command was renamed `labkit session` → **`labkit emit`** to free the
`session` namespace.) Provider-driven commands (`suggest`/`draft`/`attach` a provider) arrive
in step D behind the `ProviderAdapter` trait — **Ollama/local first** because it avoids
hosted-provider assumptions; never required.

---

## 10. Shared UI client contract (Mode 3)

The UI client may:

- read surfaces,
- subscribe to session events,
- submit human text,
- submit model text,
- create candidate Acts,
- request promotion,
- submit evidence references,
- approve or deny work,
- **never write admitted truth directly.**

---

## 11. Session event stream

Events: session opened, participant joined, surface viewed, candidate drafted, Act
admitted, evidence attached, ghost opened, receipt projection prepared, tick occurred,
learning suggested, session closed. Each is **either an Act or a projection over Acts**.
Do not create a parallel authority log.

---

## 12. MCP as remote participation surface (Mode 2)

MCP tools may expose the same capabilities as the resident session, but under explicit
grants: read Start/Today/Timeline/Proof/Learn, draft candidate Act, run conformance,
request workbench run, submit evidence reference. MCP must not receive silent authority.

Settings makes remote exposure explicit, **disabled by default**:

- local-only MCP stdio,
- local-network MCP,
- tunnel MCP,
- public/remote MCP — **off by default**,
- grants required, session audit required, human approval required for dangerous actions.

**Transport is not core.** Cloudflare, Tailscale, ngrok, SSH reverse tunnel, local
network, and hosted relay are profiles/deployments. The generic kit defines the **remote
endpoint contract, the tunnel-profile interface, the security checklist, and grants/audit**
— it does not own the transport.

---

## 13. Authority model (one table)

| Actor | May | May NOT |
|---|---|---|
| Human | authorize, confirm, reject, retry, carry consequence | — |
| Lab host | admit Acts (after promotion), project surfaces, own state | — |
| Model provider | draft, propose, criticize, translate, summarize | admit; satisfy `confirmed_by` by authorship; decide |
| App / MCP client | participate under grants; submit drafts/evidence refs | admit; act ungranted |
| Worker | execute admitted workorders; return evidence | define truth; close scope |
| Session | project, cache, route, capture candidates, request promotion | admit; hold parallel truth |

---

## 14. Generic v1 vs SOON

**In generic v1:** session substrate types; provider adapter trait; Settings provider
registry (Ollama + optional Anthropic/OpenAI via env); resident terminal session over
existing surfaces; transcript as Acts/projections; the Shared-UI client *contract* +
connection surface over existing surfaces; an MCP stdio server (rmcp) exposing session
tools under grants.

**SOON (profiles/deployments, not generic core):** the Shared-UI web app itself;
remote/tunnel transports (Cloudflare/Tailscale/ngrok/SSH/relay); OS keychain secret
storage; hosted relay; multi-client realtime presence beyond the event-stream contract.

---

## 15. Implementation sequence (authoritative)

```
A. Session design contract                    (this document)
B. Provider-neutral session substrate types   (no external provider code yet)
C. No-provider resident session FIRST         (prove session ≠ chat+tools)
D. Provider adapters behind the trait         (evaluate clients by behavior)
E. Settings provider registry + secrets       (env → secrecy → dotenvy → keyring later)
F. Resident provider MVP                       (Ollama/local first; modest, no autonomy)
G. Event stream for Shared UI Session          (consume existing surfaces; no redefinition)
H. Remote MCP later (rmcp)                      (after resident MVP; stronger grants/audit)
I. Security prerequisites before worker autonomy (allowlist, no sh -c, timeouts, dry-run,
   redaction, approval boundary, audit) — design now; provider must NOT call workers directly
```

### Buy vs build boundary (locked)

**Buy (where mature):** provider clients, transports, async runtime, secrets, MCP protocol
machinery. **Do NOT buy** an agent framework, external agent loop, memory model, tool
planner, RAG identity, or orchestration model as the Lab's runtime.

**The Lab Kit's IP (build):** the Act graph; projections; candidate capture; attribution;
grants; no-silent-admission; human-approval boundaries; provider-agnostic model
participation; worker evidence; resumability from Acts; shared surfaces.

### Progress
- ✅ **A** — this design contract.
- ✅ **B** — `logline-lab-session` crate (provider-neutral types + `ProviderAdapter` trait).
- ✅ **Provider spike** — genai 0.6.5 accepted by behavior (live round-trip pending; `recovery/PROVIDER_CLIENT_SPIKE.md`).
- ✅ **C** — provider-free resident session: `ResidentSession` in `logline-lab-labd`, `labkit session start|view|write|approve|tick|transcript|close`, no-provider fixture (`release/examples/no-provider-session.sh`) proving capture/approve/admit/tick/transcript + **resume-after-restart from Acts** (no session-truth store). Wired into the gate.
- ⏳ **D** — attach genai behind the trait (gate on a live Ollama round-trip).

### In scope (remaining)
Settings provider registry (E) · resident provider MVP (F) · event stream contract (G) ·
MCP remote (H) · worker-autonomy security prerequisites (I).

### Not yet
Full daemon · remote tunnel (Cloudflare/Tailscale/ngrok) · Shared UI frontend · MCP remote
server before the resident MVP · autonomous agent loop · external agent framework ·
science-core.

Prerequisite (satisfied): the C3 conformance gate is green and committed — canonical
bytes, hashing, and the transport Envelope are frozen, so session payloads and evidence
references rest on a stable protocol.
