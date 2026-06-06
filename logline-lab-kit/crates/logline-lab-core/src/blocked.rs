//! Blocked Act machinery.
//!
//! An Act is blocked — not failed — when a consequence cannot proceed because a
//! confirmation, a permission, or required evidence is missing. A blocked Act is
//! carried openly, never silently closed (Operator §13: no fake closure).

use logline_act::Act;
use serde::{Deserialize, Serialize};

use crate::evidence::EvidenceLog;

/// Why an Act is blocked.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockReason {
    MissingConfirmation,
    MissingPermission,
    MissingEvidence,
}

/// A blocked Act: the Act plus the open reason and its scope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockedAct {
    pub scope: String,
    pub reason: BlockReason,
    pub act: Act,
}

/// Context for evaluating whether an Act is blocked.
pub struct BlockContext<'a> {
    /// Evidence available so far.
    pub evidence: &'a EvidenceLog,
    /// Whether the acting party holds permission for this Act.
    pub permitted: bool,
}

/// Evaluate an Act against context (A10). Returns `Some(BlockedAct)` when a
/// confirmation, permission, or evidence is missing; `None` when it may proceed.
///
/// Rules:
/// - `confirmed_by` of `none`/empty + a non-`candidate` status ⇒ missing confirmation.
/// - acting party not permitted ⇒ missing permission.
/// - confirmation names `evidence` but none attached for the scope ⇒ missing evidence.
pub fn evaluate(act: &Act, scope: &str, ctx: &BlockContext) -> Option<BlockedAct> {
    if !ctx.permitted {
        return Some(BlockedAct {
            scope: scope.to_string(),
            reason: BlockReason::MissingPermission,
            act: act.clone(),
        });
    }

    let confirmed_by = act.confirmed_by.as_str().unwrap_or("");
    let status = act.status.as_str().unwrap_or("");

    if confirmed_by == "evidence" && !ctx.evidence.has_for_scope(scope) {
        return Some(BlockedAct {
            scope: scope.to_string(),
            reason: BlockReason::MissingEvidence,
            act: act.clone(),
        });
    }

    let unconfirmed = confirmed_by.is_empty() || confirmed_by == "none";
    if unconfirmed && status != "candidate" && status != "draft" {
        return Some(BlockedAct {
            scope: scope.to_string(),
            reason: BlockReason::MissingConfirmation,
            act: act.clone(),
        });
    }

    None
}
