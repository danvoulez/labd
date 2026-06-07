//! Study bench machinery — the scientific method as Lab bureaucracy.
//!
//! A study bench is a repeatable practice for testing a thesis (FINAL §9.2). It
//! is **not** a semantic primitive: it is a convention that *produces Acts*. The
//! bench declares a question, hypothesis, probe, expected observation, and a
//! receipt scope; running it yields Acts and an outcome that becomes either
//! evidence or a ghost (FINAL §9, A18–A21).

use logline_act::Act;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::evidence::Evidence;
use crate::ghost::Ghost;

/// A declared study bench.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyBench {
    pub id: String,
    pub question: String,
    pub hypothesis: String,
    #[serde(default)]
    pub inputs: serde_json::Value,
    pub procedure: String,
    pub expected: String,
    #[serde(default)]
    pub success_criteria: String,
    #[serde(default)]
    pub failure_criteria: String,
    /// The exact scope a receipt could close if the bench succeeds.
    pub receipt_scope: String,
    /// What to do when proof is missing.
    #[serde(default)]
    pub ghost_policy: String,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
}

/// The outcome of an observation against a bench.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BenchOutcome {
    /// Expectation met and proof captured.
    Evidence(Evidence),
    /// Proof missing or expectation unmet: a ghost is preserved, never fake closure.
    Ghost(Ghost),
}

impl StudyBench {
    pub fn load(text: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(text)
    }

    /// True iff the bench declares the minimum scientific fields (A18).
    pub fn is_well_formed(&self) -> bool {
        !self.question.trim().is_empty()
            && !self.hypothesis.trim().is_empty()
            && !self.procedure.trim().is_empty()
            && !self.expected.trim().is_empty()
            && !self.receipt_scope.trim().is_empty()
    }

    /// The Acts a bench emits when declared: a hypothesis Act and a probe-design
    /// Act (A19). Both are valid nine-slot Acts.
    pub fn declare_acts(&self, who: &str, when: &str) -> Vec<Act> {
        let hypothesis = Act::new(
            json!(who),
            json!("proposed"),
            json!({ "bench": self.id, "hypothesis": self.hypothesis, "question": self.question }),
            json!(when),
            json!("pending"),
            json!(format!("schedule_study_bench:{}", self.id)),
            json!("open_blocked_act_missing_conformance"),
            json!("record_contradicted_hypothesis"),
            json!("candidate"),
        );
        let probe = Act::new(
            json!(who),
            json!("designed_probe"),
            json!({ "bench": self.id, "procedure": self.procedure, "expected": self.expected }),
            json!(when),
            json!("pending"),
            json!("run_probe"),
            json!("carry_as_blocked_act"),
            json!("abandon_probe"),
            json!("candidate"),
        );
        vec![hypothesis, probe]
    }

    /// Turn an observation into evidence or a ghost (A20).
    ///
    /// `expectation_met` is the verdict of comparing the actual result against
    /// `expected`; `observed` is the captured payload. When the expectation is
    /// met and real data was observed, the bench yields evidence scoped to its
    /// receipt scope. Otherwise it yields a ghost — named missing proof.
    pub fn observe(
        &self,
        expectation_met: bool,
        observed: serde_json::Value,
        when: &str,
    ) -> BenchOutcome {
        let has_data = !observed.is_null();
        if expectation_met && has_data {
            BenchOutcome::Evidence(Evidence::new(
                self.receipt_scope.clone(),
                "bench_observation",
                observed,
                when,
            ))
        } else {
            let reason = if !has_data {
                "no observation captured"
            } else {
                "observation did not meet expectation"
            };
            BenchOutcome::Ghost(Ghost::new(
                format!("ghost:{}", self.id),
                self.receipt_scope.clone(),
                reason,
                format!("re-run bench {} until expectation is met with evidence", self.id),
            ))
        }
    }
}
