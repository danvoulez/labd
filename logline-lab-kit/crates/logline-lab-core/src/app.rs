//! App / MCP authority boundary.
//!
//! Apps (including MCP clients and model middleware) do not define semantic
//! truth. An app call becomes a *draft* Act for the Lab to admit (A16). An
//! unregistered or ungranted app action is blocked (A17, Operator §13).

use logline_act::Act;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AppError {
    #[error("app `{0}` is not registered")]
    UnknownApp(String),
    #[error("app `{app}` lacks grant for capability `{capability}`")]
    NotGranted { app: String, capability: String },
    #[error("unknown surface `{0}`")]
    UnknownSurface(String),
}

/// A registered app/entity and the capabilities it has been granted.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AppEntry {
    pub app_id: String,
    pub grants: Vec<String>,
}

/// A call coming from an app surface (CLI/MCP/model middleware).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppCall {
    pub app_id: String,
    /// The capability/did the app is requesting (e.g. `draft_observation`).
    pub capability: String,
    /// The proposed subject of the Act.
    pub this: Value,
}

/// Registry of apps allowed to reach the Lab.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppRegistry {
    apps: HashMap<String, AppEntry>,
}

impl AppRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an app/entity.
    pub fn register(&mut self, app_id: impl Into<String>) {
        let app_id = app_id.into();
        self.apps
            .entry(app_id.clone())
            .or_insert(AppEntry { app_id, grants: vec![] });
    }

    /// Issue a capability grant to a registered app.
    pub fn grant(&mut self, app_id: &str, capability: impl Into<String>) -> Result<(), AppError> {
        let entry = self
            .apps
            .get_mut(app_id)
            .ok_or_else(|| AppError::UnknownApp(app_id.to_string()))?;
        let cap = capability.into();
        if !entry.grants.contains(&cap) {
            entry.grants.push(cap);
        }
        Ok(())
    }

    /// Whether an app is registered.
    pub fn is_registered(&self, app_id: &str) -> bool {
        self.apps.contains_key(app_id)
    }

    /// Whether an app holds a capability grant.
    pub fn is_granted(&self, app_id: &str, capability: &str) -> bool {
        self.apps
            .get(app_id)
            .map(|e| e.grants.iter().any(|g| g == capability))
            .unwrap_or(false)
    }

    /// Turn an authorized app call into a *draft* Act (A16). The Act is attributed
    /// to the app (`who`) and starts as a candidate for the Lab to admit. An
    /// unregistered or ungranted app is rejected (A17).
    pub fn draft_act(&self, call: &AppCall, when: &str) -> Result<Act, AppError> {
        if !self.apps.contains_key(&call.app_id) {
            return Err(AppError::UnknownApp(call.app_id.clone()));
        }
        if !self.is_granted(&call.app_id, &call.capability) {
            return Err(AppError::NotGranted {
                app: call.app_id.clone(),
                capability: call.capability.clone(),
            });
        }
        Ok(Act::new(
            json!(format!("app:{}", call.app_id)),
            json!(call.capability),
            call.this.clone(),
            json!(when),
            json!("none"),
            json!("admit_draft_act"),
            json!("carry_as_blocked_act_pending_admission"),
            json!("reject_app_call"),
            json!("draft"),
        ))
    }
}
