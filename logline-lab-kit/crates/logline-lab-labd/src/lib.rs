//! `logline-lab-labd` — the resident Lab host.
//!
//! `labd` keeps a Lab alive: identity (manifest), an optional set of packs, a
//! profile, a local outbox, a spine, evidence, and ghosts. It exposes the
//! generic Lab API and the nine **experience surfaces** (FINAL §11) as library
//! functions — Start, Today, Timeline, Write, Schedule, Workbench, Proof, Learn,
//! Settings — so any surface (CLI/MCP/web/TUI) can wrap the same grammar.
//!
//! The basics stand alone: a Lab forms and completes a first session with only
//! an identity and a profile. Packs are additive complements (FINAL §15).

#![forbid(unsafe_code)]

mod experience;

pub use experience::{
    ProofView, SettingsView, StartView, TimelineView, TodayView, WorkbenchRun, WriteOutcome,
};

use std::path::{Path, PathBuf};

use logline_act::Act;
use logline_lab_conformance::{builtin_vectors, run as run_conformance, ConformanceReport};
use logline_lab_core::{
    evidence::{Evidence, EvidenceLog},
    ghost::{Ghost, GhostLog},
    receipt::{ReceiptCandidate, ReceiptError},
    LabManifest, PackManifest, ProfileManifest,
};
use logline_lab_local::{EmitOutcome, LocalError, LocalOutbox};
use logline_lab_reports::{generate, generate_learning, LabReport, LearningReport};
use logline_lab_spine::{sync, MemorySpine, Spine, SpineError, StoredAct, SyncReport};
use logline_lab_supabase::{SupabaseConfig, SupabaseSpine};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LabError {
    #[error(transparent)]
    Local(#[from] LocalError),
    #[error(transparent)]
    Spine(#[from] SpineError),
    #[error(transparent)]
    Receipt(#[from] ReceiptError),
    #[error("unknown spine kind `{0}` for profile `{1}`")]
    UnknownSpine(String, String),
}

/// Doctor report on a Lab's wiring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoctorReport {
    pub lab_id: String,
    pub packs: Vec<String>,
    pub profile: String,
    pub spine_kind: String,
    pub outbox_entries: usize,
    pub unsynced: usize,
    pub spine_acts: usize,
    pub conformance_green: bool,
    pub ok: bool,
}

fn read_jsonl<T: serde::de::DeserializeOwned>(path: &Path) -> Result<Vec<T>, LabError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = std::fs::read_to_string(path).map_err(|e| LocalError::Io(e.to_string()))?;
    let mut out = Vec::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        out.push(serde_json::from_str(line).map_err(|e| LocalError::Json(e.to_string()))?);
    }
    Ok(out)
}

fn write_jsonl<T: serde::Serialize>(path: &Path, items: &[T]) -> Result<(), LabError> {
    let mut buf = String::new();
    for item in items {
        buf.push_str(&serde_json::to_string(item).map_err(|e| LocalError::Json(e.to_string()))?);
        buf.push('\n');
    }
    std::fs::write(path, buf).map_err(|e| LocalError::Io(e.to_string()))?;
    Ok(())
}

/// A running Lab.
pub struct Lab {
    manifest: LabManifest,
    packs: Vec<PackManifest>,
    profile: ProfileManifest,
    outbox: LocalOutbox,
    spine: Box<dyn Spine>,
    evidence: EvidenceLog,
    ghosts: GhostLog,
    /// Ugly candidates preserved by Write before they could become valid Acts.
    candidates: Vec<Value>,
    /// When set, the Lab is a directory on disk: outbox.jsonl, evidence.jsonl,
    /// ghosts.jsonl, candidates.jsonl. This is what lets a human (via CLI) and an
    /// LLM (via MCP) open the SAME Lab and see the same reality across runs.
    store_dir: Option<PathBuf>,
}

impl Lab {
    /// Initialize a Lab from its manifest, optional packs, and profile. With no
    /// packs this is "the basics" — still fully usable. The profile selects the
    /// spine without changing core (A08).
    pub fn init(
        manifest: LabManifest,
        packs: Vec<PackManifest>,
        profile: ProfileManifest,
    ) -> Result<Self, LabError> {
        let spine = Self::spine_for(&profile)?;
        Ok(Self {
            manifest,
            packs,
            profile,
            outbox: LocalOutbox::in_memory(),
            spine,
            evidence: EvidenceLog::new(),
            ghosts: GhostLog::new(),
            candidates: Vec::new(),
            store_dir: None,
        })
    }

    /// Same as `init` but with a file-backed outbox for durability.
    pub fn init_with_outbox(
        manifest: LabManifest,
        packs: Vec<PackManifest>,
        profile: ProfileManifest,
        outbox_path: impl AsRef<std::path::Path>,
    ) -> Result<Self, LabError> {
        let spine = Self::spine_for(&profile)?;
        Ok(Self {
            manifest,
            packs,
            profile,
            outbox: LocalOutbox::open(outbox_path)?,
            spine,
            evidence: EvidenceLog::new(),
            ghosts: GhostLog::new(),
            candidates: Vec::new(),
            store_dir: None,
        })
    }

    /// Open a Lab as a directory on disk. The directory holds `outbox.jsonl`,
    /// `evidence.jsonl`, `ghosts.jsonl`, and `candidates.jsonl`. Existing state is
    /// loaded and the spine is rehydrated, so the same Lab resumes across runs and
    /// is identical for every surface (CLI, MCP, GUI).
    pub fn open(
        manifest: LabManifest,
        packs: Vec<PackManifest>,
        profile: ProfileManifest,
        dir: impl AsRef<Path>,
    ) -> Result<Self, LabError> {
        let dir = dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir).map_err(|e| LocalError::Io(e.to_string()))?;
        let spine = Self::spine_for(&profile)?;
        let outbox = LocalOutbox::open(dir.join("outbox.jsonl"))?;

        let mut evidence = EvidenceLog::new();
        for ev in read_jsonl::<Evidence>(&dir.join("evidence.jsonl"))? {
            evidence.attach(ev);
        }
        let mut ghosts = GhostLog::new();
        for g in read_jsonl::<Ghost>(&dir.join("ghosts.jsonl"))? {
            ghosts.record(g);
        }
        let candidates = read_jsonl::<Value>(&dir.join("candidates.jsonl"))?;

        let mut lab = Self {
            manifest,
            packs,
            profile,
            outbox,
            spine,
            evidence,
            ghosts,
            candidates,
            store_dir: Some(dir),
        };
        lab.rehydrate()?;
        Ok(lab)
    }

    fn spine_for(profile: &ProfileManifest) -> Result<Box<dyn Spine>, LabError> {
        match profile.spine.as_str() {
            "memory" | "local" | "local-only" => Ok(Box::new(MemorySpine::new())),
            "supabase" | "postgres" => Ok(Box::new(SupabaseSpine::new(SupabaseConfig {
                url: profile
                    .settings
                    .get("url")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                service_key_env: profile
                    .settings
                    .get("service_key_env")
                    .and_then(|v| v.as_str())
                    .unwrap_or("SUPABASE_SERVICE_KEY")
                    .to_string(),
            }))),
            other => Err(LabError::UnknownSpine(other.to_string(), profile.name.clone())),
        }
    }

    // --- accessors ---
    pub fn lab_id(&self) -> &str {
        &self.manifest.lab_id
    }
    pub fn packs(&self) -> &[PackManifest] {
        &self.packs
    }
    pub fn profile(&self) -> &ProfileManifest {
        &self.profile
    }
    pub fn spine(&self) -> &dyn Spine {
        self.spine.as_ref()
    }
    pub fn evidence(&self) -> &EvidenceLog {
        &self.evidence
    }
    pub fn ghosts(&self) -> &GhostLog {
        &self.ghosts
    }
    pub fn candidates(&self) -> &[Value] {
        &self.candidates
    }

    // --- core API ---
    pub fn emit(&mut self, act: &Act) -> Result<EmitOutcome, LabError> {
        Ok(self.outbox.emit(act)?)
    }
    pub fn sync(&mut self) -> Result<SyncReport, LabError> {
        Ok(sync(&mut self.outbox, self.spine.as_mut())?)
    }

    /// Rebuild the spine from the durable outbox (idempotent). The file-backed
    /// outbox is the resumable source of state across runs; the spine is a
    /// derived store, never the truth.
    pub fn rehydrate(&mut self) -> Result<usize, LabError> {
        let acts: Vec<Act> = self.outbox.list().iter().map(|e| e.act.clone()).collect();
        for a in &acts {
            self.spine.ingest(a)?;
        }
        Ok(acts.len())
    }
    pub fn attach_evidence(&mut self, evidence: Evidence) {
        self.evidence.attach(evidence);
        let _ = self.persist_evidence();
    }
    pub fn record_ghost(&mut self, ghost: Ghost) {
        self.ghosts.record(ghost);
        let _ = self.persist_ghosts();
    }

    /// Preserve an ugly candidate (used by the Write surface), persisting it.
    pub(crate) fn push_candidate(&mut self, value: Value) -> Result<(), LabError> {
        self.candidates.push(value);
        self.persist_candidates()
    }

    fn persist_evidence(&self) -> Result<(), LabError> {
        if let Some(dir) = &self.store_dir {
            write_jsonl(&dir.join("evidence.jsonl"), self.evidence.all())?;
        }
        Ok(())
    }
    fn persist_ghosts(&self) -> Result<(), LabError> {
        if let Some(dir) = &self.store_dir {
            write_jsonl(&dir.join("ghosts.jsonl"), self.ghosts.all())?;
        }
        Ok(())
    }
    fn persist_candidates(&self) -> Result<(), LabError> {
        if let Some(dir) = &self.store_dir {
            write_jsonl(&dir.join("candidates.jsonl"), &self.candidates)?;
        }
        Ok(())
    }
    pub fn prepare_receipt(&self, act: &Act, scope: &str) -> Result<ReceiptCandidate, LabError> {
        Ok(ReceiptCandidate::prepare(act, scope, &self.evidence)?)
    }
    pub fn get(&self, content_hash: &str) -> Option<StoredAct> {
        self.spine.get(content_hash)
    }
    pub fn report(&self, now: &str) -> LabReport {
        generate(self.spine.as_ref(), &self.manifest.lab_id, now)
    }
    pub fn learning(&self, now: &str) -> LearningReport {
        generate_learning(self.spine.as_ref(), &self.ghosts, &self.manifest.lab_id, now)
    }

    /// Run the built-in protocol conformance suite (offline).
    pub fn conformance(&self) -> ConformanceReport {
        run_conformance(&builtin_vectors())
    }

    /// Doctor: inspect the Lab's wiring (includes an offline conformance check).
    pub fn doctor(&self) -> DoctorReport {
        let conformance_green = self.conformance().is_green();
        DoctorReport {
            lab_id: self.manifest.lab_id.clone(),
            packs: self.packs.iter().map(|p| p.name.clone()).collect(),
            profile: self.profile.name.clone(),
            spine_kind: self.spine.kind().to_string(),
            outbox_entries: self.outbox.len(),
            unsynced: self.outbox.unsynced().len(),
            spine_acts: self.spine.all().len(),
            conformance_green,
            ok: conformance_green,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    pub(crate) fn basics_lab() -> Lab {
        // The basics: identity + profile, NO pack.
        let manifest =
            LabManifest::load(r#"{"lab_id":"basics.local.lab","profile":"local-only"}"#).unwrap();
        let profile = ProfileManifest::load(r#"{"name":"local-only","spine":"memory"}"#).unwrap();
        Lab::init(manifest, vec![], profile).unwrap()
    }

    fn act(did: &str) -> Act {
        Act::new(
            json!("lab.operator"),
            json!(did),
            json!({"k": did}),
            json!("2026-06-06T00:00:00Z"),
            json!("none"),
            json!("record"),
            json!("carry_blocked"),
            json!("reject"),
            json!("candidate"),
        )
    }

    /// Basics-first: a Lab with no pack still emits, syncs, reports (A06/A11/A12).
    #[test]
    fn basics_lab_runs_without_a_pack() {
        let mut lab = basics_lab();
        assert!(lab.packs().is_empty());
        let outcome = lab.emit(&act("declare_lab")).unwrap();
        lab.sync().unwrap();
        assert!(lab.get(outcome.content_hash()).is_some());
        assert_eq!(lab.report("t0").total_acts, 1);
        assert!(lab.doctor().conformance_green);
    }

    #[test]
    fn supabase_profile_selects_supabase_spine() {
        let manifest =
            LabManifest::load(r#"{"lab_id":"x","profile":"supabase-default"}"#).unwrap();
        let profile = ProfileManifest::load(
            r#"{"name":"supabase-default","spine":"supabase","settings":{"url":"https://x.supabase.co"}}"#,
        )
        .unwrap();
        let lab = Lab::init(manifest, vec![], profile).unwrap();
        assert_eq!(lab.doctor().spine_kind, "supabase");
    }
}
