//! Ghost machinery — named missing proof.
//!
//! A ghost is not a failure; it is honest, named, missing proof (FINAL §9.3). A
//! Lab must preserve doubt without pretending closure. A ghost may never be
//! silently closed (Operator §13: no fake closure); closing one requires meeting
//! its declared closure condition.

use serde::{Deserialize, Serialize};

/// A named missing proof carried openly by the Lab.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ghost {
    pub id: String,
    pub scope: String,
    /// What proof is missing.
    pub missing_proof: String,
    /// Who owns driving this to closure.
    #[serde(default)]
    pub owner: String,
    /// What the ghost blocks.
    #[serde(default)]
    pub blocks: String,
    /// The next concrete action toward closure.
    #[serde(default)]
    pub next_action: String,
    /// The condition under which the ghost may be closed.
    pub closure_condition: String,
    #[serde(default)]
    pub closed: bool,
}

impl Ghost {
    pub fn new(
        id: impl Into<String>,
        scope: impl Into<String>,
        missing_proof: impl Into<String>,
        closure_condition: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            scope: scope.into(),
            missing_proof: missing_proof.into(),
            owner: String::new(),
            blocks: String::new(),
            next_action: String::new(),
            closure_condition: closure_condition.into(),
            closed: false,
        }
    }

    /// Attempt to close the ghost. Closure requires explicit acknowledgement that
    /// the closure condition was met — a ghost is never closed by convenience.
    pub fn close(&mut self, closure_condition_met: bool) -> Result<(), &'static str> {
        if !closure_condition_met {
            return Err("ghost closure condition not met; cannot close (no fake closure)");
        }
        self.closed = true;
        Ok(())
    }
}

/// An open ledger of ghosts.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GhostLog {
    ghosts: Vec<Ghost>,
}

impl GhostLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, ghost: Ghost) {
        self.ghosts.push(ghost);
    }

    /// All ghosts still open.
    pub fn open(&self) -> Vec<&Ghost> {
        self.ghosts.iter().filter(|g| !g.closed).collect()
    }

    pub fn all(&self) -> &[Ghost] {
        &self.ghosts
    }

    pub fn for_scope(&self, scope: &str) -> Vec<&Ghost> {
        self.ghosts.iter().filter(|g| g.scope == scope).collect()
    }
}
