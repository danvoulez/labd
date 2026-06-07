//! Canonical JSON + content-addressed hashing.
//!
//! Canonicalization is **RFC 8785 / JCS**, delegated to the vetted
//! `serde_json_canonicalizer` crate (accepted by behavior against the foundation
//! conformance vectors + adversarial probe + Node reference verifier — see
//! `recovery/CONFORMANCE_PLAN.md` and `recovery/CANON_ERRATA.md`). The previously
//! hand-rolled canonicalizer was NOT RFC 8785 (it sorted keys by Unicode scalar
//! rather than UTF-16 code units and mis-formatted numbers); it survives only as a
//! `#[cfg(test)]` fixture proving that divergence, never as a production path.

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::error::ActError;

/// The nine canonical Act slots, in canonical order.
pub const SLOTS: [&str; 9] = [
    "who",
    "did",
    "this",
    "when",
    "confirmed_by",
    "if_ok",
    "if_doubt",
    "if_not",
    "status",
];

/// Canonical JSON bytes per RFC 8785 (JCS): object keys sorted by UTF-16 code units,
/// ECMAScript number formatting, minimal string escaping, no insignificant whitespace.
pub fn canonical_json(value: &Value) -> Result<String, ActError> {
    serde_json_canonicalizer::to_string(value).map_err(|_| ActError::StringEncoding)
}

fn sha256_hex(canonical: &str) -> String {
    let digest = Sha256::digest(canonical.as_bytes());
    let mut s = String::with_capacity(digest.len() * 2);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for b in digest.iter() {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0f) as usize] as char);
    }
    s
}

/// Hash over only the nine canonical slots (the semantic tuple). Envelope,
/// runtime, and storage fields do not participate.
pub fn tuple_hash(value: &Value) -> Result<String, ActError> {
    let Value::Object(obj) = value else {
        return Err(ActError::NotAnObject);
    };
    let mut tuple = Map::new();
    for slot in &SLOTS {
        if let Some(v) = obj.get(*slot) {
            tuple.insert((*slot).to_string(), v.clone());
        }
    }
    Ok(sha256_hex(&canonical_json(&Value::Object(tuple))?))
}

/// Hash over a transport envelope: `sha256(jcs(envelope minus envelope_hash))`,
/// i.e. over `{content, transport}`. Operates on the raw value so any extra
/// transport metadata is preserved in the hash (no typed round-trip loss).
///
/// This is the boundary-crossing primitive (LIP-0007): the sender computes it at
/// every transport hop; the receiver recomputes and compares before accepting. The
/// `envelope_hash` lives ONLY on the wrapper — never inside the receipt/content.
pub fn envelope_hash(value: &Value) -> Result<String, ActError> {
    let mut env = value.clone();
    let Value::Object(obj) = &mut env else {
        return Err(ActError::NotAnObject);
    };
    obj.remove("envelope_hash");
    Ok(sha256_hex(&canonical_json(&env)?))
}

/// Hash over the whole value minus self-referential `id`/`hashes` fields.
pub fn content_hash(value: &Value) -> Result<String, ActError> {
    let mut v = value.clone();
    let Value::Object(obj) = &mut v else {
        return Err(ActError::NotAnObject);
    };
    obj.remove("id");
    obj.remove("hashes");
    Ok(sha256_hex(&canonical_json(&v)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// The pre-P1 hand-rolled canonicalizer. **Test fixture ONLY** — kept to prove it
    /// diverged from RFC 8785/JCS. It is not exported and no production code calls it.
    fn legacy_hand_rolled(value: &Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => serde_json::to_string(s).unwrap(),
            Value::Array(items) => {
                let inner: Vec<String> = items.iter().map(legacy_hand_rolled).collect();
                format!("[{}]", inner.join(","))
            }
            Value::Object(obj) => {
                let mut keys: Vec<&String> = obj.keys().collect();
                keys.sort();
                let parts: Vec<String> = keys
                    .iter()
                    .map(|k| format!("{}:{}", serde_json::to_string(k).unwrap(), legacy_hand_rolled(&obj[*k])))
                    .collect();
                format!("{{{}}}", parts.join(","))
            }
        }
    }

    /// The adversarial inputs where RFC 8785 and the old hand-roll part ways, with the
    /// canon-correct bytes (per the foundation reference verifier; see CANON_ERRATA E-001).
    const ADVERSARIAL: &[(&str, &str)] = &[
        (r#"{"𐀀":1,"￿":2}"#, "{\"\u{10000}\":1,\"\u{FFFF}\":2}"), // UTF-16 key order
        (r#"{"n":1.0}"#, r#"{"n":1}"#),                            // integer-valued float
        (r#"{"n":100000000000000000000}"#, r#"{"n":100000000000000000000}"#), // exponent threshold
        (r#"{"n":-0}"#, r#"{"n":0}"#),                             // negative zero
        (r#"{"n":1e21}"#, r#"{"n":1e+21}"#),                       // exponent sign (E-001)
    ];

    /// Production canonicalization now matches the canon byte-for-byte on every
    /// adversarial case (this is the JCS conformance guarantee).
    #[test]
    fn jcs_matches_canon_on_adversarial_inputs() {
        for (input, want) in ADVERSARIAL {
            let v: Value = serde_json::from_str(input).unwrap();
            assert_eq!(&canonical_json(&v).unwrap(), want, "input {input}");
        }
    }

    /// The old hand-roll genuinely diverged on those same inputs — kept live so the
    /// reason we replaced it cannot be lost. If this ever stops diverging, the fixture
    /// is wrong, not the canon.
    #[test]
    fn legacy_hand_roll_diverged_from_jcs() {
        let mut diverged = 0;
        for (input, _want) in ADVERSARIAL {
            let v: Value = serde_json::from_str(input).unwrap();
            if legacy_hand_rolled(&v) != canonical_json(&v).unwrap() {
                diverged += 1;
            }
        }
        assert!(diverged >= 4, "expected the legacy hand-roll to diverge on most cases, got {diverged}");
    }

    /// The JCS worked example from the canon hash-profile.
    #[test]
    fn jcs_worked_example() {
        let v = json!({ "b": 2, "a": [1, 0.5], "c": "hi" });
        assert_eq!(canonical_json(&v).unwrap(), r#"{"a":[1,0.5],"b":2,"c":"hi"}"#);
    }
}
