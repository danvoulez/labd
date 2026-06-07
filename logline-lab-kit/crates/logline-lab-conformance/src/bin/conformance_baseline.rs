//! Canon-tier conformance harness (C3 gate).
//!
//! Walks the vendored, pinned canon receipt vectors and runs labd's canonicalization
//! against them via `logline_lab_conformance::canon`. Prints a per-vector PASS/FAIL
//! report and **exits non-zero on any divergence** so the release gate / CI fail hard.
//!
//! A vector "conforms" when labd's verdict matches the vector's directory:
//!   valid/*   → must verify OK
//!   invalid/* → must be rejected

use std::fs;
use std::path::{Path, PathBuf};

use logline_lab_conformance::canon::verify;
use serde_json::Value;

fn vectors_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../foundation/conformance/canon/vectors/receipt")
}

fn json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) == Some("json") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn main() {
    let root = vectors_root();
    println!("# LogLine canon conformance — BASELINE (reporter, not a gate)");
    println!("# vectors: {}", root.display());
    println!("# pinned:  LogLine-Foundation/conformance@389a6b676af30bf5e344f9287ef51472b7f7a53f\n");

    let mut pass = 0usize;
    let mut fail = 0usize;

    for (label, dir, expect_ok) in [
        ("VALID  ", root.join("valid"), true),
        ("INVALID", root.join("invalid"), false),
    ] {
        for path in json_files(&dir) {
            let name = path.file_name().unwrap().to_string_lossy();
            let text = match fs::read_to_string(&path) {
                Ok(t) => t,
                Err(e) => {
                    println!("✗ {label} {name} — unreadable: {e}");
                    fail += 1;
                    continue;
                }
            };
            let value: Value = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {label} {name} — invalid JSON: {e}");
                    fail += 1;
                    continue;
                }
            };
            let out = verify(&value);
            let conforms = out.ok == expect_ok;
            if conforms {
                println!("✓ {label} {name}");
                pass += 1;
            } else {
                let why = if expect_ok {
                    format!("expected VALID, labd rejected: {}", out.errors.join("; "))
                } else {
                    "expected REJECT, labd accepted".to_string()
                };
                println!("✗ {label} {name} — {why}");
                fail += 1;
            }
        }
    }

    let total = pass + fail;
    println!("\n{pass}/{total} conform · {fail} divergence(s)");
    if fail > 0 {
        println!("FAIL — labd diverges from the vendored canon. Gate must not pass.");
        std::process::exit(1);
    }
    println!("GREEN — labd reproduces the canon byte-for-byte.");
}
