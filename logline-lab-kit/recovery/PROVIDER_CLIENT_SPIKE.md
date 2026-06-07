# Provider client spike — accept by behavior, not by name

> Buy the provider **client** (transport to LLM providers); never buy an agent framework.
> This records the behavioral evaluation that informs step D of the session sequence.

## Candidates

| Crate | Shape | Note |
|---|---|---|
| **`genai`** | one Rust client over Ollama, Anthropic, OpenAI, Gemini, Bedrock, … | lead candidate; resolved **0.6.5 stable** (cargo skips the 0.7.0-beta) |
| `async-openai` | OpenAI-compatible only (reaches Ollama via its OpenAI-compat endpoint) | stable fallback; would need a separate Anthropic path |
| `ollama-rs` | Ollama only | too narrow to be the single seam |
| `rig` / `langchain-rust` / `swiftide` | **agent frameworks** | **rejected on principle** — they import an agent loop / memory / tool planner / RAG identity. Out of bounds. |

## genai 0.6.5 — behavioral results (2026-06-07)

Isolated `/tmp` spike, `Client::exec_chat("llama3.2", ChatRequest, None)`, no API key in env.

| Acceptance criterion | Result |
|---|---|
| Reaches a **local Ollama** with no hosted assumptions | ✅ routed `llama3.2` → `adapter: Ollama` → `http://localhost:11434/api/chat`, **no API key required**, no cloud fallback |
| Produces plain text + structured candidate material | ✅ `ChatRequest`/`ChatResponse`; streaming via `exec_chat_stream` |
| Errors clean enough for Lab reports | ✅ `Web call failed for model 'llama3.2 (adapter: Ollama)'. Cause: Reqwest error: … http://localhost:11434/api/chat` |
| Does **not** import an agent loop / memory / tool orchestration | ✅ modules are `adapter / chat / client / resolver / embed / webc` — no `agent`/`memory`/`planner`/`rag` |
| Can be disabled entirely | ✅ it is just a client behind our `ProviderAdapter` trait |
| TLS backend | ✅ **rustls** (pure-Rust; no OpenSSL/C) |
| License | ✅ MIT OR Apache-2.0 |

**Live round-trip — VERIFIED (2026-06-07):** genai completed a real round-trip against an
OpenAI-compatible endpoint provided by the operator (`https://mistral.minilab.work/v1`,
`Bearer EMPTY`) via a `ServiceTargetResolver` (custom `Endpoint` + `AuthData::from_single`
+ `AdapterKind::OpenAI`). genai connected, authenticated, sent the request, and returned a
response — proving live model behavior through the Rust client, not just connection
targeting. (The code model echoed tool-call-shaped JSON rather than the literal prompt; a
curl baseline against the same endpoint returned `ok`. That is model/prompt behavior, not a
client issue.) **Still not run:** a round-trip against a local *Ollama* specifically (Ollama
not installed here) — genai uses the same client path, and the earlier offline spike already
proved it targets `localhost:11434` with no hosted assumptions.

**Concern to weigh:** footprint — genai pulls **~269 transitive crates** (multi-provider
HTTP + TLS + async). Consistent with the already-accepted `tokio`/`reqwest`/`rustls` stack,
but larger than `async-openai` alone. `cargo-deny`/`cargo-audit` (P4) must police it.

**Minor:** `ChatResponse::content_text_as_str` is deprecated → use `.first_text()` / `.texts()`.

## Recommendation

Adopt **`genai` 0.6.5** behind the `ProviderAdapter` trait at step D — one vetted dependency
covers Ollama (local/dev first), Anthropic, and OpenAI with rustls + permissive license and
no agent-framework baggage. Gate final acceptance on a live Ollama round-trip. Keep
`async-openai` as the documented fallback if footprint or the beta track becomes a problem.

## D status (2026-06-07)

Adopted: `genai` (+`tokio`) live in the **`logline-lab-providers`** crate, behind
`logline_lab_session::ProviderAdapter`. The footprint is contained to that crate (and the
CLI). Live round-trip re-verified end to end via `labkit session suggest` against an
OpenAI-compatible endpoint: the model produced a `provider_suggested_candidate` (captured,
not admitted; `confirmed_by` empty; full provenance). genai is an implementation detail —
the Lab exposes the trait.

## P4 supply-chain TODO (do later, not in D)

The `genai` graph is large (~269 transitive crates). This is acceptable only because it
buys commodity provider plumbing — it must never bring an agent framework into the Lab.
Before exposing real external clients (P5), P4 must enforce:
- `cargo-deny` (bans, licenses, advisories) + `cargo-audit` (RustSec) in CI;
- an explicit **license policy** (current new deps: genai MIT, ryu-js Apache-2.0/BSL — OK);
- an **SBOM** (e.g. `cargo-cyclonedx`);
- a pinned `Cargo.lock` (present) + dependency review.
Until then, the footprint is documented here and must not silently grow.
