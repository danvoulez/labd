//! `logline-lab-clock` — tick, due checks, and reschedule.
//!
//! The clock evaluates which Acts are due and either lets them resolve or emits
//! a *reschedule* Act. Time is read; it is not invented inside an Act. An Act is
//! considered schedulable when its `this` carries a `due_at` (RFC3339 string)
//! and its status is still open.

#![forbid(unsafe_code)]

use logline_act::Act;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// True when an Act is due at `now`: it carries `this.due_at <= now` and is not
/// already resolved/closed.
pub fn is_due(act: &Act, now: &str) -> bool {
    let open = !matches!(act.status.as_str(), Some("done") | Some("closed") | Some("resolved"));
    let due_at = act.this.get("due_at").and_then(|v| v.as_str());
    match due_at {
        Some(d) => open && d <= now,
        None => false,
    }
}

/// All due Acts at `now`.
pub fn due_acts<'a>(acts: &'a [Act], now: &str) -> Vec<&'a Act> {
    acts.iter().filter(|a| is_due(a, now)).collect()
}

/// Outcome of evaluating one due Act.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum DueOutcome {
    /// The due Act resolved this tick.
    Resolved { content_hash: String },
    /// The due Act could not resolve and a reschedule Act was created.
    Rescheduled { reschedule: Box<Act> },
}

/// Report of a tick.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TickReport {
    pub evaluated: usize,
    pub outcomes: Vec<DueOutcome>,
}

/// Create a reschedule Act for a due Act that could not resolve (A21). The
/// reschedule is itself a normal nine-slot Act pointing at the original.
pub fn reschedule(act: &Act, new_due_at: &str, now: &str) -> Act {
    let original = act.content_hash().unwrap_or_default();
    Act::new(
        act.who.clone(),
        json!("reschedule_act"),
        json!({ "rescheduled_from": original, "due_at": new_due_at }),
        json!(now),
        json!("system"),
        json!("carry_new_due"),
        json!("carry_as_blocked_act"),
        json!("abandon_schedule"),
        json!("candidate"),
    )
}

/// Tick the clock: evaluate every due Act. `resolved` decides, per Act, whether
/// it resolves now; unresolved due Acts produce reschedule Acts (A20/A21).
pub fn tick<F>(acts: &[Act], now: &str, next_due_at: &str, mut resolved: F) -> TickReport
where
    F: FnMut(&Act) -> bool,
{
    let mut report = TickReport::default();
    for act in due_acts(acts, now) {
        report.evaluated += 1;
        if resolved(act) {
            report.outcomes.push(DueOutcome::Resolved {
                content_hash: act.content_hash().unwrap_or_default(),
            });
        } else {
            report.outcomes.push(DueOutcome::Rescheduled {
                reschedule: Box::new(reschedule(act, next_due_at, now)),
            });
        }
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scheduled_act(due_at: &str, status: &str) -> Act {
        Act::new(
            json!("lab.clock"),
            json!("scheduled_check"),
            json!({ "due_at": due_at, "target": "tunnel" }),
            json!("2026-06-06T00:00:00Z"),
            json!("system"),
            json!("run_check"),
            json!("carry_as_blocked_act"),
            json!("skip"),
            json!(status),
        )
    }

    /// A20 — Clock tick emits or evaluates due Acts.
    #[test]
    fn a20_tick_evaluates_due_acts() {
        let acts = vec![
            scheduled_act("2026-06-06T00:00:00Z", "pending"), // due
            scheduled_act("2999-01-01T00:00:00Z", "pending"), // not yet due
        ];
        let now = "2026-06-06T12:00:00Z";
        assert_eq!(due_acts(&acts, now).len(), 1);

        let report = tick(&acts, now, "2026-06-07T00:00:00Z", |_| true);
        assert_eq!(report.evaluated, 1);
        assert!(matches!(report.outcomes[0], DueOutcome::Resolved { .. }));
    }

    /// A21 — Due Act resolves or creates reschedule Act.
    #[test]
    fn a21_unresolved_due_act_reschedules() {
        let acts = vec![scheduled_act("2026-06-06T00:00:00Z", "pending")];
        let now = "2026-06-06T12:00:00Z";
        let report = tick(&acts, now, "2026-06-07T00:00:00Z", |_| false);
        assert_eq!(report.evaluated, 1);
        match &report.outcomes[0] {
            DueOutcome::Rescheduled { reschedule } => {
                assert_eq!(reschedule.did.as_str(), Some("reschedule_act"));
                assert_eq!(reschedule.this.get("due_at").unwrap(), "2026-06-07T00:00:00Z");
                assert!(reschedule.is_valid());
            }
            other => panic!("expected reschedule, got {other:?}"),
        }
    }

    /// A resolved/closed Act is never due.
    #[test]
    fn closed_act_not_due() {
        let act = scheduled_act("2020-01-01T00:00:00Z", "done");
        assert!(!is_due(&act, "2026-06-06T00:00:00Z"));
    }
}
