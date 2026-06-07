//! Resident session (provider-free) — step C.
//!
//! Wires the provider-neutral session substrate ([`logline_lab_session`]) to a live,
//! directory-backed [`Lab`]. It proves the core point **without any LLM attached**: a Lab
//! session is not "chat completions plus tools" — it is presence over the Act graph.
//!
//! Boundaries (all structural here):
//! - the session **captures candidates** and **requests promotion**; it never admits.
//! - **human approval** mints an *authorization candidate* (via the substrate) and then
//!   asks the **Lab** to admit the target through its existing discipline ([`Lab::emit`]).
//! - **resumability** comes from reopening the Lab directory (actlog + candidates), not a
//!   separate session-truth store. The transcript is projected from the Lab's Acts.

use logline_act::Act;
use logline_lab_session::{
    ApprovalDecision, ApprovalRequest, Drafter, Participant, Provenance, ProviderId, Session,
    SessionEvent, SessionId, SessionTranscriptView,
};
use serde::Serialize;
use serde_json::{json, Value};

use crate::{Lab, LabError};

/// A resident session bound to one directory-backed Lab. Holds no provider.
pub struct ResidentSession {
    lab: Lab,
    id: SessionId,
    human: String,
}

/// The result of a human approval: the authorization candidate that was captured, and the
/// Lab's admission outcome for the target (if the Lab admitted it).
#[derive(Clone, Debug, Serialize)]
pub struct ApproveOutcome {
    /// The authorization candidate Act (who = human, did = `authorize_promotion`).
    pub authorization: Value,
    /// Content hash of the target Act the **Lab** admitted (None if not admitted).
    pub admitted_content_hash: Option<String>,
    pub detail: String,
}

impl ResidentSession {
    /// Open (or resume) a resident session over a directory-backed Lab. No provider.
    pub fn open(lab: Lab, id: impl Into<String>, human: impl Into<String>) -> Self {
        Self { lab, id: SessionId(id.into()), human: human.into() }
    }

    pub fn lab(&self) -> &Lab {
        &self.lab
    }
    pub fn lab_mut(&mut self) -> &mut Lab {
        &mut self.lab
    }
    /// A resident session is provider-free (step C). Provider attachment is step D.
    pub fn provider(&self) -> Option<&ProviderId> {
        None
    }

    /// Read a Lab read-surface as JSON (start/today/timeline/schedule/learn/settings, plus
    /// `storage`). The same JSON every other surface consumer sees.
    pub fn view(&self, surface: &str, now: &str) -> Result<Value, LabError> {
        if surface == "storage" {
            return Ok(serde_json::to_value(crate::storage_matrix()).unwrap_or(Value::Null));
        }
        self.lab
            .render_surface(surface, now)
            .ok_or_else(|| LabError::Unsupported(format!("no read surface `{surface}`")))
    }

    /// Capture human text as candidate material (a `note` candidate). Captured, NOT admitted.
    pub fn write_text(&mut self, text: &str) -> Result<Value, LabError> {
        let cand = json!({
            "who": self.human, "did": "note", "this": text, "when": "",
            "confirmed_by": "", "if_ok": "", "if_doubt": "", "if_not": "", "status": "candidate"
        });
        self.lab.capture_candidate(cand.clone())?;
        Ok(cand)
    }

    /// Capture a structured candidate Act value. Captured, NOT admitted.
    pub fn write_candidate(&mut self, value: Value) -> Result<(), LabError> {
        self.lab.capture_candidate(value)
    }

    /// Request promotion of a target. Records a promotion-request candidate
    /// (who = human, did = `request_promotion`). Does NOT admit.
    pub fn request_promotion(&mut self, target: &str) -> Result<Value, LabError> {
        let req = json!({
            "who": self.human, "did": "request_promotion", "this": target, "when": "",
            "confirmed_by": "", "if_ok": "human_authorizes", "if_doubt": "hold",
            "if_not": "withdraw", "status": "candidate"
        });
        self.lab.capture_candidate(req.clone())?;
        Ok(req)
    }

    /// Human approval: mint an authorization candidate via the substrate (the session
    /// never admits), then ask the **Lab** to admit the target via its discipline. The
    /// Lab refuses admission under a `candidate-only` grade — that refusal is correct.
    pub fn approve(&mut self, target: &Act, now: &str) -> Result<ApproveOutcome, LabError> {
        let target_hash = target.content_hash().map_err(LabError::from)?;

        // Substrate produces the authorization candidate; it has no admission power.
        let mut s = Session::open(self.id.clone(), Participant::human(&self.human));
        let decision = ApprovalDecision {
            request: ApprovalRequest {
                target: target_hash.clone(),
                requested_by: Participant::human(&self.human),
                scope: None,
            },
            decided_by: Participant::human(&self.human),
            approved: true,
            note: None,
        };
        let auth = s
            .approve(decision)
            .expect("human approval yields an authorization candidate");
        let mut auth_value = auth.candidate;
        auth_value["when"] = json!(now);
        self.lab.capture_candidate(auth_value.clone())?;

        // The LAB admits the target (promotion discipline; refuses candidate-only),
        // then rehydrates the query spine from the durable Act-log.
        let outcome = self.lab.emit(target)?;
        self.lab.sync()?;
        Ok(ApproveOutcome {
            authorization: auth_value,
            admitted_content_hash: Some(outcome.content_hash().to_string()),
            detail: "lab admitted target after human authorization".to_string(),
        })
    }

    /// Tick the Lab (confront time → tick/disposition/reschedule Acts), then sync.
    pub fn tick(&mut self, now: &str, next_due: &str) -> Result<crate::TickReport, LabError> {
        let report = self.lab.tick(now, next_due)?;
        self.lab.sync()?;
        Ok(report)
    }

    /// Project the transcript from the Lab's Acts + candidates. A read-model, rebuildable
    /// after restart from the Lab alone — there is no separate session-truth store.
    pub fn transcript(&self) -> SessionTranscriptView {
        let mut events = vec![SessionEvent::Opened { by: Participant::human(&self.human) }];
        for _c in self.lab.candidates() {
            events.push(SessionEvent::CandidateDrafted {
                provenance: Provenance {
                    drafted_by: Drafter::Human { id: self.human.clone() },
                    provider: None,
                    note: None,
                },
            });
        }
        for stored in self.lab.spine().all() {
            if let Ok(h) = stored.act.content_hash() {
                events.push(SessionEvent::ActAdmitted { content_hash: h });
            }
        }
        SessionTranscriptView {
            kind: "logline.view.session_transcript.v0".to_string(),
            session: self.id.clone(),
            events,
        }
    }

    /// Close the session. State already lives in the Lab; nothing else to persist.
    pub fn close(self) -> Lab {
        self.lab
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logline_lab_core::{LabManifest, ProfileManifest};

    fn dev_lab(dir: &std::path::Path) -> Lab {
        // dev-ephemeral so admission is possible (still non-publication).
        let manifest =
            LabManifest::load(r#"{"lab_id":"ref.local.lab","profile":"dev"}"#).unwrap();
        let profile = ProfileManifest::load(
            r#"{"name":"dev","spine":"dev-ephemeral","settings":{}}"#,
        )
        .unwrap();
        Lab::open(manifest, Vec::new(), profile, dir).unwrap()
    }

    fn valid_act() -> Act {
        Act::from_value_strict(&json!({
            "who": "dan", "did": "observe", "this": "a_reading", "when": "2026-06-07T00:00:00Z",
            "confirmed_by": "dan", "if_ok": "record", "if_doubt": "open_blocked", "if_not": "discard",
            "status": "admitted"
        }))
        .unwrap()
    }

    #[test]
    fn opens_without_provider_and_reads_surfaces() {
        let tmp = std::env::temp_dir().join("llk-resident-views");
        let _ = std::fs::remove_dir_all(&tmp);
        let mut rs = ResidentSession::open(dev_lab(&tmp), "s1", "dan");
        assert!(rs.provider().is_none());
        for surface in ["start", "today", "timeline", "schedule", "learn", "settings", "storage"] {
            let v = rs.view(surface, "2026-06-07T00:00:00Z").expect(surface);
            assert!(v.is_object() || v.is_array(), "surface {surface} returned a value");
        }
        // capture human text as a candidate — NOT admitted
        rs.write_text("a scribbled thought").unwrap();
        assert_eq!(rs.lab().candidates().len(), 1);
        assert!(rs.lab().spine().all().is_empty(), "write must not admit");
    }

    #[test]
    fn request_promotion_does_not_admit() {
        let tmp = std::env::temp_dir().join("llk-resident-reqprom");
        let _ = std::fs::remove_dir_all(&tmp);
        let mut rs = ResidentSession::open(dev_lab(&tmp), "s1", "dan");
        rs.request_promotion("some_target_hash").unwrap();
        assert!(rs.lab().spine().all().is_empty(), "requesting promotion must not admit");
        assert_eq!(rs.lab().candidates().len(), 1);
    }

    #[test]
    fn approve_mints_authorization_and_lab_admits() {
        let tmp = std::env::temp_dir().join("llk-resident-approve");
        let _ = std::fs::remove_dir_all(&tmp);
        let mut rs = ResidentSession::open(dev_lab(&tmp), "s1", "dan");
        let act = valid_act();
        let out = rs.approve(&act, "2026-06-07T00:00:00Z").unwrap();
        // authorization candidate exists and is an authorize_promotion candidate Act
        assert_eq!(out.authorization.get("did").and_then(Value::as_str), Some("authorize_promotion"));
        assert_eq!(out.authorization.get("status").and_then(Value::as_str), Some("candidate"));
        // the LAB admitted the target
        assert!(out.admitted_content_hash.is_some());
        assert_eq!(rs.lab().spine().all().len(), 1);
    }

    #[test]
    fn resumes_after_restart_from_acts_not_a_session_store() {
        let tmp = std::env::temp_dir().join("llk-resident-resume");
        let _ = std::fs::remove_dir_all(&tmp);

        // Process 1: write a candidate, approve (admit) an act, then drop everything.
        {
            let mut rs = ResidentSession::open(dev_lab(&tmp), "s1", "dan");
            rs.write_text("note before restart").unwrap();
            rs.approve(&valid_act(), "2026-06-07T00:00:00Z").unwrap();
            let _lab = rs.close();
        }

        // No separate session store: only the Lab dir persists.
        assert!(!tmp.join("session.json").exists(), "there must be no session-truth store");

        // Process 2: reopen the SAME dir — state resumes from the Lab's Acts/candidates.
        let rs2 = ResidentSession::open(dev_lab(&tmp), "s1", "dan");
        let t = rs2.transcript();
        assert_eq!(t.kind, "logline.view.session_transcript.v0");
        assert!(t.events.iter().any(|e| matches!(e, SessionEvent::ActAdmitted { .. })),
            "admitted act resumes from the actlog");
        assert!(t.events.iter().any(|e| matches!(e, SessionEvent::CandidateDrafted { .. })),
            "candidate resumes from candidates.jsonl");
    }
}
