//! The nine experience surfaces (FINAL §11) as **headless read-model contracts**.
//!
//! "The surface is flexible. The experience grammar is not." Each surface is a
//! library function returning a stable, versioned JSON struct (a `kind` tag marks
//! the contract). The `labkit` CLI and any MCP/GUI/TUI client consume the *same*
//! structs, so a human and an LLM open the same Lab and see the same reality:
//! the same Acts, candidates, evidence, ghosts, receipts, schedule, blocked/
//! overdue/due obligations, experiments, proof state, learning, and uncertainty.
//!
//! Division of labor (FINAL §0–§17): humans authorize and carry consequences;
//! LLMs translate/route/explain/criticize but never decide; automation handles
//! repetition, scheduling, validation, observation, and continuity.

use logline_act::Act;
use logline_lab_core::{
    bench::{BenchOutcome, StudyBench},
    BlockContext, Grade,
};
use logline_lab_clock::reschedule;
use logline_lab_projectors::recent;
use logline_lab_ruler::{capacity, due_work, overdue_work, CapacityState};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{Lab, LabError};

/// One storage/spine option in the onboarding matrix.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpineOption {
    pub id: String,
    /// `available` | `soon`.
    pub status: String,
    /// `candidate-only` | `dev-ephemeral` | `publication`.
    pub grade: String,
    pub note: String,
}

/// Start — declare or open a Lab and confirm it can exist and remember.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartView {
    pub kind: String,
    pub lab_id: String,
    pub profile: String,
    pub packs: Vec<String>,
    pub spine_kind: String,
    /// Storage grade of this Lab (candidate-only / dev-ephemeral / publication).
    pub grade: Grade,
    pub publication_grade: bool,
    /// Honest warning when admitted Acts are not publication-grade.
    pub storage_warning: Option<String>,
    /// The storage decision matrix (what an operator may choose, with status).
    pub storage_matrix: Vec<SpineOption>,
    pub conformance_green: bool,
    pub total_acts: usize,
    /// First valid next actions an operator (or LLM) can take.
    pub next_actions: Vec<String>,
}

/// Today — the current operational study state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodayView {
    pub kind: String,
    pub now: String,
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
    pub kind: String,
    pub now: String,
    pub past: Vec<String>,
    pub present: Vec<String>,
    pub future: Vec<String>,
}

/// Schedule — scheduled/due/overdue/blocked obligations and ruler decisions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleView {
    pub kind: String,
    pub now: String,
    pub scheduled: Vec<String>,
    pub due: Vec<String>,
    pub overdue: Vec<String>,
    pub blocked: Vec<String>,
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
    pub kind: String,
    pub bench_id: String,
    pub acts_emitted: usize,
    pub outcome: BenchOutcome,
}

/// Proof — claim / evidence / receipt / ghost kept strictly separate.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProofView {
    pub kind: String,
    pub scope: String,
    pub claim_act_hash: String,
    pub evidence_count: usize,
    pub has_receipt_candidate: bool,
    pub open_ghosts: Vec<String>,
    /// Reminder that a report or model text is never proof.
    pub note: String,
}

/// The explicit disposition of one due Act at a tick.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DueDisposition {
    pub content_hash: String,
    /// `executable` | `blocked`.
    pub disposition: String,
    pub reason: Option<String>,
}

/// Tick — the materialized confrontation with time (the ruler report). Unlike the
/// read-only Today/Schedule views, a tick *emits Acts*: a `clock_tick` Act, a
/// `due_disposition` Act per due Act, and `reschedule_act` Acts for overdue work.
/// No due Act is skipped silently (A25).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TickReport {
    pub kind: String,
    pub now: String,
    pub tick_act: Option<String>,
    pub due: Vec<DueDisposition>,
    pub overdue: Vec<String>,
    pub rescheduled: Vec<String>,
    pub capacity: CapacityState,
    pub next_study: Option<String>,
    pub acts_emitted: usize,
    /// When candidate-only, the tick evaluates but cannot emit (no admission).
    pub materialized: bool,
}

/// Settings — configuration view. Authority is always locked: settings cannot
/// bypass Act discipline, proof discipline, or gate policy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingsView {
    pub kind: String,
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

/// The onboarding storage matrix: where admitted Acts can be registered, with
/// honest status. Most external spines are SOON in v0 (RELEASE_SCOPE).
pub fn storage_matrix() -> Vec<SpineOption> {
    let row = |id: &str, status: &str, grade: &str, note: &str| SpineOption {
        id: id.to_string(),
        status: status.to_string(),
        grade: grade.to_string(),
        note: note.to_string(),
    };
    vec![
        row("candidate-only", "available", "candidate-only", "capture candidates only; no admitted Acts"),
        row("dev-ephemeral", "available", "dev-ephemeral", "local Act-log; dev-only, non-publication-grade"),
        row("postgres", "soon", "publication", "external Postgres spine (--features supabase-profile)"),
        row("neon", "soon", "publication", "Neon Postgres spine"),
        row("supabase", "soon", "publication", "Supabase spine (--features supabase-profile)"),
        row("bring-your-own", "soon", "publication", "implement the Spine trait for your own backend"),
    ]
}

impl Lab {
    /// Surface 1 — Start. Surfaces the storage decision honestly so an operator
    /// (or LLM) knows whether this Lab's Acts are publication-grade.
    pub fn start(&self) -> StartView {
        let total_acts = self.spine().all().len();
        let grade = self.admission_grade();
        let mut next_actions = Vec::new();
        if !grade.is_publication() {
            next_actions.push(
                "choose a Spine Profile for publication-grade Acts (see storage_matrix)".to_string(),
            );
        }
        if total_acts == 0 {
            next_actions.push("write the first candidate Act (labkit write)".to_string());
        }
        next_actions.push("run a study bench (labkit workbench)".to_string());
        next_actions.push("check what is due today (labkit today)".to_string());
        StartView {
            kind: "logline.view.start.v0".to_string(),
            lab_id: self.lab_id().to_string(),
            profile: self.profile().name.clone(),
            packs: self.packs().iter().map(|p| p.name.clone()).collect(),
            spine_kind: self.spine().kind().to_string(),
            grade,
            publication_grade: grade.is_publication(),
            storage_warning: grade.warning().map(|s| s.to_string()),
            storage_matrix: storage_matrix(),
            conformance_green: self.conformance().is_green(),
            total_acts,
            next_actions,
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
            kind: "logline.view.today.v0".to_string(),
            now: now.to_string(),
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
        TimelineView {
            kind: "logline.view.timeline.v0".to_string(),
            now: now.to_string(),
            past,
            present,
            future,
        }
    }

    /// Surface 5 (read model) — Schedule.
    pub fn schedule_view(&self, now: &str) -> ScheduleView {
        let mut scheduled = Vec::new();
        for s in self.spine().all() {
            let is_scheduled = s.act.status.as_str() == Some("scheduled")
                || s.act
                    .this
                    .get("due_at")
                    .and_then(|v| v.as_str())
                    .map(|d| d > now)
                    .unwrap_or(false);
            if is_scheduled && is_open(&s.act) {
                scheduled.push(s.content_hash.clone());
            }
        }
        let due: Vec<String> = due_work(self.spine(), now)
            .iter()
            .map(|a| a.content_hash().unwrap_or_default())
            .collect();
        let overdue: Vec<String> = overdue_work(self.spine(), now)
            .iter()
            .map(|a| a.content_hash().unwrap_or_default())
            .collect();
        let blocked: Vec<String> = self
            .spine()
            .all()
            .iter()
            .filter(|s| {
                let scope = s.act.did.as_str().unwrap_or("");
                let ctx = BlockContext { evidence: self.evidence(), permitted: true };
                logline_lab_core::evaluate_blocked(&s.act, scope, &ctx).is_some()
            })
            .map(|s| s.content_hash.clone())
            .collect();
        ScheduleView {
            kind: "logline.view.schedule.v0".to_string(),
            now: now.to_string(),
            scheduled,
            due,
            overdue,
            blocked,
        }
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
                self.push_candidate(value.clone())?;
                Ok(WriteOutcome::Candidate { missing })
            }
        }
    }

    /// Surface 5 (action) — Schedule: place an Act as a future obligation.
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

    /// Surface 6 — Workbench: run a study bench, turning observation into
    /// evidence or a ghost.
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
            kind: "logline.view.workbench.v0".to_string(),
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
            kind: "logline.view.proof.v0".to_string(),
            scope: scope.to_string(),
            claim_act_hash: act.content_hash().unwrap_or_default(),
            evidence_count,
            has_receipt_candidate,
            open_ghosts,
            note: "claim, evidence, receipt candidate, and ghost are separate; a report or model text is never proof".to_string(),
        }
    }

    /// Tick the clock: confront time and materialize it as Acts. Evaluates due
    /// work, emits a `clock_tick` Act, a `due_disposition` Act per due Act
    /// (executable/blocked with reason), and a `reschedule_act` for each overdue
    /// Act. Under a `candidate-only` Lab it evaluates but does not emit.
    pub fn tick(&mut self, now: &str, next_due_at: &str) -> Result<TickReport, LabError> {
        let materialized = self.admission_grade() != Grade::CandidateOnly;

        // Evaluate (read) first — these don't depend on emission.
        let due_acts = due_work(self.spine(), now);
        let mut due = Vec::new();
        for a in &due_acts {
            let scope = a.did.as_str().unwrap_or("");
            let ctx = BlockContext { evidence: self.evidence(), permitted: true };
            let (disposition, reason) =
                match logline_lab_core::evaluate_blocked(a, scope, &ctx) {
                    Some(b) => ("blocked", Some(format!("{:?}", b.reason))),
                    None => ("executable", None),
                };
            due.push(DueDisposition {
                content_hash: a.content_hash().unwrap_or_default(),
                disposition: disposition.to_string(),
                reason,
            });
        }
        let overdue_acts = overdue_work(self.spine(), now);
        let overdue: Vec<String> = overdue_acts
            .iter()
            .map(|a| a.content_hash().unwrap_or_default())
            .collect();
        let executable = due.iter().filter(|d| d.disposition == "executable").count();
        let cap = capacity(self.spine(), now, executable);

        let mut acts_emitted = 0usize;
        let mut tick_act = None;
        let mut rescheduled = Vec::new();

        if materialized {
            // 1. The tick itself becomes an Act.
            let tick = Act::new(
                json!("lab.clock"),
                json!("clock_tick"),
                json!({ "now": now, "due": due.len(), "overdue": overdue.len() }),
                json!(now),
                json!("system"),
                json!("record_tick"),
                json!("carry"),
                json!("skip"),
                json!("admitted"),
            );
            tick_act = Some(self.emit(&tick)?.content_hash().to_string());
            acts_emitted += 1;

            // 2. Each due Act gets an explicit disposition Act (nothing skipped).
            for d in &due {
                let disp = Act::new(
                    json!("lab.clock"),
                    json!("due_disposition"),
                    json!({ "target": d.content_hash, "disposition": d.disposition, "reason": d.reason }),
                    json!(now),
                    json!("system"),
                    json!("act_on_disposition"),
                    json!("carry_as_blocked_act"),
                    json!("skip"),
                    json!("admitted"),
                );
                self.emit(&disp)?;
                acts_emitted += 1;
            }

            // 3. Overdue Acts get reschedule Acts.
            for a in &overdue_acts {
                let r = reschedule(a, next_due_at, now);
                self.emit(&r)?;
                rescheduled.push(r.content_hash().unwrap_or_default());
                acts_emitted += 1;
            }
        }

        Ok(TickReport {
            kind: "logline.ruler_report.v0".to_string(),
            now: now.to_string(),
            tick_act,
            due,
            overdue,
            rescheduled,
            capacity: cap.state,
            next_study: cap.next_study_proposal,
            acts_emitted,
            materialized,
        })
    }

    /// Surface 8 — Learn (delegates to the learning report).
    pub fn learn(&self, now: &str) -> logline_lab_reports::LearningReport {
        self.learning(now)
    }

    /// Surface 9 — Settings. Read-only view; authority is always locked.
    pub fn settings(&self) -> SettingsView {
        SettingsView {
            kind: "logline.view.settings.v0".to_string(),
            profile: self.profile().name.clone(),
            spine: self.profile().spine.clone(),
            packs: self.packs().iter().map(|p| p.name.clone()).collect(),
            authority_locked: true,
        }
    }

    /// Render a named read-only surface as JSON — the single contract a CLI, MCP
    /// tool, or GUI consumes so everyone sees the same reality.
    pub fn render_surface(&self, surface: &str, now: &str) -> Option<Value> {
        let v = match surface {
            "start" => serde_json::to_value(self.start()),
            "today" => serde_json::to_value(self.today(now)),
            "timeline" => serde_json::to_value(self.timeline(now)),
            "schedule" => serde_json::to_value(self.schedule_view(now)),
            "learn" => serde_json::to_value(self.learn(now)),
            "settings" => serde_json::to_value(self.settings()),
            _ => return None,
        };
        v.ok()
    }

    /// The set of read-only surfaces that can be rendered headlessly.
    pub fn read_surfaces() -> &'static [&'static str] {
        &["start", "today", "timeline", "schedule", "learn", "settings"]
    }

    /// Count of currently open (non-terminal) Acts on the spine.
    pub fn open_act_count(&self) -> usize {
        self.spine().all().iter().filter(|s| is_open(&s.act)).count()
    }
}
