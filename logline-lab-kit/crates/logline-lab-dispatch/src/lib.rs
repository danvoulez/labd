//! `logline-lab-dispatch` — the worker contract.
//!
//! A worker executes admitted workorders. It is *hands*, not authority: it can
//! never define semantic truth, and it returns **evidence, not closure**
//! (Operator §14, A19). A worker cannot execute without an explicit allow
//! (A18). The shell worker supports a dry-run mode and a real command mode.

#![forbid(unsafe_code)]

use std::process::Command;

use logline_lab_core::Evidence;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DispatchError {
    #[error("worker refused: workorder `{0}` is not allowed")]
    NotAllowed(String),
    #[error("worker refused: empty command")]
    EmptyCommand,
    #[error("worker execution failed: {0}")]
    Exec(String),
}

/// Execution mode for the shell worker.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    DryRun,
    Real,
}

/// A workorder: a scoped, gated instruction for a worker. The `allow` flag is the
/// admission gate — it is set by the Lab, not by the worker.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workorder {
    pub id: String,
    pub scope: String,
    pub command: Vec<String>,
    pub allow: bool,
    pub mode: Mode,
}

impl Workorder {
    pub fn new(id: impl Into<String>, scope: impl Into<String>, command: Vec<String>) -> Self {
        Self {
            id: id.into(),
            scope: scope.into(),
            command,
            allow: false,
            mode: Mode::DryRun,
        }
    }
    pub fn allowed(mut self, allow: bool) -> Self {
        self.allow = allow;
        self
    }
    pub fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }
}

/// What a worker returns: evidence and a transport status. Explicitly **not** a
/// receipt — the worker does not close anything.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub workorder_id: String,
    pub scope: String,
    pub mode: Mode,
    /// Real captured evidence (command output / dry-run description).
    pub evidence: Evidence,
    /// Transport-level success of running the command; not a semantic verdict.
    pub transport_ok: bool,
}

impl ExecutionReport {
    /// A worker report is never a receipt (A19).
    pub fn is_receipt(&self) -> bool {
        false
    }
}

/// The shell worker. Runs a workorder, returning evidence.
pub struct ShellWorker;

impl ShellWorker {
    /// Execute a workorder. Refuses without allow (A18); in real mode runs the
    /// command and captures genuine output; in dry-run mode describes what would
    /// run. Either way it returns evidence, never closure (A19).
    pub fn execute(workorder: &Workorder, now: &str) -> Result<ExecutionReport, DispatchError> {
        if !workorder.allow {
            return Err(DispatchError::NotAllowed(workorder.id.clone()));
        }
        if workorder.command.is_empty() {
            return Err(DispatchError::EmptyCommand);
        }

        let (payload, transport_ok, kind) = match workorder.mode {
            Mode::DryRun => (
                json!({
                    "dry_run": true,
                    "would_run": workorder.command,
                }),
                true,
                "dry_run",
            ),
            Mode::Real => {
                let (program, args) = workorder.command.split_first().unwrap();
                let output = Command::new(program)
                    .args(args)
                    .output()
                    .map_err(|e| DispatchError::Exec(e.to_string()))?;
                let transport_ok = output.status.success();
                (
                    json!({
                        "command": workorder.command,
                        "exit_code": output.status.code(),
                        "stdout": String::from_utf8_lossy(&output.stdout),
                        "stderr": String::from_utf8_lossy(&output.stderr),
                    }),
                    transport_ok,
                    "command_output",
                )
            }
        };

        Ok(ExecutionReport {
            workorder_id: workorder.id.clone(),
            scope: workorder.scope.clone(),
            mode: workorder.mode,
            evidence: Evidence::new(workorder.scope.clone(), kind, payload, now),
            transport_ok,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A18 — Worker cannot execute without allow.
    #[test]
    fn a18_worker_requires_allow() {
        let wo = Workorder::new("wo-1", "scope.x", vec!["echo".into(), "hi".into()]);
        // Not allowed by default.
        let err = ShellWorker::execute(&wo, "t0").unwrap_err();
        assert!(matches!(err, DispatchError::NotAllowed(_)));
    }

    /// A19 — Worker returns evidence, not closure.
    #[test]
    fn a19_worker_returns_evidence_not_closure() {
        let wo = Workorder::new("wo-2", "scope.x", vec!["echo".into(), "proof".into()])
            .allowed(true)
            .with_mode(Mode::Real);
        let report = ShellWorker::execute(&wo, "t0").unwrap();
        assert!(!report.is_receipt());
        assert_eq!(report.evidence.scope, "scope.x");
        // Real captured output, not an assertion.
        let stdout = report.evidence.payload.get("stdout").unwrap().as_str().unwrap();
        assert!(stdout.contains("proof"));
        assert!(report.transport_ok);
    }

    /// Dry-run mode does not execute, but still yields evidence.
    #[test]
    fn dry_run_yields_evidence() {
        let wo = Workorder::new("wo-3", "scope.x", vec!["rm".into(), "-rf".into(), "/".into()])
            .allowed(true);
        let report = ShellWorker::execute(&wo, "t0").unwrap();
        assert_eq!(report.mode, Mode::DryRun);
        assert_eq!(report.evidence.payload.get("dry_run").unwrap(), true);
    }
}
