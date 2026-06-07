//! Lab / pack / profile manifests and their loaders.
//!
//! Packs and profiles are loaded as *data*. Loading one must never change core
//! semantics (A14/A15, Operator §10). The loaders here are pure functions over
//! JSON; they return owned manifest values and touch no global core state.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ManifestError {
    #[error("invalid manifest json: {0}")]
    Json(String),
    #[error("manifest is missing required field `{0}`")]
    MissingField(&'static str),
}

/// A pack: opinionated conventions layered over the generic kit. A pack may
/// declare named scopes and `did` vocabularies, but it cannot redefine the nine
/// slots or core validation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackManifest {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub did_vocabulary: Vec<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub extra: Value,
}

impl PackManifest {
    pub fn load(text: &str) -> Result<Self, ManifestError> {
        let m: PackManifest =
            serde_json::from_str(text).map_err(|e| ManifestError::Json(e.to_string()))?;
        if m.name.trim().is_empty() {
            return Err(ManifestError::MissingField("name"));
        }
        Ok(m)
    }
}

/// The protocol grade of a Lab's storage, derived from its Spine Profile.
///
/// A local file/outbox is capture/transport/cache — never the protocol-grade home
/// of admitted Acts (RELEASE_SCOPE §1). Admitted Acts are only publication-grade
/// when a declared external Spine Profile is configured.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Grade {
    /// Capture candidates only; no protocol-grade admission.
    CandidateOnly,
    /// Local append-only Act-log spine; dev-only / unregistered / non-publication.
    DevEphemeral,
    /// A declared external Spine Profile; publication-grade admitted Acts.
    Publication,
}

impl Grade {
    pub fn is_publication(&self) -> bool {
        matches!(self, Grade::Publication)
    }
    /// Honest label applied to runs/Acts admitted under this grade.
    pub fn run_label(&self) -> &'static str {
        match self {
            Grade::CandidateOnly => "candidate-only (no admitted Acts)",
            Grade::DevEphemeral => "dev-only / unregistered / non-publication-grade",
            Grade::Publication => "publication-grade",
        }
    }
    pub fn warning(&self) -> Option<&'static str> {
        match self {
            Grade::CandidateOnly => Some(
                "candidate-only: this Lab captures candidates but cannot admit \
                 protocol-grade Acts. Configure a Spine Profile to admit.",
            ),
            Grade::DevEphemeral => Some(
                "dev-ephemeral: admitted Acts live in a local Act-log and are \
                 NOT publication-grade. Configure an external Spine Profile to publish.",
            ),
            Grade::Publication => None,
        }
    }
}

/// A profile: an infrastructure choice (the Spine Profile). A profile selects a
/// spine adapter; it does not change core.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileManifest {
    pub name: String,
    /// Spine kind selected by this profile.
    pub spine: String,
    #[serde(default)]
    pub settings: Value,
}

impl ProfileManifest {
    pub fn load(text: &str) -> Result<Self, ManifestError> {
        let m: ProfileManifest =
            serde_json::from_str(text).map_err(|e| ManifestError::Json(e.to_string()))?;
        if m.name.trim().is_empty() {
            return Err(ManifestError::MissingField("name"));
        }
        if m.spine.trim().is_empty() {
            return Err(ManifestError::MissingField("spine"));
        }
        Ok(m)
    }

    /// The protocol grade implied by this profile's spine kind.
    pub fn grade(&self) -> Grade {
        match self.spine.as_str() {
            "candidate-only" => Grade::CandidateOnly,
            "dev-ephemeral" | "memory" | "local" | "local-only" => Grade::DevEphemeral,
            "postgres" | "neon" | "supabase" | "byo" | "bring-your-own" => Grade::Publication,
            // Unknown spine kinds are treated conservatively as dev-ephemeral.
            _ => Grade::DevEphemeral,
        }
    }
}

/// A Lab instance: identity + chosen profile, plus *optional* packs.
///
/// A Lab needs only an identity and a profile to form and run a first session —
/// that is "the basics". Packs are purely additive complements (Operator/FINAL
/// §15): `pack` may be absent, a single name, or several (`packs`). This is a
/// permanent architectural choice, not a convenience: the kit must be useful
/// before any pack is loaded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabManifest {
    pub lab_id: String,
    pub profile: String,
    /// Optional single pack (back-compat / common case).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack: Option<String>,
    /// Optional additional packs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packs: Vec<String>,
    #[serde(default)]
    pub settings: Value,
}

impl LabManifest {
    pub fn load(text: &str) -> Result<Self, ManifestError> {
        let m: LabManifest =
            serde_json::from_str(text).map_err(|e| ManifestError::Json(e.to_string()))?;
        if m.lab_id.trim().is_empty() {
            return Err(ManifestError::MissingField("lab_id"));
        }
        if m.profile.trim().is_empty() {
            return Err(ManifestError::MissingField("profile"));
        }
        Ok(m)
    }

    /// Every pack this Lab requests (single + list), de-duplicated in order.
    pub fn all_packs(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for p in self.pack.iter().chain(self.packs.iter()) {
            if !out.contains(p) {
                out.push(p.clone());
            }
        }
        out
    }
}
