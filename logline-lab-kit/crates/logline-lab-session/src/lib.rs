//! `logline-lab-session` — the provider-neutral **session substrate** (step B).
//!
//! A session is **presence + runtime over the Act graph, not authority**. This crate
//! deliberately has NO admission path, NO spine, and NO provider/runtime dependencies:
//!
//! - A session can **draft / propose / capture candidates** only.
//! - A session **cannot admit Acts**. Only the Lab admits, after promotion discipline.
//!   There is no `admit` here and no access to a spine — the boundary is *structural*.
//! - Human approval produces an **authorization candidate** (a LogLine Act candidate:
//!   who = human, did = `authorize_promotion`), never an admission.
//! - **Model output alone never satisfies confirmation.** Model authorship is recorded as
//!   [`Provenance`], which is separate from the Act's `confirmed_by` slot; the substrate
//!   never writes `confirmed_by` from provenance.
//! - **Resumability derives from Acts/projections**, not a separate durable session store.
//!   A [`Session`] holds only in-memory events (a cache/projection); rebuild it from the
//!   Lab's Acts via [`Session::resume_from`].
//!
//! "Receipt", "ghost", "evidence", "message", "workorder", "learning" are projections /
//! conventions over Acts — not separate truth objects. See
//! `docs/SESSIONS_AND_AGENT_RUNTIME.md`.

#![forbid(unsafe_code)]

use logline_act::{Act, Candidate};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

mod provider;
pub use provider::{Critique, ProviderAdapter, ProviderContext, ProviderError, ProviderTurn};

/// Opaque session identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

/// Opaque provider identifier (e.g. `ollama`, `anthropic`, `openai`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProviderId(pub String);

/// The kind of a session participant. Each has a fixed place in the authority model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParticipantKind {
    /// Authorizes, confirms, rejects, carries consequence.
    Human,
    /// Drafts / proposes / criticizes / translates. Never decides.
    Model,
    /// Participates under grants (MCP client, app).
    App,
    /// Executes admitted workorders, returns evidence (never closure).
    Worker,
    /// Owns state and admits Acts. The only admission authority.
    LabHost,
}

/// A participant in a session.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    pub kind: ParticipantKind,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub display: Option<String>,
}

impl Participant {
    pub fn human(id: impl Into<String>) -> Self {
        Self { id: id.into(), kind: ParticipantKind::Human, display: None }
    }
    pub fn model(id: impl Into<String>) -> Self {
        Self { id: id.into(), kind: ParticipantKind::Model, display: None }
    }
    pub fn is_human(&self) -> bool {
        self.kind == ParticipantKind::Human
    }
}

/// Who drafted a candidate. Authorship, NOT confirmation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "by")]
pub enum Drafter {
    Human { id: String },
    Model { provider: ProviderId },
    App { id: String },
    Worker { id: String },
}

/// Provenance of a candidate: who drafted it and (if a model assisted) which provider.
/// Kept separate from the Act's `confirmed_by` slot on purpose — provenance is authorship,
/// `confirmed_by` is confirmation/witness/proof/validator/signature/acknowledged-absence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    pub drafted_by: Drafter,
    /// Set iff a model produced or assisted this candidate.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub provider: Option<ProviderId>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub note: Option<String>,
}

/// A candidate Act produced in-session, with attribution. The Lab admits it (or not);
/// the session never does.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionActCandidate {
    /// The raw, preservable candidate value (candidate-generous; promotion-strict).
    pub candidate: Value,
    pub provenance: Provenance,
}

impl SessionActCandidate {
    /// A human-drafted candidate.
    pub fn from_human(human_id: impl Into<String>, raw: Value) -> Self {
        Self {
            candidate: raw,
            provenance: Provenance {
                drafted_by: Drafter::Human { id: human_id.into() },
                provider: None,
                note: None,
            },
        }
    }

    /// A model-drafted candidate. Records model provenance; does NOT — and must not —
    /// touch `confirmed_by`. Model authorship is not confirmation.
    pub fn from_model(provider: ProviderId, raw: Value) -> Self {
        Self {
            candidate: raw,
            provenance: Provenance {
                drafted_by: Drafter::Model { provider: provider.clone() },
                provider: Some(provider),
                note: None,
            },
        }
    }

    /// Preserve the raw value as a [`Candidate`] (never rejected; repaired before promotion).
    pub fn as_candidate(&self) -> Candidate {
        Act::candidate_from_value(&self.candidate)
    }

    /// True iff a model produced/assisted this candidate.
    pub fn is_model_drafted(&self) -> bool {
        self.provenance.provider.is_some()
    }
}

/// A capability granted to a participant for the duration of a session.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolGrant {
    pub participant: String,
    /// e.g. `read:today`, `draft_candidate`, `run_conformance`, `submit_evidence`.
    pub capability: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scope: Option<String>,
}

/// A request to promote a target candidate. Promotion is triggered/authorized by a human;
/// admission is still the Lab's.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRequest {
    /// Identifies the target (e.g. a content hash or in-session candidate ref).
    pub target: String,
    pub requested_by: Participant,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scope: Option<String>,
}

/// A human's decision on an [`ApprovalRequest`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalDecision {
    pub request: ApprovalRequest,
    pub decided_by: Participant,
    pub approved: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub note: Option<String>,
}

/// A session event. Each is an Act or a projection over Acts — never a parallel authority
/// log. The transcript is built from these.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum SessionEvent {
    Opened { by: Participant },
    ParticipantJoined { participant: Participant },
    ProviderAttached { provider: ProviderId },
    SurfaceViewed { participant: String, surface: String },
    HumanMessage { from: String, text: String },
    ModelMessage { provider: ProviderId, text: String },
    CandidateDrafted { provenance: Provenance },
    ApprovalRequested { request: ApprovalRequest },
    ApprovalDecided { decision: ApprovalDecision },
    /// Recorded when the *Lab* admits an Act (the session observes; it does not admit).
    ActAdmitted { content_hash: String },
    EvidenceReferenced { reference: String },
    GhostProjected { about: String },
    Tick { now: String },
    LearningSuggested { summary: String },
    Closed,
}

/// A read-model projection of the session transcript. Rebuildable from events / Acts;
/// never a source of truth.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionTranscriptView {
    pub kind: String,
    pub session: SessionId,
    pub events: Vec<SessionEvent>,
}

/// A live session. Holds only in-memory events (cache/projection). No admission path,
/// no spine, no provider handle — those live outside the substrate.
#[derive(Clone, Debug)]
pub struct Session {
    id: SessionId,
    participants: Vec<Participant>,
    provider: Option<ProviderId>,
    grants: Vec<ToolGrant>,
    events: Vec<SessionEvent>,
}

impl Session {
    /// Open a session. Valid with NO provider attached.
    pub fn open(id: SessionId, opener: Participant) -> Self {
        let mut s = Self {
            id,
            participants: vec![opener.clone()],
            provider: None,
            grants: Vec::new(),
            events: Vec::new(),
        };
        s.events.push(SessionEvent::Opened { by: opener });
        s
    }

    /// Rebuild a session view from events projected out of the Lab's Acts. This is how
    /// continuity works after a restart — there is no separate durable session store.
    pub fn resume_from(id: SessionId, events: Vec<SessionEvent>) -> Self {
        let mut participants = Vec::new();
        let mut provider = None;
        for e in &events {
            match e {
                SessionEvent::Opened { by } => participants.push(by.clone()),
                SessionEvent::ParticipantJoined { participant } => participants.push(participant.clone()),
                SessionEvent::ProviderAttached { provider: p } => provider = Some(p.clone()),
                _ => {}
            }
        }
        Self { id, participants, provider, grants: Vec::new(), events }
    }

    pub fn id(&self) -> &SessionId {
        &self.id
    }
    pub fn participants(&self) -> &[Participant] {
        &self.participants
    }
    pub fn provider(&self) -> Option<&ProviderId> {
        self.provider.as_ref()
    }
    pub fn grants(&self) -> &[ToolGrant] {
        &self.grants
    }

    pub fn join(&mut self, participant: Participant) {
        self.events.push(SessionEvent::ParticipantJoined { participant: participant.clone() });
        self.participants.push(participant);
    }

    /// Provider attachment is optional; a session works without one.
    pub fn attach_provider(&mut self, provider: ProviderId) {
        self.events.push(SessionEvent::ProviderAttached { provider: provider.clone() });
        self.provider = Some(provider);
    }

    pub fn grant(&mut self, grant: ToolGrant) {
        self.grants.push(grant);
    }

    pub fn capture_human_text(&mut self, from: impl Into<String>, text: impl Into<String>) {
        self.events.push(SessionEvent::HumanMessage { from: from.into(), text: text.into() });
    }

    pub fn capture_model_text(&mut self, provider: ProviderId, text: impl Into<String>) {
        self.events.push(SessionEvent::ModelMessage { provider, text: text.into() });
    }

    /// Capture a drafted candidate (records authorship). The candidate is for the Lab to
    /// admit; the session does not admit.
    pub fn draft(&mut self, candidate: &SessionActCandidate) {
        self.events.push(SessionEvent::CandidateDrafted { provenance: candidate.provenance.clone() });
    }

    pub fn request_approval(&mut self, request: ApprovalRequest) {
        self.events.push(SessionEvent::ApprovalRequested { request });
    }

    /// Record a human approval decision and, if approved, emit an **authorization
    /// candidate** — a LogLine Act candidate (who = human, did = `authorize_promotion`,
    /// this = target). This expresses human authorization *as a candidate*; it does NOT
    /// admit the target. The Lab still admits, after promotion discipline.
    pub fn approve(&mut self, decision: ApprovalDecision) -> Option<SessionActCandidate> {
        self.events.push(SessionEvent::ApprovalDecided { decision: decision.clone() });
        if !decision.approved || !decision.decided_by.is_human() {
            return None;
        }
        let auth = json!({
            "who": decision.decided_by.id,
            "did": "authorize_promotion",
            "this": decision.request.target,
            "when": "",
            "confirmed_by": decision.decided_by.id,
            "if_ok": "lab_admits_target",
            "if_doubt": "hold_target_as_candidate",
            "if_not": "reject_target",
            "status": "candidate"
        });
        Some(SessionActCandidate::from_human(decision.decided_by.id, auth))
    }

    /// Observe that the Lab admitted an Act (the session records; it never admits).
    pub fn note_admission(&mut self, content_hash: impl Into<String>) {
        self.events.push(SessionEvent::ActAdmitted { content_hash: content_hash.into() });
    }

    pub fn close(&mut self) {
        self.events.push(SessionEvent::Closed);
    }

    /// Project the transcript. A read-model, rebuildable from events/Acts.
    pub fn transcript(&self) -> SessionTranscriptView {
        SessionTranscriptView {
            kind: "logline.view.session_transcript.v0".to_string(),
            session: self.id.clone(),
            events: self.events.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sid() -> SessionId {
        SessionId("s-001".into())
    }

    /// A session is valid with NO provider attached.
    #[test]
    fn session_valid_without_provider() {
        let s = Session::open(sid(), Participant::human("dan"));
        assert!(s.provider().is_none());
        assert_eq!(s.participants().len(), 1);
    }

    /// Model authorship is provenance, NOT confirmation: a model candidate records the
    /// provider but the substrate never sets `confirmed_by` from it.
    #[test]
    fn model_output_is_not_confirmation() {
        let raw = json!({
            "who": "ollama", "did": "propose", "this": "an_observation", "when": "",
            "confirmed_by": "", "if_ok": "", "if_doubt": "", "if_not": "", "status": "candidate"
        });
        let c = SessionActCandidate::from_model(ProviderId("ollama".into()), raw);
        assert!(c.is_model_drafted());
        assert_eq!(c.provenance.provider, Some(ProviderId("ollama".into())));
        // The substrate carried the model's draft verbatim and did NOT fill confirmed_by.
        assert_eq!(c.candidate.get("confirmed_by").and_then(Value::as_str), Some(""));
    }

    /// `approve` produces an authorization CANDIDATE, never an admission. (There is no
    /// admit method and no spine on Session — the boundary is structural.)
    #[test]
    fn approve_yields_authorization_candidate_not_admission() {
        let mut s = Session::open(sid(), Participant::human("dan"));
        let req = ApprovalRequest {
            target: "abc123".into(),
            requested_by: Participant::model("ollama"),
            scope: None,
        };
        let decision = ApprovalDecision {
            request: req,
            decided_by: Participant::human("dan"),
            approved: true,
            note: None,
        };
        let auth = s.approve(decision).expect("approved by a human yields an auth candidate");
        assert_eq!(auth.candidate.get("did").and_then(Value::as_str), Some("authorize_promotion"));
        assert_eq!(auth.candidate.get("status").and_then(Value::as_str), Some("candidate"));
        // It is a candidate, not an admitted Act: status is candidate, and the Lab must
        // still admit the *target*.
        assert!(matches!(auth.provenance.drafted_by, Drafter::Human { .. }));
    }

    /// A non-human (or denying) decision cannot mint an authorization candidate.
    #[test]
    fn only_human_approval_authorizes() {
        let mut s = Session::open(sid(), Participant::human("dan"));
        let req = ApprovalRequest { target: "abc".into(), requested_by: Participant::model("ollama"), scope: None };
        let by_model = ApprovalDecision { request: req.clone(), decided_by: Participant::model("ollama"), approved: true, note: None };
        assert!(s.approve(by_model).is_none(), "a model cannot authorize promotion");
        let denied = ApprovalDecision { request: req, decided_by: Participant::human("dan"), approved: false, note: None };
        assert!(s.approve(denied).is_none(), "a denial mints no authorization candidate");
    }

    /// Resumability: a session view rebuilds from projected events, with no separate store.
    #[test]
    fn resumes_from_events() {
        let mut s = Session::open(sid(), Participant::human("dan"));
        s.attach_provider(ProviderId("ollama".into()));
        s.join(Participant::model("ollama"));
        let events = s.transcript().events;

        let resumed = Session::resume_from(sid(), events);
        assert_eq!(resumed.provider(), Some(&ProviderId("ollama".into())));
        assert_eq!(resumed.participants().len(), 2);
    }

    /// The transcript is a projection (stable kind tag), rebuildable from events.
    #[test]
    fn transcript_is_a_projection() {
        let mut s = Session::open(sid(), Participant::human("dan"));
        s.capture_human_text("dan", "hello lab");
        let t = s.transcript();
        assert_eq!(t.kind, "logline.view.session_transcript.v0");
        assert!(t.events.iter().any(|e| matches!(e, SessionEvent::HumanMessage { .. })));
    }
}
