//! The provider adapter contract — provider-NEUTRAL, no admission authority.
//!
//! A [`ProviderAdapter`] lets an optional model participate: draft, propose, criticize,
//! translate, summarize. It can produce **model-drafted candidates** and read context; it
//! can **never** admit, confirm, or decide. Concrete adapters (Ollama, Anthropic, OpenAI,
//! …) are bought behind this trait in a later step — and an agent framework / external
//! loop / memory model / tool planner is explicitly NOT imported here.
//!
//! Methods are `async` because real adapters do network I/O; this crate adds no runtime
//! and ships no adapter (step B is the contract only).

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::SessionActCandidate;

/// Read-only context handed to a provider: snapshots of surfaces the provider is granted
/// to read, plus recent transcript text. Never includes admission capability.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProviderContext {
    /// `(surface_name, surface_json)` for granted read surfaces (the same JSON a human sees).
    pub surfaces: Vec<(String, Value)>,
    /// Recent transcript lines (projection/cache).
    pub transcript: Vec<String>,
}

/// A provider's response to user input: free text plus an optional model-drafted candidate.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderTurn {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub candidate: Option<SessionActCandidate>,
}

/// A provider's critique of an Act/candidate: gaps it sees, and notes. Advisory only —
/// a critique is not proof and not confirmation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Critique {
    pub gaps: Vec<String>,
    pub notes: String,
}

/// A provider call failed. Kept simple and `std::error::Error`-compatible so Lab reports
/// can surface it without a heavy dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderError(pub String);

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "provider error: {}", self.0)
    }
}

impl std::error::Error for ProviderError {}

/// The provider-neutral adapter. No method admits, confirms, or decides; the strongest
/// thing a provider can do is hand back a **model-drafted candidate** (provenance =
/// model), which the Lab may later admit.
#[allow(async_fn_in_trait)]
pub trait ProviderAdapter {
    /// Assemble read-only context from granted surfaces + transcript.
    async fn prepare_context(&self) -> Result<ProviderContext, ProviderError>;

    /// Turn human input into a provider turn (text, optional drafted candidate).
    async fn receive_user_input(
        &self,
        input: &str,
        ctx: &ProviderContext,
    ) -> Result<ProviderTurn, ProviderError>;

    /// Draft a candidate Act (model provenance). DRAFT ONLY — never admitted, never confirmed.
    async fn produce_candidate(
        &self,
        ctx: &ProviderContext,
    ) -> Result<SessionActCandidate, ProviderError>;

    /// Critique an Act/candidate's evidence gaps. Advisory; not proof, not confirmation.
    async fn critique_evidence(
        &self,
        act: &Value,
        ctx: &ProviderContext,
    ) -> Result<Critique, ProviderError>;

    /// Suggest a next Act as a model-drafted candidate. DRAFT ONLY.
    async fn suggest_next_act(
        &self,
        ctx: &ProviderContext,
    ) -> Result<SessionActCandidate, ProviderError>;

    /// Summarize a surface's JSON read-model in prose. Read projection; produces no Act.
    async fn summarize_surface(&self, surface_json: &Value) -> Result<String, ProviderError>;
}
