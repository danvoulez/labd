//! Receipt candidate machinery.
//!
//! A receipt candidate names the *exact* scope it would close and references the
//! evidence that proves it. "Receipts close only what was proven" (Operator §14).
//! Preparing a receipt candidate without evidence is rejected (Operator §6:
//! "receipt without evidence").

use logline_act::Act;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::evidence::EvidenceLog;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReceiptError {
    #[error("cannot prepare a receipt candidate for scope `{0}` without evidence")]
    NoEvidence(String),
    #[error("act hashing failed: {0}")]
    Hash(String),
}

/// A scoped receipt candidate. It is *not* a receipt: it is a proposal that
/// names the scope and the evidence, ready to be sealed only if the proof holds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceiptCandidate {
    pub kind: String,
    /// The exact scope this candidate would close — and nothing else.
    pub scope: String,
    /// Content hash of the Act being closed.
    pub act_content_hash: String,
    /// Content hashes / identifiers of the evidence supporting the scope.
    pub evidence_kinds: Vec<String>,
    /// Number of evidence items backing this candidate.
    pub evidence_count: usize,
}

impl ReceiptCandidate {
    /// Prepare a receipt candidate for an exact scope (A12). Requires at least
    /// one piece of evidence attached to that scope.
    pub fn prepare(
        act: &Act,
        scope: &str,
        evidence: &EvidenceLog,
    ) -> Result<Self, ReceiptError> {
        let items = evidence.for_scope(scope);
        if items.is_empty() {
            return Err(ReceiptError::NoEvidence(scope.to_string()));
        }
        let act_content_hash = act
            .content_hash()
            .map_err(|e| ReceiptError::Hash(e.to_string()))?;
        Ok(Self {
            kind: "logline.receipt_candidate.v0".to_string(),
            scope: scope.to_string(),
            act_content_hash,
            evidence_kinds: items.iter().map(|e| e.kind.clone()).collect(),
            evidence_count: items.len(),
        })
    }

    /// True iff this candidate closes exactly `scope` and no other.
    pub fn closes_only(&self, scope: &str) -> bool {
        self.scope == scope
    }
}
