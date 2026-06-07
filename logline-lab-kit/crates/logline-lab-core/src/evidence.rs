//! Evidence machinery.
//!
//! Evidence is proof captured against a *scope*. A receipt may close only what
//! evidence proves (Operator §14). Evidence is real captured data, never a bare
//! claim (Operator §6: "runtime claim without runtime evidence" is rejected).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A single piece of evidence attached to a scope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Evidence {
    /// The exact scope this evidence supports (e.g. an Act content hash or a
    /// pack-defined scope like `manhattan.L-06`).
    pub scope: String,
    /// What kind of evidence (e.g. `command_output`, `probe`, `attachment`).
    pub kind: String,
    /// The captured payload. Must be real data, not an assertion of success.
    pub payload: Value,
    /// When the evidence was captured (RFC3339).
    pub captured_at: String,
}

impl Evidence {
    pub fn new(
        scope: impl Into<String>,
        kind: impl Into<String>,
        payload: Value,
        captured_at: impl Into<String>,
    ) -> Self {
        Self {
            scope: scope.into(),
            kind: kind.into(),
            payload,
            captured_at: captured_at.into(),
        }
    }
}

/// An append-only log of evidence. This is a cache/index around Acts, not a
/// source of semantic truth.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EvidenceLog {
    items: Vec<Evidence>,
}

impl EvidenceLog {
    pub fn new() -> Self {
        Self::default()
    }

    /// Attach evidence to its scope (A11).
    pub fn attach(&mut self, evidence: Evidence) {
        self.items.push(evidence);
    }

    /// All evidence captured for an exact scope.
    pub fn for_scope(&self, scope: &str) -> Vec<&Evidence> {
        self.items.iter().filter(|e| e.scope == scope).collect()
    }

    /// True when at least one piece of evidence exists for the scope.
    pub fn has_for_scope(&self, scope: &str) -> bool {
        self.items.iter().any(|e| e.scope == scope)
    }

    pub fn all(&self) -> &[Evidence] {
        &self.items
    }
}
