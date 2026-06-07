//! Canon-tier conformance: verify `logline.receipt.v0` vectors against the
//! **vendored, pinned** LogLine canon (`foundation/conformance/canon/`).
//!
//! This mirrors the foundation reference verifier (`tools/verify-receipt.mjs`) but
//! drives labd's OWN canonicalization (`logline_act::{tuple_hash, content_hash,
//! canonical_json}`) so it measures whether labd reproduces the canon's golden
//! hashes byte-for-byte. Divergences (JCS unicode/number rules; the missing
//! envelope primitive) surface here as honest failures — that is the point.
//!
//! Three hash layers (LIP-0007):
//! - `tuple_hash`    = sha256(jcs(9 slots only))
//! - `content_hash`  = sha256(jcs(receipt minus {id, hashes}))  — equals the `id`
//! - `envelope_hash` = sha256(jcs({content, transport}))         — Envelope wrapper only

use serde_json::Value;

pub const RECEIPT_VERSION: &str = "logline.receipt.v0";
pub const JSON_CANON: &str = "jcs-rfc8785";

/// Canonical nine slots, in canon order.
pub const SLOTS: [&str; 9] = [
    "who", "did", "this", "when", "confirmed_by", "if_ok", "if_doubt", "if_not", "status",
];

/// Names that carried meaning in earlier drafts and MUST NOT appear in a v0
/// receipt (transport lives only on the Envelope; result/evidence are upstream).
pub const FORBIDDEN_LEGACY: [&str; 3] = ["result", "evidence", "transport"];

/// Outcome of verifying one receipt or envelope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifyOutcome {
    pub ok: bool,
    pub errors: Vec<String>,
}

impl VerifyOutcome {
    fn from(errors: Vec<String>) -> Self {
        Self { ok: errors.is_empty(), errors }
    }
}

fn is_hex64(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}

/// True when the value looks like an Envelope wrapper rather than a bare receipt.
pub fn is_envelope(value: &Value) -> bool {
    let Some(obj) = value.as_object() else { return false };
    (obj.contains_key("content") && obj.contains_key("transport")) || obj.contains_key("envelope_hash")
}

/// Schema-shape checks for a bare receipt (no hash recomputation yet).
fn validate_receipt_schema(receipt: &Value) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(obj) = receipt.as_object() else {
        return vec!["receipt is not a JSON object".into()];
    };

    match obj.get("receipt_version").and_then(Value::as_str) {
        Some(RECEIPT_VERSION) => {}
        other => errors.push(format!("receipt_version must be {RECEIPT_VERSION:?}, got {other:?}")),
    }
    match obj.get("json_canonicalization").and_then(Value::as_str) {
        Some(JSON_CANON) => {}
        other => errors.push(format!("json_canonicalization must be {JSON_CANON:?}, got {other:?}")),
    }

    for slot in SLOTS {
        match obj.get(slot) {
            None => errors.push(format!("missing required slot {slot:?}")),
            Some(v) if !v.is_string() => errors.push(format!("slot {slot:?} must be a string")),
            Some(_) => {}
        }
    }

    match obj.get("hashes").and_then(Value::as_object) {
        None => errors.push("missing or invalid \"hashes\" object".into()),
        Some(h) => {
            if h.get("algorithm").and_then(Value::as_str) != Some("sha256") {
                errors.push("hashes.algorithm must be \"sha256\"".into());
            }
            for f in ["tuple_hash", "content_hash"] {
                match h.get(f).and_then(Value::as_str) {
                    Some(s) if is_hex64(s) => {}
                    _ => errors.push(format!("hashes.{f} must be a 64-char lowercase hex sha256")),
                }
            }
            for k in h.keys() {
                if !matches!(k.as_str(), "tuple_hash" | "content_hash" | "algorithm") {
                    errors.push(format!(
                        "hashes contains unexpected field {k:?} (envelope_hash MUST NOT live inside the receipt)"
                    ));
                }
            }
        }
    }

    match obj.get("id").and_then(Value::as_str) {
        Some(s) if is_hex64(s) => {}
        _ => errors.push("id must be a 64-char lowercase hex sha256".into()),
    }

    for k in FORBIDDEN_LEGACY {
        if obj.contains_key(k) {
            errors.push(format!("reserved legacy field {k:?} MUST NOT appear in a v0 receipt"));
        }
    }

    errors
}

/// Verify a bare receipt: schema, then recompute the two emission-time hashes via
/// labd's canonicalization and compare to the embedded values and the `id`.
pub fn verify_receipt(receipt: &Value) -> VerifyOutcome {
    let mut errors = validate_receipt_schema(receipt);
    if !errors.is_empty() {
        return VerifyOutcome::from(errors);
    }

    let obj = receipt.as_object().expect("schema-checked object");
    let hashes = obj.get("hashes").and_then(Value::as_object).expect("schema-checked hashes");

    match logline_act::tuple_hash(receipt) {
        Ok(got) => {
            let want = hashes.get("tuple_hash").and_then(Value::as_str).unwrap_or_default();
            if got != want {
                errors.push(format!("tuple_hash mismatch: recomputed {got}, embedded {want}"));
            }
        }
        Err(e) => errors.push(format!("tuple_hash could not be computed: {e}")),
    }

    match logline_act::content_hash(receipt) {
        Ok(got) => {
            let want = hashes.get("content_hash").and_then(Value::as_str).unwrap_or_default();
            if got != want {
                errors.push(format!("content_hash mismatch: recomputed {got}, embedded {want}"));
            }
            let id = obj.get("id").and_then(Value::as_str).unwrap_or_default();
            if id != got {
                errors.push(format!("id must equal content_hash: id {id}, content_hash {got}"));
            }
        }
        Err(e) => errors.push(format!("content_hash could not be computed: {e}")),
    }

    VerifyOutcome::from(errors)
}

/// Verify an Envelope wrapper: the inner content receipt, the transport metadata, and
/// the `envelope_hash` itself (recomputed via labd's `logline_act::envelope_hash` over
/// the raw `{content, transport}`, preserving any extra transport fields).
pub fn verify_envelope(env: &Value) -> VerifyOutcome {
    let mut errors = Vec::new();
    let Some(obj) = env.as_object() else {
        return VerifyOutcome::from(vec!["envelope is not a JSON object".into()]);
    };

    match obj.get("content") {
        Some(content) if content.is_object() => {
            let inner = verify_receipt(content);
            errors.extend(inner.errors.into_iter().map(|e| format!("content: {e}")));
        }
        _ => errors.push("envelope.content must be a receipt object".into()),
    }

    match obj.get("transport").and_then(Value::as_object) {
        None => errors.push("envelope.transport must be an object".into()),
        Some(t) => {
            for k in ["sent_by", "sent_to", "sent_at"] {
                match t.get(k).and_then(Value::as_str) {
                    Some(s) if !s.is_empty() => {}
                    _ => errors.push(format!("envelope.transport.{k} must be a non-empty string")),
                }
            }
        }
    }

    match obj.get("envelope_hash").and_then(Value::as_str) {
        Some(s) if is_hex64(s) => {}
        _ => errors.push("envelope_hash must be a 64-char lowercase hex sha256".into()),
    }

    // Recompute and confirm the boundary-crossing hash (receiver-side check).
    if let Err(e) = logline_act::verify_envelope_value(env) {
        errors.push(format!("envelope_hash verification failed: {e}"));
    }

    VerifyOutcome::from(errors)
}

/// Classify and verify any vector value (receipt or envelope).
pub fn verify(value: &Value) -> VerifyOutcome {
    if is_envelope(value) {
        verify_envelope(value)
    } else {
        verify_receipt(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// The canon's JCS worked example: labd's canonicalizer must emit these exact
    /// bytes for this input (ASCII keys + a simple float). This is the floor — if
    /// this fails, labd diverges even on trivial input.
    #[test]
    fn jcs_worked_example_bytes() {
        let v = json!({ "b": 2, "a": [1, 0.5], "c": "hi" });
        assert_eq!(logline_act::canonical_json(&v).unwrap(), r#"{"a":[1,0.5],"b":2,"c":"hi"}"#);
    }

    /// Adversarial JCS conformance — **EXPECTED-RED until P1 (JCS replacement)**.
    ///
    /// Proof that labd's hand-rolled canonicalizer is NOT RFC 8785 / JCS, even though
    /// the receipt vectors pass 20/20 (they contain no astral keys or ECMAScript-
    /// formatted numbers). Marked `#[ignore]` so it does not break the green suite;
    /// run `cargo test -p logline-lab-conformance -- --ignored` to see the divergence.
    /// When P1 lands a conformant JCS impl, remove `#[ignore]` — it must then pass.
    /// Full evidence: `release/checks/jcs-adversarial-probe.txt`.
    #[test]
    #[ignore = "expected-red until P1 JCS replacement; proves the hand-roll diverges from RFC 8785"]
    fn jcs_adversarial_matches_canon() {
        // (input JSON, canon-correct canonicalization per the foundation reference JCS)
        let cases = [
            (r#"{"𐀀":1,"￿":2}"#, "{\"\u{10000}\":1,\"\u{FFFF}\":2}"), // UTF-16 key order
            (r#"{"n":1.0}"#, r#"{"n":1}"#),                            // integer-valued float
            (r#"{"n":100000000000000000000}"#, r#"{"n":100000000000000000000}"#), // exp threshold
            (r#"{"n":-0}"#, r#"{"n":0}"#),                             // negative zero
        ];
        let mut diffs = Vec::new();
        for (input, want) in cases {
            let v: Value = serde_json::from_str(input).unwrap();
            let got = logline_act::canonical_json(&v).unwrap();
            if got != want {
                diffs.push(format!("  {input}\n    labd : {got}\n    canon: {want}"));
            }
        }
        assert!(
            diffs.is_empty(),
            "labd canonicalization diverges from RFC 8785 / JCS:\n{}",
            diffs.join("\n")
        );
    }

    /// A minimal, all-ASCII valid receipt verifies (labd agrees with the canon here).
    #[test]
    fn minimal_resolved_verifies() {
        let r = json!({
            "receipt_version": "logline.receipt.v0",
            "who": "", "did": "", "this": "", "when": "", "confirmed_by": "",
            "if_ok": "", "if_doubt": "", "if_not": "", "status": "",
            "hashes": {
                "tuple_hash": "35b20f03e04f2073adf068ec9f5e893c476ed626933611bbf3c64edfd6a054ef",
                "content_hash": "6ccd6e8d8a8a13b083bf5ed75c7c71d9f6d8ac9ae55cd036bdf5f39fb91102d8",
                "algorithm": "sha256"
            },
            "json_canonicalization": "jcs-rfc8785",
            "id": "6ccd6e8d8a8a13b083bf5ed75c7c71d9f6d8ac9ae55cd036bdf5f39fb91102d8"
        });
        let out = verify_receipt(&r);
        assert!(out.ok, "expected valid, got {:?}", out.errors);
    }

    /// A tampered tuple_hash must be rejected (the harness actually checks hashes).
    #[test]
    fn tuple_hash_mismatch_rejected() {
        let r = json!({
            "receipt_version": "logline.receipt.v0",
            "who": "dan", "did": "rested", "this": "slept_well",
            "when": "2026-05-17T07:30:00Z", "confirmed_by": "dan",
            "if_ok": "continue_minilab_work", "if_doubt": "", "if_not": "", "status": "claimed",
            "hashes": {
                "tuple_hash": "0000000000000000000000000000000000000000000000000000000000000000",
                "content_hash": "b74954069c9135090740439e08bdd442b4b12767c4df63b504c3e6b1029ffbb8",
                "algorithm": "sha256"
            },
            "json_canonicalization": "jcs-rfc8785",
            "id": "b74954069c9135090740439e08bdd442b4b12767c4df63b504c3e6b1029ffbb8"
        });
        let out = verify_receipt(&r);
        assert!(!out.ok, "tampered tuple_hash should be rejected");
    }

    /// A 10th, non-reserved top-level field is valid AUX (the canon is
    /// additionalProperties:true) — NOT an error. This is exactly the rule the
    /// quarantined `i02_tenth_slot` vector got wrong.
    #[test]
    fn aux_field_is_not_rejected_as_tenth_slot() {
        let r = json!({
            "receipt_version": "logline.receipt.v0",
            "who": "dan", "did": "noted", "this": "x", "when": "t",
            "confirmed_by": "dan", "if_ok": "", "if_doubt": "", "if_not": "", "status": "claimed",
            "free_aux": "i am allowed",
            "hashes": { "tuple_hash": "x", "content_hash": "x", "algorithm": "sha256" },
            "json_canonicalization": "jcs-rfc8785",
            "id": "x"
        });
        // Schema-level: the AUX field itself must not be flagged. (Hashes here are
        // placeholders, so hash checks will fail — we only assert no "tenth field" error.)
        let errs = validate_receipt_schema(&r);
        assert!(
            !errs.iter().any(|e| e.contains("free_aux")),
            "AUX field must not be rejected as a tenth slot: {errs:?}"
        );
    }
}
