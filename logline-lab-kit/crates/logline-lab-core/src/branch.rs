//! Branch / verdict behavior.
//!
//! An Act carries three consequence slots (`if_ok`, `if_doubt`, `if_not`). The
//! *selected* branch is a runtime decision that lives around the Act, never
//! inside it (Operator §5). This module resolves a verdict to a consequence
//! without mutating the Act.

use logline_act::Act;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The three possible verdicts on an Act.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Ok,
    Doubt,
    Not,
}

/// Resolve which consequence slot applies for a verdict. Returns a clone of the
/// slot value; the Act is left untouched.
pub fn select_branch(act: &Act, verdict: Verdict) -> Value {
    match verdict {
        Verdict::Ok => act.if_ok.clone(),
        Verdict::Doubt => act.if_doubt.clone(),
        Verdict::Not => act.if_not.clone(),
    }
}
