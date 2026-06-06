//! `logline-lab-labd` — the generic Lab host.
//!
//! `labd` is the runtime that holds a Lab's identity (manifest), its chosen pack
//! and profile, a local outbox, and a spine. It exposes the generic Lab API:
//! emit, sync, attach evidence, prepare receipt candidates, project, report, and
//! doctor. It is generic machinery — no pack-specific or Dan-specific semantics
//! (Operator §13).

#![forbid(unsafe_code)]

use logline_act::Act;
use logline_lab_core::{
    evidence::{Evidence, EvidenceLog},
    receipt::{ReceiptCandidate, ReceiptError},
    LabManifest, PackManifest, ProfileManifest,
};
use logline_lab_local::{EmitOutcome, LocalError, LocalOutbox};
use logline_lab_reports::{generate, LabReport};
use logline_lab_spine::{sync, MemorySpine, Spine, SpineError, StoredAct, SyncReport};
use logline_lab_supabase::{SupabaseConfig, SupabaseSpine};
use serde::{Deserialize, Serialize};
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
    pub pack: String,
    pub profile: String,
    pub spine_kind: String,
    pub outbox_entries: usize,
    pub unsynced: usize,
    pub spine_acts: usize,
    pub ok: bool,
}

/// A running Lab.
pub struct Lab {
    manifest: LabManifest,
    pack: PackManifest,
    profile: ProfileManifest,
    outbox: LocalOutbox,
    spine: Box<dyn Spine>,
    evidence: EvidenceLog,
}

impl Lab {
    /// Initialize a Lab from its manifest, pack, and profile. The profile selects
    /// the spine adapter without changing core (A15).
    pub fn init(
        manifest: LabManifest,
        pack: PackManifest,
        profile: ProfileManifest,
    ) -> Result<Self, LabError> {
        let spine = Self::spine_for(&profile)?;
        Ok(Self {
            manifest,
            pack,
            profile,
            outbox: LocalOutbox::in_memory(),
            spine,
            evidence: EvidenceLog::new(),
        })
    }

    /// Same as `init` but with a file-backed outbox for durability.
    pub fn init_with_outbox(
        manifest: LabManifest,
        pack: PackManifest,
        profile: ProfileManifest,
        outbox_path: impl AsRef<std::path::Path>,
    ) -> Result<Self, LabError> {
        let spine = Self::spine_for(&profile)?;
        Ok(Self {
            manifest,
            pack,
            profile,
            outbox: LocalOutbox::open(outbox_path)?,
            spine,
            evidence: EvidenceLog::new(),
        })
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
            other => Err(LabError::UnknownSpine(
                other.to_string(),
                profile.name.clone(),
            )),
        }
    }

    pub fn lab_id(&self) -> &str {
        &self.manifest.lab_id
    }
    pub fn pack(&self) -> &PackManifest {
        &self.pack
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

    /// Emit an Act into the local outbox.
    pub fn emit(&mut self, act: &Act) -> Result<EmitOutcome, LabError> {
        Ok(self.outbox.emit(act)?)
    }

    /// Sync the outbox to the configured spine.
    pub fn sync(&mut self) -> Result<SyncReport, LabError> {
        Ok(sync(&mut self.outbox, self.spine.as_mut())?)
    }

    /// Attach evidence to a scope.
    pub fn attach_evidence(&mut self, evidence: Evidence) {
        self.evidence.attach(evidence);
    }

    /// Prepare a scoped receipt candidate (requires evidence).
    pub fn prepare_receipt(
        &self,
        act: &Act,
        scope: &str,
    ) -> Result<ReceiptCandidate, LabError> {
        Ok(ReceiptCandidate::prepare(act, scope, &self.evidence)?)
    }

    /// Read an Act back from the spine.
    pub fn get(&self, content_hash: &str) -> Option<StoredAct> {
        self.spine.get(content_hash)
    }

    /// Generate a Lab report from projections.
    pub fn report(&self, now: &str) -> LabReport {
        generate(self.spine.as_ref(), &self.manifest.lab_id, now)
    }

    /// Doctor: inspect the Lab's wiring.
    pub fn doctor(&self) -> DoctorReport {
        DoctorReport {
            lab_id: self.manifest.lab_id.clone(),
            pack: self.pack.name.clone(),
            profile: self.profile.name.clone(),
            spine_kind: self.spine.kind().to_string(),
            outbox_entries: self.outbox.len(),
            unsynced: self.outbox.unsynced().len(),
            spine_acts: self.spine.all().len(),
            ok: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn lab() -> Lab {
        let manifest = LabManifest::load(
            r#"{"lab_id":"test.local.lab","profile":"local-only","pack":"demo"}"#,
        )
        .unwrap();
        let pack = PackManifest::load(r#"{"name":"demo","version":"0.1.0"}"#).unwrap();
        let profile =
            ProfileManifest::load(r#"{"name":"local-only","spine":"memory"}"#).unwrap();
        Lab::init(manifest, pack, profile).unwrap()
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

    #[test]
    fn lab_end_to_end_emit_sync_report() {
        let mut lab = lab();
        let outcome = lab.emit(&act("declare_lab")).unwrap();
        lab.sync().unwrap();
        assert!(lab.get(outcome.content_hash()).is_some());

        let report = lab.report("2026-06-06T12:00:00Z");
        assert_eq!(report.total_acts, 1);

        let doctor = lab.doctor();
        assert_eq!(doctor.spine_kind, "memory");
        assert_eq!(doctor.spine_acts, 1);
        assert_eq!(doctor.unsynced, 0);
    }

    #[test]
    fn supabase_profile_selects_supabase_spine() {
        let manifest =
            LabManifest::load(r#"{"lab_id":"x","profile":"supabase","pack":"demo"}"#).unwrap();
        let pack = PackManifest::load(r#"{"name":"demo"}"#).unwrap();
        let profile = ProfileManifest::load(
            r#"{"name":"supabase","spine":"supabase","settings":{"url":"https://x.supabase.co"}}"#,
        )
        .unwrap();
        let lab = Lab::init(manifest, pack, profile).unwrap();
        assert_eq!(lab.doctor().spine_kind, "supabase");
    }
}
