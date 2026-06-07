//! `logline-lab-providers` — provider PLUMBING, contained, with **provider truth as Acts**.
//!
//! Buys commodity plumbing (`genai` multi-provider client + `tokio`) hidden behind
//! `logline_lab_session::ProviderAdapter`. genai is an implementation detail; no agent
//! framework / memory / planner enters here.
//!
//! **Source of truth = the Act graph.** A Lab's decision to register / update / disable /
//! set-default / call a provider is accountable history and is emitted as a LogLine Act:
//! `register_provider`, `update_provider`, `disable_provider`, `set_default_provider`,
//! `test_provider`, `provider_call`. The live [`ProviderRegistry`] is a **projection**
//! folded from those Acts ([`ProviderRegistry::project_from_acts`]) — disposable and
//! rebuildable, never an independent truth store. There is no `providers.json` authority.
//!
//! **Secrets never enter Acts.** An Act records the env-var *name* and the (non-secret)
//! endpoint — never the key value.
//!
//! Provider-READY, not provider-complete: one adapter (`openai-compatible`). Native
//! providers are SOON (see [`availability`]).

#![forbid(unsafe_code)]

use genai::adapter::AdapterKind;
use genai::chat::{ChatMessage, ChatRequest};
use genai::resolver::{AuthData, Endpoint, ServiceTargetResolver};
use genai::{Client, ModelIden, ServiceTarget};
use logline_act::Act;
use logline_lab_session::{
    Critique, ModelProvenance, ProviderAdapter, ProviderContext, ProviderError, ProviderTurn,
    SessionActCandidate,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

// ============================================================ availability

/// Provider availability matrix. Only `openai-compatible` is available; the rest are SOON.
/// No provider receives special semantic authority — all participate via `ProviderAdapter`.
pub fn availability() -> Vec<(&'static str, &'static str)> {
    vec![
        ("openai-compatible", "available"),
        ("ollama-native", "soon"),
        ("anthropic", "soon"),
        ("gemini", "soon"),
        ("bedrock", "soon"),
        ("custom-http", "soon"),
    ]
}

// ============================================================ data shapes

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderKind {
    #[serde(rename = "openai-compatible")]
    OpenAiCompatible,
    OllamaNative,
    Anthropic,
    Gemini,
    Bedrock,
    CustomHttp,
}

impl ProviderKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenAiCompatible => "openai-compatible",
            Self::OllamaNative => "ollama-native",
            Self::Anthropic => "anthropic",
            Self::Gemini => "gemini",
            Self::Bedrock => "bedrock",
            Self::CustomHttp => "custom-http",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "openai-compatible" => Some(Self::OpenAiCompatible),
            "ollama-native" => Some(Self::OllamaNative),
            "anthropic" => Some(Self::Anthropic),
            "gemini" => Some(Self::Gemini),
            "bedrock" => Some(Self::Bedrock),
            "custom-http" => Some(Self::CustomHttp),
            _ => None,
        }
    }
    pub fn is_available(&self) -> bool {
        matches!(self, Self::OpenAiCompatible)
    }
}

/// Auth configuration. The API key is read from an env var at use time — its *name* is
/// recorded, the *value* never is.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthConfig {
    /// e.g. `bearer_env` — a bearer token sourced from an environment variable.
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub env: Option<String>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self { kind: "bearer_env".to_string(), env: None }
    }
}

/// A provider profile — the *materialized* shape used by the runtime. This is projection
/// state, folded from Acts; it is never persisted as an independent source of truth.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderProfile {
    pub id: String,
    pub kind: ProviderKind,
    pub base_url: String,
    pub model: String,
    #[serde(default)]
    pub auth: AuthConfig,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub dev_only: bool,
}

fn default_true() -> bool {
    true
}

// ============================================================ provider-decision Acts

// Provider-decision Acts use STRING slots — the canon mold's nature (`logline.receipt.v0`
// slots are strings). The `this` slot is a `key=value; key=value` descriptor, NOT a
// structured object, so these are canon-shape Acts and do not lean on the still-open
// divergence #3 (labd's Value-slots vs the canon's string-slots). Secrets never enter an
// Act — only the env-var NAME and the non-secret endpoint.

/// Render a `key=value; key=value` descriptor for a string `this` slot.
fn descriptor(pairs: &[(&str, String)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Parse a `key=value; key=value` descriptor back into a map (split each pair on the FIRST `=`).
fn parse_descriptor(s: &str) -> std::collections::BTreeMap<String, String> {
    let mut m = std::collections::BTreeMap::new();
    for seg in s.split(';') {
        let seg = seg.trim();
        if let Some((k, v)) = seg.split_once('=') {
            m.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    m
}

fn decision_act(who: &str, did: &str, this: String, if_ok: &str, if_doubt: &str, if_not: &str, now: &str) -> Value {
    json!({
        "who": who,
        "did": did,
        "this": this, // STRING slot (canon mold nature)
        "when": now,
        "confirmed_by": who, // a human operator authorizes the decision (never a model)
        "if_ok": if_ok,
        "if_doubt": if_doubt,
        "if_not": if_not,
        "status": "admitted"
    })
}

/// `register_provider` Act value. Secrets excluded — only the env-var name + endpoint.
pub fn register_provider_act(who: &str, profile: &ProviderProfile, now: &str) -> Value {
    let this = descriptor(&[
        ("provider_id", profile.id.clone()),
        ("kind", profile.kind.as_str().to_string()),
        ("base_url", profile.base_url.clone()),
        ("model", profile.model.clone()),
        ("auth_env", profile.auth.env.clone().unwrap_or_default()),
        ("dev_only", profile.dev_only.to_string()),
    ]);
    decision_act(who, "register_provider", this,
        "provider_available_for_session_candidates", "keep_provider_disabled", "do_not_use_provider", now)
}

/// `set_default_provider` Act value.
pub fn set_default_provider_act(who: &str, provider_id: &str, scope: &str, now: &str) -> Value {
    let this = descriptor(&[("provider_id", provider_id.to_string()), ("scope", scope.to_string())]);
    decision_act(who, "set_default_provider", this,
        "use_as_default_provider_for_suggestions", "ask_provider_explicitly", "no_provider_default", now)
}

/// `disable_provider` Act value.
pub fn disable_provider_act(who: &str, provider_id: &str, now: &str) -> Value {
    let this = descriptor(&[("provider_id", provider_id.to_string())]);
    decision_act(who, "disable_provider", this, "provider_unavailable", "keep_as_is", "provider_unavailable", now)
}

/// `provider_call` Act value — the accountable record that a provider was called. No
/// secrets; carries the same provenance the suggested candidate carries.
pub fn provider_call_act(who: &str, prov: &ModelProvenance, now: &str) -> Value {
    let this = descriptor(&[
        ("provider_id", prov.provider_id.clone()),
        ("kind", prov.provider_kind.clone()),
        ("model", prov.model.clone()),
        ("endpoint", prov.endpoint_label.clone()),
        ("response_hash", prov.response_hash.clone().unwrap_or_default()),
    ]);
    decision_act(who, "provider_call", this, "candidate_captured", "review", "discard", now)
}

/// `test_provider` Act value — records a live-reachability check outcome.
pub fn test_provider_act(who: &str, provider_id: &str, ok: bool, now: &str) -> Value {
    let this = descriptor(&[("provider_id", provider_id.to_string()), ("ok", ok.to_string())]);
    decision_act(who, "test_provider", this, "provider_reachable", "retry", "provider_unreachable", now)
}

// ============================================================ registry projection

/// The provider registry — a **projection** folded from provider-decision Acts. Disposable
/// and rebuildable; never an independent truth store.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRegistry {
    pub providers: Vec<ProviderProfile>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default: Option<String>,
}

fn profile_from_this(this: &Value) -> Option<ProviderProfile> {
    let m = parse_descriptor(this.as_str()?);
    let id = m.get("provider_id")?.clone();
    let kind = ProviderKind::parse(m.get("kind")?)?;
    let base_url = m.get("base_url")?.clone();
    let model = m.get("model")?.clone();
    let env = m.get("auth_env").filter(|s| !s.is_empty()).cloned();
    let dev_only = m.get("dev_only").map(|v| v == "true").unwrap_or(false);
    Some(ProviderProfile {
        id,
        kind,
        base_url,
        model,
        auth: AuthConfig { kind: "bearer_env".to_string(), env },
        enabled: true,
        dev_only,
    })
}

impl ProviderRegistry {
    /// Fold provider-decision Acts (in `when` order) into the live registry. This is the
    /// ONLY source of provider truth — rebuilt from the Lab's Acts every time.
    pub fn project_from_acts(acts: &[Act]) -> Self {
        let mut decisions: Vec<&Act> = acts
            .iter()
            .filter(|a| {
                matches!(
                    a.did.as_str(),
                    Some("register_provider")
                        | Some("update_provider")
                        | Some("disable_provider")
                        | Some("set_default_provider")
                )
            })
            .collect();
        // Order by `when` (the Act graph is truth; this projection folds chronologically).
        decisions.sort_by(|a, b| {
            a.when.as_str().unwrap_or("").cmp(b.when.as_str().unwrap_or(""))
        });

        let mut reg = Self::default();
        for act in decisions {
            let this = &act.this;
            match act.did.as_str() {
                Some("register_provider") | Some("update_provider") => {
                    if let Some(p) = profile_from_this(this) {
                        reg.upsert(p);
                    }
                }
                Some("disable_provider") => {
                    let m = parse_descriptor(this.as_str().unwrap_or_default());
                    if let Some(id) = m.get("provider_id") {
                        reg.set_enabled(id, false);
                    }
                }
                Some("set_default_provider") => {
                    let m = parse_descriptor(this.as_str().unwrap_or_default());
                    if let Some(id) = m.get("provider_id") {
                        reg.default = Some(id.clone());
                    }
                }
                _ => {}
            }
        }
        reg
    }

    fn upsert(&mut self, profile: ProviderProfile) {
        self.providers.retain(|p| p.id != profile.id);
        self.providers.push(profile);
    }
    fn set_enabled(&mut self, id: &str, enabled: bool) {
        if let Some(p) = self.providers.iter_mut().find(|p| p.id == id) {
            p.enabled = enabled;
        }
    }

    pub fn get(&self, id: &str) -> Option<&ProviderProfile> {
        self.providers.iter().find(|p| p.id == id)
    }
    /// An enabled provider by id (disabled providers are not usable).
    pub fn get_enabled(&self, id: &str) -> Option<&ProviderProfile> {
        self.providers.iter().find(|p| p.id == id && p.enabled)
    }
    pub fn list(&self) -> &[ProviderProfile] {
        &self.providers
    }
    pub fn default_profile(&self) -> Option<&ProviderProfile> {
        self.default.as_ref().and_then(|id| self.get_enabled(id))
    }
}

// ============================================================ candidate building (pure)

fn sha256_hex(s: &str) -> String {
    let digest = Sha256::digest(s.as_bytes());
    let mut out = String::with_capacity(digest.len() * 2);
    for b in digest.iter() {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

/// Build the model-drafted candidate from a model's text output. **Pure** (no network) so
/// the provenance/confirmation invariants are unit-testable. `confirmed_by` is deliberately
/// empty — model authorship is provenance, never confirmation.
pub fn build_candidate(profile: &ProviderProfile, output_text: &str, source: Option<String>, now: &str) -> SessionActCandidate {
    let mp = ModelProvenance {
        provider_id: profile.id.clone(),
        provider_kind: profile.kind.as_str().to_string(),
        model: profile.model.clone(),
        endpoint_label: profile.base_url.clone(),
        request_time: now.to_string(),
        source,
        response_id: None,
        response_hash: Some(sha256_hex(output_text)),
    };
    let candidate = json!({
        "who": profile.id,
        "did": "provider_suggested_candidate",
        "this": output_text,
        "when": now,
        "confirmed_by": "",
        "if_ok": "",
        "if_doubt": "",
        "if_not": "",
        "status": "candidate"
    });
    SessionActCandidate::from_model_with_provenance(mp, candidate)
}

fn system_prompt(ctx: &ProviderContext) -> String {
    let mut s = String::from(
        "You participate in a LogLine Lab. You may draft, propose, critique, or summarize. \
         You do NOT decide, confirm, or admit anything. Reply concisely.",
    );
    for (name, json) in &ctx.surfaces {
        s.push_str(&format!("\n\n[surface:{name}]\n{json}"));
    }
    s
}

// ============================================================ openai-compatible adapter

/// The one concrete adapter, driven by genai, hidden behind `ProviderAdapter`.
pub struct OpenAiCompatibleAdapter {
    profile: ProviderProfile,
    api_key: String,
    now: String,
}

impl OpenAiCompatibleAdapter {
    pub fn from_profile(profile: ProviderProfile, now: impl Into<String>) -> Result<Self, ProviderError> {
        if !profile.kind.is_available() {
            return Err(ProviderError(format!(
                "provider kind `{}` is SOON; only openai-compatible is available",
                profile.kind.as_str()
            )));
        }
        let api_key = match &profile.auth.env {
            Some(env) => std::env::var(env).unwrap_or_default(),
            None => String::new(),
        };
        Ok(Self { profile, api_key, now: now.into() })
    }

    fn client(&self) -> Client {
        // genai's OpenAI adapter appends `chat/completions` to the endpoint, so the base
        // must end with `/` (accept profiles written with or without the trailing slash).
        let base = if self.profile.base_url.ends_with('/') {
            self.profile.base_url.clone()
        } else {
            format!("{}/", self.profile.base_url)
        };
        let key = if self.api_key.is_empty() { "EMPTY".to_string() } else { self.api_key.clone() };
        let model = self.profile.model.clone();
        let resolver = ServiceTargetResolver::from_resolver_fn(
            move |_t: ServiceTarget| -> Result<ServiceTarget, genai::resolver::Error> {
                Ok(ServiceTarget {
                    endpoint: Endpoint::from_owned(base.clone()),
                    auth: AuthData::from_single(key.clone()),
                    model: ModelIden::new(AdapterKind::OpenAI, model.clone()),
                })
            },
        );
        Client::builder().with_service_target_resolver(resolver).build()
    }

    async fn chat(&self, system: &str, user: &str) -> Result<String, ProviderError> {
        let req = ChatRequest::new(vec![ChatMessage::system(system), ChatMessage::user(user)]);
        let res = self
            .client()
            .exec_chat(&self.profile.model, req, None)
            .await
            .map_err(|e| ProviderError(e.to_string()))?;
        Ok(res.into_first_text().unwrap_or_default())
    }

    /// Suggest a candidate Act from user text + context. Candidate material only.
    pub async fn suggest_candidate(&self, text: &str, ctx: &ProviderContext) -> Result<SessionActCandidate, ProviderError> {
        let out = self.chat(&system_prompt(ctx), text).await?;
        let source = ctx.surfaces.first().map(|(n, _)| n.clone());
        Ok(build_candidate(&self.profile, &out, source, &self.now))
    }
}

impl ProviderAdapter for OpenAiCompatibleAdapter {
    async fn prepare_context(&self) -> Result<ProviderContext, ProviderError> {
        Ok(ProviderContext::default())
    }
    async fn receive_user_input(&self, input: &str, ctx: &ProviderContext) -> Result<ProviderTurn, ProviderError> {
        let candidate = self.suggest_candidate(input, ctx).await?;
        let text = candidate.candidate.get("this").and_then(Value::as_str).unwrap_or_default().to_string();
        Ok(ProviderTurn { text, candidate: Some(candidate) })
    }
    async fn produce_candidate(&self, ctx: &ProviderContext) -> Result<SessionActCandidate, ProviderError> {
        self.suggest_candidate("Propose one next candidate Act for this Lab.", ctx).await
    }
    async fn critique_evidence(&self, act: &Value, ctx: &ProviderContext) -> Result<Critique, ProviderError> {
        let notes = self.chat(&system_prompt(ctx), &format!("Critique the evidence gaps in this Act:\n{act}")).await?;
        Ok(Critique { gaps: Vec::new(), notes })
    }
    async fn suggest_next_act(&self, ctx: &ProviderContext) -> Result<SessionActCandidate, ProviderError> {
        self.produce_candidate(ctx).await
    }
    async fn summarize_surface(&self, surface_json: &Value) -> Result<String, ProviderError> {
        self.chat("Summarize this LogLine surface for a human. No decisions.", &surface_json.to_string()).await
    }
}

/// Synchronous suggest for the (sync) CLI: build the adapter, one round-trip on a small
/// tokio runtime, return a model-drafted candidate. Admits nothing.
pub fn suggest_blocking(profile: &ProviderProfile, text: &str, context: &ProviderContext, now: &str) -> Result<SessionActCandidate, ProviderError> {
    let adapter = OpenAiCompatibleAdapter::from_profile(profile.clone(), now)?;
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| ProviderError(e.to_string()))?;
    rt.block_on(adapter.suggest_candidate(text, context))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile() -> ProviderProfile {
        ProviderProfile {
            id: "minilab".into(),
            kind: ProviderKind::OpenAiCompatible,
            base_url: "https://example.com/v1".into(),
            model: "default".into(),
            auth: AuthConfig { kind: "bearer_env".into(), env: Some("LOGLINE_PROVIDER_API_KEY".into()) },
            enabled: true,
            dev_only: true,
        }
    }

    fn act(v: Value) -> Act {
        Act::from_value_strict(&v).expect("provider-decision Act is a strict nine-slot Act")
    }

    /// Provider-decision Act builders produce strict, admittable nine-slot Acts that carry
    /// NO secret (only the env-var name).
    #[test]
    fn register_act_is_strict_string_slots_and_secret_free() {
        let v = register_provider_act("operator", &profile(), "2026-06-07T00:00:00Z");
        let a = act(v.clone());
        assert_eq!(a.did.as_str(), Some("register_provider"));
        // `this` is a STRING slot (canon mold nature), not a structured object.
        let this = v["this"].as_str().expect("this is a string slot");
        assert!(!v["this"].is_object(), "canon-shape Acts use string slots, not objects");
        // env var NAME present; the key VALUE is never present.
        assert!(this.contains("auth_env=LOGLINE_PROVIDER_API_KEY"));
        assert!(this.contains("base_url=https://example.com/v1"));
        // round-trips through the projection parser
        let p = profile_from_this(&v["this"]).expect("projects back");
        assert_eq!(p.id, "minilab");
        assert_eq!(p.auth.env.as_deref(), Some("LOGLINE_PROVIDER_API_KEY"));
        assert!(p.dev_only);
    }

    /// The registry is a PROJECTION folded from Acts in `when` order: register → set
    /// default → disable resolves correctly, and there is no file truth involved.
    #[test]
    fn registry_projects_from_acts() {
        let acts = vec![
            act(register_provider_act("operator", &profile(), "2026-06-07T00:00:01Z")),
            act(set_default_provider_act("operator", "minilab", "resident_session", "2026-06-07T00:00:02Z")),
        ];
        let reg = ProviderRegistry::project_from_acts(&acts);
        assert_eq!(reg.default_profile().unwrap().model, "default");
        assert!(reg.get_enabled("minilab").is_some());

        // a later disable Act removes it from the usable set — rebuilt purely from Acts
        let mut acts2 = acts;
        acts2.push(act(disable_provider_act("operator", "minilab", "2026-06-07T00:00:03Z")));
        let reg2 = ProviderRegistry::project_from_acts(&acts2);
        assert!(reg2.get_enabled("minilab").is_none(), "disabled provider is not usable");
        assert!(reg2.default_profile().is_none(), "default falls away when disabled");
    }

    #[test]
    fn build_candidate_carries_provenance_and_is_not_confirmation() {
        let c = build_candidate(&profile(), "maybe measure twice?", Some("today".into()), "2026-06-07T00:00:00Z");
        let mp = c.provenance.model.as_ref().expect("model provenance");
        assert_eq!(mp.provider_id, "minilab");
        assert_eq!(mp.provider_kind, "openai-compatible");
        assert!(mp.response_hash.is_some());
        assert!(c.is_model_drafted());
        assert_eq!(c.candidate.get("confirmed_by").and_then(Value::as_str), Some(""));
        assert_eq!(c.candidate.get("status").and_then(Value::as_str), Some("candidate"));
        assert_eq!(c.candidate.get("did").and_then(Value::as_str), Some("provider_suggested_candidate"));
    }

    #[test]
    fn soon_provider_kinds_are_rejected_by_adapter() {
        let mut p = profile();
        p.kind = ProviderKind::Anthropic;
        let err = match OpenAiCompatibleAdapter::from_profile(p, "t0") {
            Err(e) => e,
            Ok(_) => panic!("expected a SOON rejection"),
        };
        assert!(err.to_string().contains("SOON"));
    }

    #[test]
    fn availability_matrix_marks_only_openai_compatible_available() {
        let m = availability();
        assert!(m.iter().any(|(k, s)| *k == "openai-compatible" && *s == "available"));
        assert!(m.iter().filter(|(_, s)| *s == "soon").count() >= 4);
    }

    #[test]
    fn provider_error_is_reportable() {
        assert!(ProviderError("boom".into()).to_string().contains("boom"));
    }
}
