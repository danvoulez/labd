//! The canonical LogLine Act.
//!
//! Exactly nine slots (Operator §5). Hashes, signatures, runtime data, selected
//! branch, storage timestamps, and envelopes live *around* the Act, never inside
//! it. There is no tenth slot.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::canonical::{content_hash, tuple_hash, SLOTS};
use crate::error::ActError;

/// The nine-slot canonical Act.
///
/// All slots are JSON values: `who`/`did`/`when`/`status` are conventionally
/// strings, while `this`/`confirmed_by`/`if_ok`/`if_doubt`/`if_not` may carry
/// structured data. `deny_unknown_fields` rejects any tenth canonical slot at
/// parse time (A3).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Act {
    pub who: Value,
    pub did: Value,
    pub this: Value,
    pub when: Value,
    pub confirmed_by: Value,
    pub if_ok: Value,
    pub if_doubt: Value,
    pub if_not: Value,
    pub status: Value,
}

impl Act {
    /// Build an Act from its nine slots.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        who: impl Into<Value>,
        did: impl Into<Value>,
        this: impl Into<Value>,
        when: impl Into<Value>,
        confirmed_by: impl Into<Value>,
        if_ok: impl Into<Value>,
        if_doubt: impl Into<Value>,
        if_not: impl Into<Value>,
        status: impl Into<Value>,
    ) -> Self {
        Self {
            who: who.into(),
            did: did.into(),
            this: this.into(),
            when: when.into(),
            confirmed_by: confirmed_by.into(),
            if_ok: if_ok.into(),
            if_doubt: if_doubt.into(),
            if_not: if_not.into(),
            status: status.into(),
        }
    }

    /// Parse and strictly validate an Act from JSON text. Rejects missing slots,
    /// extra slots, and empty required slots.
    pub fn from_json_strict(text: &str) -> Result<Self, ActError> {
        let value: Value = serde_json::from_str(text).map_err(|e| ActError::Json(e.to_string()))?;
        Self::from_value_strict(&value)
    }

    /// Validate an Act from an already-parsed JSON value.
    pub fn from_value_strict(value: &Value) -> Result<Self, ActError> {
        validate_object(value)?;
        let act: Act =
            serde_json::from_value(value.clone()).map_err(|e| ActError::Json(e.to_string()))?;
        Ok(act)
    }

    /// Preserve a candidate Act *as-is*, even if ugly or incomplete (A5).
    /// Candidate mode never rejects; it carries the raw value forward so nothing
    /// consequential is lost before it can be repaired.
    pub fn candidate_from_value(value: &Value) -> Candidate {
        Candidate {
            raw: value.clone(),
        }
    }

    /// The Act as a JSON object with slots in canonical order.
    pub fn to_value(&self) -> Value {
        let mut map = Map::new();
        map.insert("who".into(), self.who.clone());
        map.insert("did".into(), self.did.clone());
        map.insert("this".into(), self.this.clone());
        map.insert("when".into(), self.when.clone());
        map.insert("confirmed_by".into(), self.confirmed_by.clone());
        map.insert("if_ok".into(), self.if_ok.clone());
        map.insert("if_doubt".into(), self.if_doubt.clone());
        map.insert("if_not".into(), self.if_not.clone());
        map.insert("status".into(), self.status.clone());
        Value::Object(map)
    }

    /// Slots that are absent, null, or empty strings.
    pub fn missing_slots(&self) -> Vec<&'static str> {
        let v = self.to_value();
        missing_in(&v)
    }

    /// True when this Act validates as exactly nine non-empty canonical slots.
    pub fn is_valid(&self) -> bool {
        validate_object(&self.to_value()).is_ok()
    }

    /// Hash over only the nine canonical slots.
    pub fn tuple_hash(&self) -> Result<String, ActError> {
        tuple_hash(&self.to_value())
    }

    /// Content hash over the canonical slots (stable identity of the Act).
    pub fn content_hash(&self) -> Result<String, ActError> {
        content_hash(&self.to_value())
    }

    /// Deterministic canonical-JSON bytes of the nine slots.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ActError> {
        Ok(crate::canonical::canonical_json(&self.to_value())?.into_bytes())
    }

    /// Convenience: is this Act explicitly a candidate?
    pub fn is_candidate(&self) -> bool {
        self.status.as_str() == Some("candidate")
    }
}

/// A preserved candidate Act. Holds the raw value verbatim so ugly drafts are
/// never silently dropped (A5). Can be promoted to a strict Act once repaired.
#[derive(Clone, Debug, PartialEq)]
pub struct Candidate {
    pub raw: Value,
}

impl Candidate {
    /// The raw preserved value.
    pub fn value(&self) -> &Value {
        &self.raw
    }

    /// Slots still missing before this candidate could become a valid Act.
    pub fn missing_slots(&self) -> Vec<&'static str> {
        missing_in(&self.raw)
    }

    /// Attempt to promote a repaired candidate into a strict Act.
    pub fn promote(&self) -> Result<Act, ActError> {
        Act::from_value_strict(&self.raw)
    }
}

fn slot_is_present(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::String(s) => !s.trim().is_empty(),
        _ => true,
    }
}

fn missing_in(value: &Value) -> Vec<&'static str> {
    let Value::Object(obj) = value else {
        return SLOTS.to_vec();
    };
    SLOTS
        .iter()
        .filter(|slot| match obj.get(**slot) {
            Some(v) => !slot_is_present(v),
            None => true,
        })
        .copied()
        .collect()
}

/// Strict structural validation: exactly the nine canonical slots, none missing,
/// none extra.
pub fn validate_object(value: &Value) -> Result<(), ActError> {
    let Value::Object(obj) = value else {
        return Err(ActError::NotAnObject);
    };

    let extra: Vec<&str> = obj
        .keys()
        .filter(|k| !SLOTS.contains(&k.as_str()))
        .map(|k| k.as_str())
        .collect();
    if !extra.is_empty() {
        return Err(ActError::ExtraSlots(extra.join(", ")));
    }

    let missing = missing_in(value);
    if !missing.is_empty() {
        return Err(ActError::MissingSlots(missing.join(", ")));
    }

    Ok(())
}
