//! `logline-lab-hooks` — the hook runner.
//!
//! Hooks react to Lab events (e.g. a clock tick, a daily report). A hook is a
//! side-effecting reaction, not an authority: it cannot close scopes or assert
//! truth. The runner is generic so packs/profiles can register their own hooks
//! without changing core.

#![forbid(unsafe_code)]

use serde_json::Value;

/// A Lab event a hook can react to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub name: String,
}

impl Event {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// What a hook produced.
#[derive(Clone, Debug, PartialEq)]
pub struct HookOutcome {
    pub hook: String,
    pub ran: bool,
    pub detail: Value,
}

/// A registered hook.
pub trait Hook {
    /// Hook name.
    fn name(&self) -> &str;
    /// Which event names this hook fires on.
    fn fires_on(&self, event: &Event) -> bool;
    /// Run the hook for an event.
    fn run(&self, event: &Event) -> Value;
}

/// Runs hooks for events.
#[derive(Default)]
pub struct HookRunner {
    hooks: Vec<Box<dyn Hook>>,
}

impl HookRunner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, hook: Box<dyn Hook>) {
        self.hooks.push(hook);
    }

    /// Dispatch an event to all hooks that fire on it.
    pub fn dispatch(&self, event: &Event) -> Vec<HookOutcome> {
        self.hooks
            .iter()
            .filter(|h| h.fires_on(event))
            .map(|h| HookOutcome {
                hook: h.name().to_string(),
                ran: true,
                detail: h.run(event),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct DailyReportHook;
    impl Hook for DailyReportHook {
        fn name(&self) -> &str {
            "daily_report"
        }
        fn fires_on(&self, event: &Event) -> bool {
            event.name == "clock.tick.daily"
        }
        fn run(&self, _event: &Event) -> Value {
            json!({"generated": true})
        }
    }

    #[test]
    fn hook_runner_dispatches_matching_hooks() {
        let mut runner = HookRunner::new();
        runner.register(Box::new(DailyReportHook));

        let out = runner.dispatch(&Event::new("clock.tick.daily"));
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].hook, "daily_report");

        // Non-matching event fires nothing.
        assert!(runner.dispatch(&Event::new("other.event")).is_empty());
    }
}
