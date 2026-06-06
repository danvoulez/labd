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

/// A profile: an infrastructure choice (local-only / postgres / supabase /
/// filesystem-manual). A profile selects a spine adapter; it does not change core.
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
}

/// A Lab instance: identity + chosen profile + chosen pack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabManifest {
    pub lab_id: String,
    pub profile: String,
    pub pack: String,
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
        Ok(m)
    }
}
