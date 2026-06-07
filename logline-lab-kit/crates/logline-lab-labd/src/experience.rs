//! The nine experience surfaces (FINAL §11) as library functions.
//!
//! "The surface is flexible. The experience grammar is not." These methods are
//! the grammar; CLI/MCP/web/TUI are surfaces that wrap them. They read
//! projections and act through the same discipline as everything else — settings
//! never bypass proof discipline (A40).

use logline_act::Act;
use logline_lab_core::{
    bench::{BenchOutcome, StudyBench},
    BlockContext,
};
use logline_lab_projectors::recent;
use logline_lab_ruler::{capacity, due_work, overdue_work, CapacityState};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{Lab, LabError};

/// Start — declare or open a Lab and confirm it can exist and remember.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartView {
    pub lab_id: String,
    pub profile: String,
    pub packs: Vec<String>,
    pub spine_kind: String,
    pub conformance_green: bool,
    pub total_acts: usize,
}

/// Today — the current operational study state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodayView {
    pub due: Vec<String>,
    pub overdue: Vec<String>,
    pub blocked: Vec<String>,
    pub running: Vec<String>,
    pub recent: Vec<String>,
    pub open_ghosts: Vec<String>,
    pub capacity: CapacityState,
    pub next_study: Option<String>,
}

/// Timeline — past / present / future Acts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimelineView {
    pub past: Vec<String>,
    pub present: Vec<String>,
    pub future: Vec<String>,
}

/// Write — outcome of capturing a candidate (ugly capture allowed).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum WriteOutcome {
    /// A valid nine-slot Act was admitted into the outbox.
    Admitted { content_hash: String },
    /// An imperfect candidate was preserved, with the slots it still needs.
    Candidate { missing: Vec<String> },
}

/// Workbench — the result of running a study bench.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbenchRun {
    pub bench_id: String,
    pub acts_emitted: usize,
    pub outcome: BenchOutcome,
}

/// Proof — claim / evidence / receipt / ghost kept strictly separate.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProofView {
    pub scope: String,
    pub claim_act_hash: String,
    pub evidence_count: usize,
    pub has_receipt_candidate: bool,
    pub open_ghosts: Vec<String>,
    /// Reminder that a report or model text is never proof.
    pub note: String,
}

/// Settings — configuration view. Authority is always locked: settings cannot
/// bypass Act discipline, proof discipline, or gate policy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingsView {
    pub profile: String,
    pub spine: String,
    pub packs: Vec<String>,
    pub authority_locked: bool,
}

fn is_open(act: &Act) -> bool {
    !matches!(
        act.status.as_str(),
        Some("done") | Some("closed") | Some("resolved") | Some("failed")
    )
}

impl Lab {
    /// Surface 1 — Start.
    pub fn start(&self) -> StartView {
        StartView {
            lab_id: self.lab_id().to_string(),
            profile: self.profile().name.clone(),
            packs: self.packs().iter().map(|p| p.name.clone()).collect(),
            spine_kind: self.spine().kind().to_string(),
            conformance_green: self.conformance().is_green(),
            total_acts: self.spine().all().len(),
        }
    }

    /// Surface 2 — Today.
    pub fn today(&self, now: &str) -> TodayView {
        let due: Vec<String> = due_work(self.spine(), now)
            .iter()
            .map(|a| a.content_hash().unwrap_or_default())
            .collect();
        let overdue: Vec<String> = overdue_work(self.spine(), now)
            .iter()
            .map(|a| a.content_hash().unwrap_or_default())
            .collect();

        let mut blocked = Vec::new();
        let mut running = Vec::new();
        for s in self.spine().all() {
            if s.act.status.as_str() == Some("running") {
                running.push(s.content_hash.clone());
            }
            let scope = s.act.did.as_str().unwrap_or("");
            let ctx = BlockContext { evidence: self.evidence(), permitted: true };
            if logline_lab_core::evaluate_blocked(&s.act, scope, &ctx).is_some() {
                blocked.push(s.content_hash.clone());
            }
        }

        let recent: Vec<String> = recent(self.spine(), 10)
            .iter()
            .filter_map(|s| s.act.did.as_str().map(|d| d.to_string()))
            .collect();
        let open_ghosts: Vec<String> = self.ghosts().open().iter().map(|g| g.id.clone()).collect();
        let cap = capacity(self.spine(), now, due.len());

        TodayView {
            due,
            overdue,
            blocked,
            running,
            recent,
            open_ghosts,
            capacity: cap.state,
            next_study: cap.next_study_proposal,
        }
    }

    /// Surface 3 — Timeline.
    pub fn timeline(&self, now: &str) -> TimelineView {
        let mut past = Vec::new();
        let mut present = Vec::new();
        let mut future = Vec::new();
        for s in self.spine().all() {
            let when = s.act.when.as_str().unwrap_or("");
            let due_at = s.act.this.get("due_at").and_then(|v| v.as_str());
            let marker = s.content_hash.clone();
            if let Some(d) = due_at {
                if d > now {
                    future.push(marker);
                    continue;
                }
            }
            match when.cmp(now) {
                std::cmp::Ordering::Less => past.push(marker),
                std::cmp::Ordering::Equal => present.push(marker),
                std::cmp::Ordering::Greater => future.push(marker),
            }
        }
        TimelineView { past, present, future }
    }

    /// Surface 4 — Write (ugly capture allowed, promotion strict).
    pub fn write(&mut self, value: &Value) -> Result<WriteOutcome, LabError> {
        match Act::from_value_strict(value) {
            Ok(act) => {
                let outcome = self.emit(&act)?;
                Ok(WriteOutcome::Admitted {
                    content_hash: outcome.content_hash().to_string(),
                })
            }
            Err(_) => {
                let candidate = Act::candidate_from_value(value);
                let missing = candidate.missing_slots().iter().map(|s| s.to_string()).collect();
                self.candidates.push(value.clone());
                Ok(WriteOutcome::Candidate { missing })
            }
        }
    }

    /// Surface 5 — Schedule: place an Act as a future obligation at `due_at`.
    pub fn schedule(&mut self, act: &Act, due_at: &str) -> Result<Act, LabError> {
        let mut this = act.this.clone();
        if let Value::Object(map) = &mut this {
            map.insert("due_at".to_string(), json!(due_at));
        } else {
            this = json!({ "due_at": due_at, "subject": act.this });
        }
        let scheduled = Act::new(
            act.who.clone(),
            act.did.clone(),
            this,
            act.when.clone(),
            act.confirmed_by.clone(),
            act.if_ok.clone(),
            act.if_doubt.clone(),
            act.if_not.clone(),
            json!("scheduled"),
        );
        self.emit(&scheduled)?;
        Ok(scheduled)
    }

    /// Surface 6 — Workbench: run a study bench. Emits its declared Acts, then
    /// turns the observation into evidence or a ghost.
    pub fn workbench(
        &mut self,
        bench: &StudyBench,
        expectation_met: bool,
        observed: Value,
        who: &str,
        now: &str,
    ) -> Result<WorkbenchRun, LabError> {
        let acts = bench.declare_acts(who, now);
        for a in &acts {
            self.emit(a)?;
        }
        let outcome = bench.observe(expectation_met, observed, now);
        match &outcome {
            BenchOutcome::Evidence(ev) => self.attach_evidence(ev.clone()),
            BenchOutcome::Ghost(g) => self.record_ghost(g.clone()),
        }
        Ok(WorkbenchRun {
            bench_id: bench.id.clone(),
            acts_emitted: acts.len(),
            outcome,
        })
    }

    /// Surface 7 — Proof: separate claim, evidence, receipt candidate, and ghost.
    pub fn proof(&self, act: &Act, scope: &str) -> ProofView {
        let evidence_count = self.evidence().for_scope(scope).len();
        let has_receipt_candidate = self.prepare_receipt(act, scope).is_ok();
        let open_ghosts: Vec<String> = self
            .ghosts()
            .for_scope(scope)
            .iter()
            .filter(|g| !g.closed)
            .map(|g| g.id.clone())
            .collect();
        ProofView {
            scope: scope.to_string(),
            claim_act_hash: act.content_hash().unwrap_or_default(),
            evidence_count,
            has_receipt_candidate,
            open_ghosts,
            note: "claim, evidence, receipt candidate, and ghost are separate; a report or model text is never proof".to_string(),
        }
    }

    /// Surface 8 — Learn (delegates to the learning report).
    pub fn learn(&self, now: &str) -> logline_lab_reports::LearningReport {
        self.learning(now)
    }

    /// Surface 9 — Settings. Read-only view; authority is always locked.
    pub fn settings(&self) -> SettingsView {
        SettingsView {
            profile: self.profile().name.clone(),
            spine: self.profile().spine.clone(),
            packs: self.packs().iter().map(|p| p.name.clone()).collect(),
            authority_locked: true,
        }
    }

    /// Count of currently open (non-terminal) Acts on the spine.
    pub fn open_act_count(&self) -> usize {
        self.spine().all().iter().filter(|s| is_open(&s.act)).count()
    }
}
