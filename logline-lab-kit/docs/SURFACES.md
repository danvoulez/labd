# Surfaces — the headless contract

The nine experience surfaces are **headless-first**. Each is a library function on
the Lab host that returns a stable, versioned JSON read-model (a `kind` tag marks
the contract). The `labkit` CLI, an MCP client, and any future GUI/TUI/ChatGPT-App
consume the **same** structs — so a human and an LLM open the same Lab and look at
the same reality: the same Acts, candidates, evidence, ghosts, receipts, schedule,
blocked/overdue/due obligations, experiments, proof state, learning, uncertainty.

This is not an app. It is the set of contracts that make apps possible.

## The Lab on disk

A Lab is a directory (`--store <dir>`) holding:

```
outbox.jsonl       admitted/scheduled Acts (durable; spine rehydrates from it)
evidence.jsonl     captured evidence
ghosts.jsonl       named missing proof
candidates.jsonl   ugly candidates preserved by Write
```

Open the same directory from the CLI or from an MCP client and you get the same
state. Nothing here is "truth" — it is a resumable cache around the Acts.

## Division of labor

- **Humans** speak natural language and carry consequences: authorize, confirm,
  reject, retry.
- **LLMs** translate, route, compare, criticize, explain, warn, suggest — they
  never decide. Model text is never evidence.
- **Automation** handles repetition, scheduling, validation, observation,
  execution, continuity, boring correctness.
- **LogLine Acts** are the shared substrate.

## The nine surfaces

| Surface | Purpose | CLI | JSON `kind` | MCP read tool |
|---|---|---|---|---|
| Start | open a Lab; profile/pack; doctor; conformance; first next actions | `labkit start` | `logline.view.start.v0` | `read:start` |
| Today | due / overdue / blocked / running / recent / ghosts / capacity | `labkit today` | `logline.view.today.v0` | `read:today` |
| Timeline | past / present / future Acts | `labkit timeline` | `logline.view.timeline.v0` | `read:timeline` |
| Write | capture candidates (ugly allowed); admit valid Acts | `labkit write` | `WriteOutcome` (`outcome`) | (draft via `handle`) |
| Schedule | scheduled / due / overdue / blocked; place future obligations | `labkit schedule` | `logline.view.schedule.v0` | `read:schedule` |
| Workbench | run a study bench; observe → evidence or ghost | `labkit workbench` | `logline.view.workbench.v0` | (action) |
| Proof | claim / evidence / receipt / ghost, kept separate | `labkit proof` | `logline.view.proof.v0` | (read via proof) |
| Learn | learning report; proposes the next Act | `labkit learn` | `logline.learning_report.v0` | `read:learn` |
| Settings | inspect config; authority always locked | `labkit settings` | `logline.view.settings.v0` | `read:settings` |
| Tick | confront time; emit tick/disposition/reschedule Acts | `labkit tick` | `logline.ruler_report.v0` | (action) |
| Storage | onboarding matrix: where admitted Acts register | `labkit storage` | `[SpineOption]` | (read) |

Start, Today, Doctor report the storage **grade** (`candidate-only` /
`dev-ephemeral` / `publication`) and warn when a Lab is not publication-grade
(see [`STORAGE.md`](STORAGE.md)). `Tick` is the only surface that *materializes
time as Acts* — no due Act is skipped silently.

Read surfaces are exposed over MCP under read grants (`McpServer::grant_read`);
the returned JSON is byte-identical to the CLI/library output (tested in
`apps/mcp-server`: `human_and_llm_see_the_same_map`). Write/Schedule/Workbench are
*actions*: an app call becomes a draft Act for the Lab to admit (A43), and an
unauthorized action is blocked (A44).

## Guarantees

- Act stays exactly nine slots; capture generous, promotion strict.
- No artifact category; no file/SQLite-as-truth.
- Packs/profiles load as data and never mutate core.
- Workers return evidence, not closure; receipts close only what is proven;
  ghosts are named, never silently closed.
- Settings never bypass Act discipline, proof discipline, or gate policy.

These properties make retry natural, mistakes visible, uncertainty explicit,
experiments repeatable, proof inspectable, and learning cumulative.
