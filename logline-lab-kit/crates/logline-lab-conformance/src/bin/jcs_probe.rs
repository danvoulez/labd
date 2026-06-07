//! Adversarial JCS probe (evidence reporter, exits 0).
//!
//! Exercises the RFC 8785 edge cases the receipt vectors do NOT cover (astral key
//! ordering, ECMAScript number formatting). Pre-P1 this proved the hand-roll diverged;
//! post-P1 (canonicalization delegated to `serde_json_canonicalizer`) it confirms labd
//! now matches the canon-correct bytes byte-for-byte. The historical divergence is
//! preserved in git and in `logline-act`'s `legacy_hand_roll_diverged_from_jcs` test.

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
    println!("# Adversarial JCS probe — labd canonicalization vs canon-correct (RFC 8785)");
    println!("# labd: serde_json_canonicalizer (P1). canon truth: foundation reference JCS.\n");

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
    println!("\n{diverge}/{total} cases diverge from the canon.");
    if diverge == 0 {
        println!(
            "CONFORMANT: labd canonicalization matches RFC 8785 / JCS on every adversarial case, \
             including the ones the receipt vectors do not exercise (astral keys, ECMAScript \
             numbers). The historical hand-roll divergence is preserved in git and in \
             logline-act::canonical::tests::legacy_hand_roll_diverged_from_jcs."
        );
    } else {
        println!("REGRESSION: labd diverges from the canon — JCS canonicalization is broken.");
        std::process::exit(1);
    }
}
