//! `logline-lab-mcp-server` — a generic MCP/app connection surface.
//!
//! This is an *app boundary*, not authority. A tool call from a connected client
//! is turned into a **draft Act** for the Lab to admit (A16). Unregistered or
//! ungranted clients are rejected (A17). All semantic decisions remain with the
//! Lab core; the MCP server only marshals calls.
//!
//! GHOST `mcp-server-as-ts`: Operator §10 also lists a TypeScript MCP package.
//! The Rust-vs-TS surface strategy is an open decision (build-pack ghost 06); v0
//! implements the boundary logic in Rust so it is testable in-tree.

#![forbid(unsafe_code)]

use logline_act::Act;
use logline_lab_core::{AppCall, AppError, AppRegistry};
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

    /// Handle a tool call: produce a draft Act, or reject if unauthorized.
    pub fn handle(&self, call: &ToolCall, now: &str) -> Result<Act, AppError> {
        let app_call = AppCall {
            app_id: call.app_id.clone(),
            capability: call.tool.clone(),
            this: call.arguments.clone(),
        };
        self.registry.draft_act(&app_call, now)
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
}
