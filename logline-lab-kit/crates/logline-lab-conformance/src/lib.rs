//! `logline-lab-conformance` — the protocol's proof of compatibility.
//!
//! This is the heart of "protocol, not company" (FINAL §16): reference vectors
//! that any Lab can run, a conformance report, and exportable examples another
//! Lab can compare against. It runs entirely offline — no central hosted service
//! is required (A17).
//!
//! A vector declares an expectation:
//! - `valid`     — strictly admits as exactly nine non-empty slots.
//! - `invalid`   — must be rejected (missing slot, tenth slot, …).
//! - `ambiguous` — not strictly valid, but preservable as a candidate (ugly
//!   capture is allowed; promotion is strict).
//!
//! This module is the **kit tier** (structural slot-validity over labd's own
//! lab-formation examples in `foundation/conformance/kit-examples/`). The
//! authoritative **canon tier** (JCS byte-exact three-layer hashing against the
//! vendored, pinned `logline.receipt.v0` vectors) lives in [`canon`].

#![forbid(unsafe_code)]

pub mod canon;

use logline_act::Act;
use serde::{Deserialize, Serialize};

/// What a vector asserts about an Act.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expectation {
    Valid,
    Invalid,
    Ambiguous,
}

/// A reference vector: an Act JSON and its expectation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    pub id: String,
    pub expectation: Expectation,
    pub act_json: String,
}

/// The result of running a single vector.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorResult {
    pub id: String,
    pub expectation: Expectation,
    pub passed: bool,
    pub detail: String,
}

/// A conformance report over a vector set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConformanceReport {
    pub kind: String,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<VectorResult>,
}

impl ConformanceReport {
    pub fn is_green(&self) -> bool {
        self.failed == 0 && self.total > 0
    }
}

/// An exported example another Lab can compare against: the canonical Act plus
/// its content hash. Determinism of the hash is the compatibility check.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedExample {
    pub id: String,
    pub act_json: String,
    pub content_hash: String,
}

const VALID: &[(&str, &str)] = &[
    ("v01_declare_lab", include_str!("../../../foundation/conformance/kit-examples/v01_declare_lab.json")),
    ("v02_observe", include_str!("../../../foundation/conformance/kit-examples/v02_observe.json")),
];
const INVALID: &[(&str, &str)] = &[
    ("i01_missing_slot", include_str!("../../../foundation/conformance/kit-examples/i01_missing_slot.json")),
];
const AMBIGUOUS: &[(&str, &str)] = &[
    ("a01_ugly_candidate", include_str!("../../../foundation/conformance/kit-examples/a01_ugly_candidate.json")),
];

/// The built-in reference vector set (shipped with the kit, runs offline).
pub fn builtin_vectors() -> Vec<Vector> {
    let mut v = Vec::new();
    for (id, json) in VALID {
        v.push(Vector { id: (*id).into(), expectation: Expectation::Valid, act_json: (*json).into() });
    }
    for (id, json) in INVALID {
        v.push(Vector { id: (*id).into(), expectation: Expectation::Invalid, act_json: (*json).into() });
    }
    for (id, json) in AMBIGUOUS {
        v.push(Vector { id: (*id).into(), expectation: Expectation::Ambiguous, act_json: (*json).into() });
    }
    v
}

fn check(vector: &Vector) -> VectorResult {
    let strict = Act::from_json_strict(&vector.act_json);
    let (passed, detail) = match vector.expectation {
        Expectation::Valid => match strict {
            Ok(_) => (true, "admitted as nine valid slots".into()),
            Err(e) => (false, format!("expected valid, rejected: {e}")),
        },
        Expectation::Invalid => match strict {
            Ok(_) => (false, "expected invalid, but it was admitted".into()),
            Err(e) => (true, format!("correctly rejected: {e}")),
        },
        Expectation::Ambiguous => {
            // Must not strictly validate, but must be preservable as a candidate
            // (parseable object) — ugly capture allowed, promotion strict.
            let parseable = serde_json::from_str::<serde_json::Value>(&vector.act_json)
                .map(|v| v.is_object())
                .unwrap_or(false);
            match (strict.is_err(), parseable) {
                (true, true) => (true, "preserved as candidate, not strictly valid".into()),
                (false, _) => (false, "expected ambiguous, but strictly validated".into()),
                (_, false) => (false, "not preservable as a candidate object".into()),
            }
        }
    };
    VectorResult {
        id: vector.id.clone(),
        expectation: vector.expectation,
        passed,
        detail,
    }
}

/// Run a vector set and produce a report (A13/A14).
pub fn run(vectors: &[Vector]) -> ConformanceReport {
    let results: Vec<VectorResult> = vectors.iter().map(check).collect();
    let passed = results.iter().filter(|r| r.passed).count();
    ConformanceReport {
        kind: "logline.conformance_report.v0".to_string(),
        total: results.len(),
        passed,
        failed: results.len() - passed,
        results,
    }
}

/// Export the valid vectors as comparable examples with content hashes (A15).
pub fn export_examples(vectors: &[Vector]) -> Vec<ExportedExample> {
    vectors
        .iter()
        .filter(|v| v.expectation == Expectation::Valid)
        .filter_map(|v| {
            let act = Act::from_json_strict(&v.act_json).ok()?;
            Some(ExportedExample {
                id: v.id.clone(),
                act_json: v.act_json.clone(),
                content_hash: act.content_hash().ok()?,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A13/A14 — Conformance vectors run and produce a report.
    #[test]
    fn a13_a14_vectors_run_and_report() {
        let report = run(&builtin_vectors());
        assert!(report.total >= 4);
        assert!(report.is_green(), "report not green: {:?}", report.results);
    }

    /// A15 — Example Acts can be exported (with content hashes).
    #[test]
    fn a15_export_examples() {
        let examples = export_examples(&builtin_vectors());
        assert_eq!(examples.len(), 2);
        assert!(examples.iter().all(|e| e.content_hash.len() == 64));
    }

    /// A16 — Another Lab can compare against the examples (deterministic hash).
    #[test]
    fn a16_examples_are_reproducible() {
        let examples = export_examples(&builtin_vectors());
        for e in &examples {
            // A "second Lab" recomputes the hash from the exported Act JSON.
            let act = Act::from_json_strict(&e.act_json).unwrap();
            assert_eq!(act.content_hash().unwrap(), e.content_hash);
        }
    }

    /// A17 — No central hosted service required (runs fully offline / in-process).
    #[test]
    fn a17_runs_offline() {
        // No network, no IO: builtin vectors are embedded and the runner is pure.
        let report = run(&builtin_vectors());
        assert!(report.is_green());
    }
}
