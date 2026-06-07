//! Adversarial JCS probe (evidence reporter, exits 0).
//!
//! Proves *why* labd's hand-rolled canonicalizer must be replaced even though the
//! current receipt vectors pass 20/20: the vectors never exercise these edge cases.
//! Each case is canonicalized by labd (`logline_act::canonical_json`) and compared to
//! the canon-correct bytes produced by the foundation reference JCS (verified via
//! `tools/verify-receipt.mjs`'s algorithm; ECMAScript number formatting + UTF-16
//! code-unit key ordering, per RFC 8785).

use logline_act::canonical_json;
use serde_json::Value;

/// (name, input JSON text, canon-correct canonicalization)
const CASES: &[(&str, &str, &str)] = &[
    // UTF-16 code-unit key order: U+10000 (surrogate D800 DC00) sorts BEFORE U+FFFF.
    // labd sorts by Unicode scalar (UTF-8 byte order), giving the opposite order.
    ("astral_key_order", r#"{"𐀀":1,"￿":2}"#, "{\"\u{10000}\":1,\"\u{FFFF}\":2}"),
    // ECMAScript Number::toString — the canon/reference forms.
    ("es6_large_exp", r#"{"n":1e21}"#, r#"{"n":1e+21}"#),
    ("es6_small_exp", r#"{"n":1e-7}"#, r#"{"n":1e-7}"#),
    ("es6_int_float", r#"{"n":1.0}"#, r#"{"n":1}"#),
    ("es6_big_int", r#"{"n":100000000000000000000}"#, r#"{"n":100000000000000000000}"#),
    ("neg_zero", r#"{"n":-0}"#, r#"{"n":0}"#),
    ("frac_trailing", r#"{"n":1.50}"#, r#"{"n":1.5}"#),
];

fn main() {
    println!("# Adversarial JCS probe — labd hand-roll vs canon-correct (RFC 8785)");
    println!("# canon truth computed by the foundation reference JCS (verify-receipt.mjs algorithm)\n");

    let mut diverge = 0usize;
    for (name, input, canon) in CASES {
        let value: Value = match serde_json::from_str(input) {
            Ok(v) => v,
            Err(e) => {
                println!("[parse-error] {name}: {e}");
                diverge += 1;
                continue;
            }
        };
        let labd = canonical_json(&value).unwrap_or_else(|e| format!("<error: {e}>"));
        let verdict = if labd == *canon { "MATCH   " } else { "DIVERGE " };
        if labd != *canon {
            diverge += 1;
        }
        println!("{verdict} {name}");
        println!("    input: {input}");
        println!("    labd : {labd}");
        println!("    canon: {canon}");
    }

    let total = CASES.len();
    println!("\n{diverge}/{total} cases DIVERGE from the canon.");
    if diverge > 0 {
        println!(
            "PROOF: the hand-rolled canonicalizer is NOT RFC 8785 / JCS. Replacing it (P1) is \
             mandatory regardless of the receipt vectors passing 20/20 — those vectors simply do \
             not contain astral keys or ECMAScript-formatted numbers."
        );
    }
}
