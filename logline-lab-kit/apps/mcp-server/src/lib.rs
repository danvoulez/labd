//! `logline-lab-mcp-server` — a generic MCP/app connection surface.
//!
//! This is an *app boundary*, not authority. A tool call from a connected client
//! is turned into a **draft Act** for the Lab to admit (A16). Unregistered or
//! ungranted clients are rejected (A17). All semantic decisions remain with the
//! Lab core; the MCP server only marshals calls.
//!
//! It also exposes the Lab's **read surfaces** (Start/Today/Timeline/Schedule/
//! Learn/Settings) under read grants, returning the *same JSON* a human sees via
//! the CLI — so a human and an LLM open the same Lab and look at the same reality.
//!
//! GHOST `mcp-server-as-ts`: Operator §10 also lists a TypeScript MCP package.
//! The Rust-vs-TS surface strategy is an open decision (build-pack ghost 06); v0
//! implements the boundary logic in Rust so it is testable in-tree.

#![forbid(unsafe_code)]

use logline_act::Act;
use logline_lab_core::{AppCall, AppError, AppRegistry};
use logline_lab_labd::Lab;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A tool call arriving over the MCP surface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    pub app_id: String,
    pub tool: String,
    pub arguments: Value,
}

/// A thin MCP server holding the app registry.
#[derive(Default)]
pub struct McpServer {
    registry: AppRegistry,
}

impl McpServer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a connecting app/entity.
    pub fn register_app(&mut self, app_id: impl Into<String>) {
        self.registry.register(app_id);
    }

    /// Grant a capability (tool) to an app.
    pub fn grant(&mut self, app_id: &str, tool: impl Into<String>) -> Result<(), AppError> {
        self.registry.grant(app_id, tool)
    }

    /// Grant a read capability for a surface (e.g. `today` ⇒ grant `read:today`).
    pub fn grant_read(&mut self, app_id: &str, surface: &str) -> Result<(), AppError> {
        self.registry.grant(app_id, format!("read:{surface}"))
    }

    /// Handle a tool call: produce a draft Act, or reject if unauthorized.
    pub fn handle(&self, call: &ToolCall, now: &str) -> Result<Act, AppError> {
        let app_call = AppCall {
            app_id: call.app_id.clone(),
            capability: call.tool.clone(),
            this: call.arguments.clone(),
        };
        self.registry.draft_act(&app_call, now)
    }

    /// Render a read-only surface for a granted app. Returns the SAME JSON a human
    /// sees via `labkit <surface>` — one shared reality for human and LLM.
    pub fn read_surface(
        &self,
        lab: &Lab,
        app_id: &str,
        surface: &str,
        now: &str,
    ) -> Result<Value, AppError> {
        if !self.registry.is_registered(app_id) {
            return Err(AppError::UnknownApp(app_id.to_string()));
        }
        let cap = format!("read:{surface}");
        if !self.registry.is_granted(app_id, &cap) {
            return Err(AppError::NotGranted {
                app: app_id.to_string(),
                capability: cap,
            });
        }
        lab.render_surface(surface, now)
            .ok_or_else(|| AppError::UnknownSurface(surface.to_string()))
    }

    /// The read surfaces an MCP client may request.
    pub fn read_surfaces() -> &'static [&'static str] {
        Lab::read_surfaces()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn authorized_tool_call_becomes_draft_act() {
        let mut server = McpServer::new();
        server.register_app("cockpit");
        server.grant("cockpit", "draft_observation").unwrap();
        let call = ToolCall {
            app_id: "cockpit".into(),
            tool: "draft_observation".into(),
            arguments: json!({"note": "ok"}),
        };
        let act = server.handle(&call, "t0").unwrap();
        assert_eq!(act.status.as_str(), Some("draft"));
        assert_eq!(act.who.as_str(), Some("app:cockpit"));
    }

    #[test]
    fn unauthorized_tool_call_rejected() {
        let server = McpServer::new();
        let call = ToolCall {
            app_id: "rogue".into(),
            tool: "draft_observation".into(),
            arguments: json!({}),
        };
        assert!(server.handle(&call, "t0").is_err());
    }

    fn demo_lab() -> Lab {
        use logline_lab_core::{LabManifest, ProfileManifest};
        let manifest =
            LabManifest::load(r#"{"lab_id":"same.map.lab","profile":"local-only"}"#).unwrap();
        let profile = ProfileManifest::load(r#"{"name":"local-only","spine":"memory"}"#).unwrap();
        Lab::init(manifest, vec![], profile).unwrap()
    }

    /// The LLM (via MCP read) and the human (via the library/CLI surface) see the
    /// EXACT same reality — byte-identical JSON.
    #[test]
    fn human_and_llm_see_the_same_map() {
        let lab = demo_lab();
        let mut server = McpServer::new();
        server.register_app("assistant");
        for s in McpServer::read_surfaces() {
            server.grant_read("assistant", s).unwrap();
        }
        for surface in McpServer::read_surfaces() {
            let human = lab.render_surface(surface, "2026-06-07T00:00:00Z").unwrap();
            let llm = server
                .read_surface(&lab, "assistant", surface, "2026-06-07T00:00:00Z")
                .unwrap();
            assert_eq!(human, llm, "surface `{surface}` differs between human and LLM");
        }
    }

    /// An ungranted app cannot read a surface (A17 / authority boundary).
    #[test]
    fn ungranted_read_is_blocked() {
        let lab = demo_lab();
        let mut server = McpServer::new();
        server.register_app("assistant"); // registered, but no read grants
        assert!(server.read_surface(&lab, "assistant", "today", "t0").is_err());
        assert!(server.read_surface(&lab, "rogue", "today", "t0").is_err());
    }
}
