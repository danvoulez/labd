//! Canon conformance suite as a hard `cargo test` gate (C3).
//!
//! Walks the vendored, pinned canon receipt vectors and asserts labd's verdict matches
//! each vector's directory: `valid/*` must verify, `invalid/*` must be rejected. This is
//! the enforcing counterpart of the `conformance_baseline` reporter binary — when this
//! test is green, `labd == 21/21` against the vendored canon.

use std::fs;
use std::path::{Path, PathBuf};

use logline_lab_conformance::canon::verify;
use serde_json::Value;

fn vectors_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../foundation/conformance/canon/vectors/receipt")
}

fn json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("read {dir:?}: {e}"))
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();
    out.sort();
    out
}

fn load(path: &Path) -> Value {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {path:?}: {e}"))
}

#[test]
fn canon_valid_vectors_all_verify() {
    let dir = vectors_root().join("valid");
    let files = json_files(&dir);
    assert!(!files.is_empty(), "no valid vectors found in {dir:?}");
    let mut failures = Vec::new();
    for f in &files {
        let out = verify(&load(f));
        if !out.ok {
            failures.push(format!("{}: {}", f.file_name().unwrap().to_string_lossy(), out.errors.join("; ")));
        }
    }
    assert!(failures.is_empty(), "canon VALID vectors must all verify:\n{}", failures.join("\n"));
}

#[test]
fn canon_invalid_vectors_all_rejected() {
    let dir = vectors_root().join("invalid");
    let files = json_files(&dir);
    assert!(!files.is_empty(), "no invalid vectors found in {dir:?}");
    let mut leaks = Vec::new();
    for f in &files {
        if verify(&load(f)).ok {
            leaks.push(f.file_name().unwrap().to_string_lossy().to_string());
        }
    }
    assert!(leaks.is_empty(), "canon INVALID vectors must all be rejected, but these passed:\n{}", leaks.join("\n"));
}
