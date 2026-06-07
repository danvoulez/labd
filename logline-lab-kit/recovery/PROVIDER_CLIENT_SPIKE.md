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

**Not yet verified:** a **live round-trip** — Ollama is not installed on this machine, so the
call failed at connection (the *correct* failure, proving local targeting). Final acceptance
at step D requires a real round-trip against a running Ollama.

**Concern to weigh:** footprint — genai pulls **~269 transitive crates** (multi-provider
HTTP + TLS + async). Consistent with the already-accepted `tokio`/`reqwest`/`rustls` stack,
but larger than `async-openai` alone. `cargo-deny`/`cargo-audit` (P4) must police it.

**Minor:** `ChatResponse::content_text_as_str` is deprecated → use `.first_text()` / `.texts()`.

## Recommendation

Adopt **`genai` 0.6.5** behind the `ProviderAdapter` trait at step D — one vetted dependency
covers Ollama (local/dev first), Anthropic, and OpenAI with rustls + permissive license and
no agent-framework baggage. Gate final acceptance on a live Ollama round-trip. Keep
`async-openai` as the documented fallback if footprint or the beta track becomes a problem.
