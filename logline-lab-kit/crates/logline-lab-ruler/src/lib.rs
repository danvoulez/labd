//! `logline-lab-ruler` — the Lab's confrontation with time.
//!
//! The ruler compares the current time against the Lab's Acts and answers: what
//! is due, overdue, blocked, runnable, waiting, or needs evidence; and whether
//! the Lab is harmfully idle (FINAL §10.2). It is a necessary component, not the
//! product identity. Crucially, **no due Act is skipped silently** (A25): every
//! due Act receives an explicit disposition, and idleness is surfaced honestly
//! rather than filled with fake busywork (A26).

#![forbid(unsafe_code)]

use logline_act::Act;
use logline_lab_clock::is_due;
use logline_lab_spine::Spine;
use serde::{Deserialize, Serialize};

/// The explicit disposition of a due Act. There is no "skipped" variant: a due
/// Act is always visibly executable, blocked, or to-be-rescheduled.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "disposition", rename_all = "snake_case")]
pub enum Disposition {
    Executable { content_hash: String },
    Blocked { content_hash: String, reason: String },
    Reschedule { content_hash: String },
}

impl Disposition {
    pub fn content_hash(&self) -> &str {
        match self {
            Disposition::Executable { content_hash }
            | Disposition::Blocked { content_hash, .. }
            | Disposition::Reschedule { content_hash } => content_hash,
        }
    }
}

/// Capacity state of the Lab.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapacityState {
    /// Harmful idleness: nothing executable and nothing pending.
    Under,
    /// There is legitimate work in flight or waiting.
    Healthy,
}

/// Capacity report. When under-capacity, it proposes a *legitimate* next study,
/// never fabricated busywork.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityReport {
    pub state: CapacityState,
    pub executable_work: usize,
    pub pending_due: usize,
    /// A real next-study proposal when idle; `None` when there is work.
    pub next_study_proposal: Option<String>,
}

/// A full ruler report over a Lab's spine at `now`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RulerReport {
    pub now: String,
    pub due: Vec<Disposition>,
    pub overdue: Vec<String>,
    pub capacity: CapacityReport,
}

/// An Act is "open" when its status is not a terminal state.
fn is_open(act: &Act) -> bool {
    !matches!(
        act.status.as_str(),
        Some("done") | Some("closed") | Some("resolved") | Some("failed")
    )
}

/// All open, due Acts at `now` (A23).
pub fn due_work(spine: &dyn Spine, now: &str) -> Vec<Act> {
    spine
        .all()
        .into_iter()
        .map(|s| s.act)
        .filter(|a| is_due(a, now))
        .collect()
}

/// Open Acts that became due strictly before `before` and are still open (overdue).
pub fn overdue_work(spine: &dyn Spine, before: &str) -> Vec<Act> {
    spine
        .all()
        .into_iter()
        .map(|s| s.act)
        .filter(|a| {
            is_open(a)
                && a.this
                    .get("due_at")
                    .and_then(|v| v.as_str())
                    .map(|d| d < before)
                    .unwrap_or(false)
        })
        .collect()
}

/// Evaluate every due Act, returning one explicit disposition each (A24/A25).
///
/// `executable` decides, per Act, whether the Lab may run it now (gate/permission
/// and evidence policy live with the caller). Acts that are not executable and
/// not blocked are marked for reschedule. The returned vector length always
/// equals the number of due Acts, so nothing is dropped.
pub fn evaluate_due<F>(spine: &dyn Spine, now: &str, mut executable: F) -> Vec<Disposition>
where
    F: FnMut(&Act) -> DueDecision,
{
    due_work(spine, now)
        .iter()
        .map(|act| {
            let hash = act.content_hash().unwrap_or_default();
            match executable(act) {
                DueDecision::Run => Disposition::Executable { content_hash: hash },
                DueDecision::Block(reason) => Disposition::Blocked {
                    content_hash: hash,
                    reason,
                },
                DueDecision::Reschedule => Disposition::Reschedule { content_hash: hash },
            }
        })
        .collect()
}

/// Per-Act decision supplied to `evaluate_due`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DueDecision {
    Run,
    Block(String),
    Reschedule,
}

/// Capacity band (A26). Idleness is surfaced honestly with a real next-study
/// proposal; it is never filled with fake work.
pub fn capacity(spine: &dyn Spine, now: &str, executable_work: usize) -> CapacityReport {
    let pending_due = due_work(spine, now).len();
    if executable_work == 0 && pending_due == 0 {
        CapacityReport {
            state: CapacityState::Under,
            executable_work,
            pending_due,
            next_study_proposal: Some(
                "no executable work and no due obligation — propose a study bench \
                 (e.g. candidate-promotion or proof-discipline) rather than fabricate work"
                    .to_string(),
            ),
        }
    } else {
        CapacityReport {
            state: CapacityState::Healthy,
            executable_work,
            pending_due,
            next_study_proposal: None,
        }
    }
}

/// Build a full ruler report.
pub fn report<F>(spine: &dyn Spine, now: &str, overdue_before: &str, executable: F) -> RulerReport
where
    F: FnMut(&Act) -> DueDecision,
{
    let due = evaluate_due(spine, now, executable);
    let executable_count = due
        .iter()
        .filter(|d| matches!(d, Disposition::Executable { .. }))
        .count();
    RulerReport {
        now: now.to_string(),
        due,
        overdue: overdue_work(spine, overdue_before)
            .iter()
            .map(|a| a.content_hash().unwrap_or_default())
            .collect(),
        capacity: capacity(spine, now, executable_count),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logline_lab_local::LocalOutbox;
    use logline_lab_spine::{sync, MemorySpine};
    use serde_json::json;

    fn scheduled(did: &str, due_at: &str, status: &str) -> Act {
        Act::new(
            json!("lab.clock"),
            json!(did),
            json!({ "due_at": due_at }),
            json!("2026-06-06T00:00:00Z"),
            json!("system"),
            json!("run"),
            json!("carry_as_blocked_act"),
            json!("skip"),
            json!(status),
        )
    }

    fn spine_with(acts: Vec<Act>) -> MemorySpine {
        let mut outbox = LocalOutbox::in_memory();
        for a in &acts {
            outbox.emit(a).unwrap();
        }
        let mut spine = MemorySpine::new();
        sync(&mut outbox, &mut spine).unwrap();
        spine
    }

    /// A23 — Tick/check discovers due work.
    #[test]
    fn a23_discovers_due_work() {
        let spine = spine_with(vec![
            scheduled("check_a", "2026-06-06T00:00:00Z", "pending"),
            scheduled("check_b", "2999-01-01T00:00:00Z", "pending"),
        ]);
        assert_eq!(due_work(&spine, "2026-06-06T12:00:00Z").len(), 1);
    }

    /// A24/A25 — Due work resolves/blocks/reschedules visibly; none skipped.
    #[test]
    fn a24_a25_every_due_act_has_a_disposition() {
        let spine = spine_with(vec![
            scheduled("run_me", "2026-06-06T00:00:00Z", "pending"),
            scheduled("block_me", "2026-06-06T00:00:00Z", "pending"),
            scheduled("later", "2026-06-06T00:00:00Z", "pending"),
        ]);
        let now = "2026-06-06T12:00:00Z";
        let due_count = due_work(&spine, now).len();
        let dispositions = evaluate_due(&spine, now, |a| match a.did.as_str() {
            Some("run_me") => DueDecision::Run,
            Some("block_me") => DueDecision::Block("missing permission".into()),
            _ => DueDecision::Reschedule,
        });
        // A25: every due Act is accounted for — nothing skipped silently.
        assert_eq!(dispositions.len(), due_count);
        assert_eq!(due_count, 3);
        assert!(dispositions.iter().any(|d| matches!(d, Disposition::Executable { .. })));
        assert!(dispositions.iter().any(|d| matches!(d, Disposition::Blocked { .. })));
        assert!(dispositions.iter().any(|d| matches!(d, Disposition::Reschedule { .. })));
    }

    /// A26 — Capacity report surfaces harmful idleness without fake busywork.
    #[test]
    fn a26_capacity_surfaces_idleness_honestly() {
        let idle = spine_with(vec![]);
        let cap = capacity(&idle, "2026-06-06T12:00:00Z", 0);
        assert_eq!(cap.state, CapacityState::Under);
        assert!(cap.next_study_proposal.is_some());

        let busy = spine_with(vec![scheduled("run_me", "2026-06-06T00:00:00Z", "pending")]);
        let cap = capacity(&busy, "2026-06-06T12:00:00Z", 1);
        assert_eq!(cap.state, CapacityState::Healthy);
        assert!(cap.next_study_proposal.is_none());
    }
}
