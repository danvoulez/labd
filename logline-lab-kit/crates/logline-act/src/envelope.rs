//! Transport Envelope (LIP-0007) — the boundary-crossing wrapper.
//!
//! An Envelope is **not** a new truth layer and **not** a tenth slot. It wraps a
//! content payload (a receipt/Act projection) with transport metadata and an
//! `envelope_hash` so a receiver can verify the crossing before accepting. Truth
//! lives in the content's own hashes; the envelope only attests *who sent what to
//! whom, when*.
//!
//! Invariants (canon):
//! - `envelope_hash` lives ONLY on the wrapper — never inside the content.
//! - transport metadata never becomes content/Act semantics.
//! - the envelope verifies the boundary crossing, not the truth of the content.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::canonical::envelope_hash;
use crate::error::ActError;

/// Transport metadata: who sent the package to whom, when, and (optionally) by which
/// channel. Extra fields are permitted by the canon; for fidelity, verify incoming
/// envelopes with [`verify_envelope_value`] (raw) rather than round-tripping through
/// this struct, which only models the known fields.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportMeta {
    pub sent_by: String,
    pub sent_to: String,
    pub sent_at: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub channel: Option<String>,
}

/// A sealed transport envelope: `{content, transport, envelope_hash}`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    pub content: Value,
    pub transport: TransportMeta,
    pub envelope_hash: String,
}

impl Envelope {
    /// **Sender** side: wrap `content` + `transport` and compute the `envelope_hash`
    /// over the canonical `{content, transport}`. Transport never enters the content.
    pub fn seal(content: Value, transport: TransportMeta) -> Result<Self, ActError> {
        let to_hash = json!({ "content": content, "transport": transport });
        let hash = envelope_hash(&to_hash)?;
        Ok(Self { content, transport, envelope_hash: hash })
    }

    /// **Receiver** side: recompute and confirm the `envelope_hash` before accepting.
    /// Returns the mismatch (recomputed vs declared) on failure.
    pub fn verify(&self) -> Result<(), ActError> {
        let to_hash = json!({ "content": self.content, "transport": self.transport });
        let recomputed = envelope_hash(&to_hash)?;
        if recomputed == self.envelope_hash {
            Ok(())
        } else {
            Err(ActError::EnvelopeHashMismatch {
                recomputed,
                declared: self.envelope_hash.clone(),
            })
        }
    }
}

/// Verify a raw incoming envelope value (preserves any extra transport fields in the
/// hash). The value must be an object carrying `envelope_hash`. This is the
/// fidelity-preserving receiver check used for arbitrary, externally-produced
/// envelopes (e.g. conformance vectors).
pub fn verify_envelope_value(value: &Value) -> Result<(), ActError> {
    let declared = value
        .get("envelope_hash")
        .and_then(Value::as_str)
        .ok_or(ActError::NotAnObject)?
        .to_string();
    let recomputed = envelope_hash(value)?;
    if recomputed == declared {
        Ok(())
    } else {
        Err(ActError::EnvelopeHashMismatch { recomputed, declared })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn transport() -> TransportMeta {
        TransportMeta {
            sent_by: "lab256".into(),
            sent_to: "supabase".into(),
            sent_at: "2026-05-17T11:00:00Z".into(),
            channel: Some("supabase.rpc".into()),
        }
    }

    #[test]
    fn seal_then_verify_round_trips() {
        let content = json!({ "who": "dan", "did": "rested" });
        let env = Envelope::seal(content, transport()).unwrap();
        assert_eq!(env.envelope_hash.len(), 64);
        env.verify().expect("freshly sealed envelope verifies");
    }

    #[test]
    fn tampered_content_fails_verification() {
        let env = Envelope::seal(json!({ "who": "dan" }), transport()).unwrap();
        let mut tampered = env.clone();
        tampered.content = json!({ "who": "mallory" });
        assert!(matches!(
            tampered.verify(),
            Err(ActError::EnvelopeHashMismatch { .. })
        ));
    }

    #[test]
    fn envelope_hash_is_not_inside_content() {
        // The content is whatever the caller passed; the wrapper carries the hash.
        let content = json!({ "who": "dan", "did": "rested" });
        let env = Envelope::seal(content.clone(), transport()).unwrap();
        assert_eq!(env.content, content);
        assert!(env.content.get("envelope_hash").is_none());
    }
}
