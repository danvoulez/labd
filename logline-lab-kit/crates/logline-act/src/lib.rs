//! `logline-act` — the canonical nine-slot LogLine Act, candidate mode,
//! canonical JSON, and content-addressed hashing.
//!
//! This crate is the semantic floor of LogLine Lab Kit. Everything consequential
//! starts as an Act (Operator §5). The Act has exactly nine slots; there is no
//! tenth slot, and storage/runtime/envelope data never live inside it.

#![forbid(unsafe_code)]

mod act;
mod canonical;
mod envelope;
mod error;

pub use act::{validate_object, Act, Candidate};
pub use canonical::{canonical_json, content_hash, envelope_hash, tuple_hash, SLOTS};
pub use envelope::{verify_envelope_value, Envelope, TransportMeta};
pub use error::ActError;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn valid_value() -> serde_json::Value {
        json!({
            "who": "lab.operator",
            "did": "declare_lab",
            "this": {"lab_id": "test.local.lab"},
            "when": "2026-06-06T00:00:00Z",
            "confirmed_by": "none",
            "if_ok": "record_declaration",
            "if_doubt": "carry_as_blocked_act",
            "if_not": "reject_declaration",
            "status": "candidate"
        })
    }

    /// A1 — Act with exactly nine slots validates.
    #[test]
    fn a1_nine_slots_validate() {
        let act = Act::from_value_strict(&valid_value()).expect("nine-slot act validates");
        assert!(act.is_valid());
        assert_eq!(act.missing_slots(), Vec::<&str>::new());
    }

    /// A2 — Act with a missing slot fails.
    #[test]
    fn a2_missing_slot_fails() {
        let mut v = valid_value();
        v.as_object_mut().unwrap().remove("confirmed_by");
        let err = Act::from_value_strict(&v).unwrap_err();
        assert!(matches!(err, ActError::MissingSlots(_)), "got {err:?}");
    }

    /// A2b — present-but-empty slot also counts as missing.
    #[test]
    fn a2_empty_slot_fails() {
        let mut v = valid_value();
        v["who"] = json!("   ");
        let err = Act::from_value_strict(&v).unwrap_err();
        assert!(matches!(err, ActError::MissingSlots(_)), "got {err:?}");
    }

    /// A3 — Act with a tenth canonical slot fails.
    #[test]
    fn a3_tenth_slot_fails() {
        let mut v = valid_value();
        v["extra_slot"] = json!("not allowed");
        let err = Act::from_value_strict(&v).unwrap_err();
        assert!(matches!(err, ActError::ExtraSlots(_)), "got {err:?}");

        // And the strict JSON parser rejects it too (deny_unknown_fields).
        let text = serde_json::to_string(&v).unwrap();
        assert!(Act::from_json_strict(&text).is_err());
    }

    /// A4 — Same Act produces same canonical hash.
    #[test]
    fn a4_same_act_same_hash() {
        let a = Act::from_value_strict(&valid_value()).unwrap();
        // Reorder the keys in the source JSON: canonicalization must absorb it.
        let reordered = json!({
            "status": "candidate",
            "if_not": "reject_declaration",
            "if_doubt": "carry_as_blocked_act",
            "if_ok": "record_declaration",
            "confirmed_by": "none",
            "when": "2026-06-06T00:00:00Z",
            "this": {"lab_id": "test.local.lab"},
            "did": "declare_lab",
            "who": "lab.operator"
        });
        let b = Act::from_value_strict(&reordered).unwrap();
        assert_eq!(a.content_hash().unwrap(), b.content_hash().unwrap());
        assert_eq!(a.tuple_hash().unwrap(), b.tuple_hash().unwrap());
        // Hash is stable across repeated computation.
        assert_eq!(a.content_hash().unwrap(), a.content_hash().unwrap());
    }

    /// A5 — Ugly candidate can be preserved.
    #[test]
    fn a5_ugly_candidate_preserved() {
        let ugly = json!({
            "who": "",
            "did": "scribbled_thought",
            "this": "raw unfinished note",
            "status": "candidate"
            // missing when/confirmed_by/if_ok/if_doubt/if_not
        });
        // Strict validation rejects it...
        assert!(Act::from_value_strict(&ugly).is_err());
        // ...but candidate mode preserves it verbatim, nothing dropped.
        let cand = Act::candidate_from_value(&ugly);
        assert_eq!(cand.value(), &ugly);
        assert!(!cand.missing_slots().is_empty());
        // It cannot be promoted until repaired.
        assert!(cand.promote().is_err());
    }
}
