//! Canonical JSON + content-addressed hashing.
//!
//! Recovered (rewritten standalone, no vendored dependency) from the LogLine
//! Foundation `status` crate (`vendor/logline-foundation/engine/crates/status/
//! src/receipt.rs`) found in the source fruits. Object keys are sorted; numbers,
//! strings, and arrays are encoded deterministically so the same Act always
//! produces the same bytes and therefore the same hash.

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

/// Deterministic JSON encoding: object keys sorted lexically, no insignificant
/// whitespace.
pub fn canonical_json(value: &Value) -> Result<String, ActError> {
    match value {
        Value::Null => Ok("null".to_string()),
        Value::Bool(b) => Ok(if *b { "true" } else { "false" }.to_string()),
        Value::Number(n) => Ok(n.to_string()),
        Value::String(s) => serde_json::to_string(s).map_err(|_| ActError::StringEncoding),
        Value::Array(items) => {
            let mut out = String::from("[");
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str(&canonical_json(item)?);
            }
            out.push(']');
            Ok(out)
        }
        Value::Object(obj) => canonical_object(obj),
    }
}

fn canonical_object(obj: &Map<String, Value>) -> Result<String, ActError> {
    let mut keys: Vec<&String> = obj.keys().collect();
    keys.sort();
    let mut out = String::from("{");
    for (i, key) in keys.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let k = serde_json::to_string(key).map_err(|_| ActError::StringEncoding)?;
        out.push_str(&k);
        out.push(':');
        out.push_str(&canonical_json(&obj[*key])?);
    }
    out.push('}');
    Ok(out)
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
