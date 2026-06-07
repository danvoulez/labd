# LLM / agent flow (same Lab, read-only + proposals)

An LLM never decides. It reads the same surfaces the human reads and proposes
draft Acts under grants. Boundary code: `apps/mcp-server`.

```rust
use logline_lab_mcp_server::{McpServer, ToolCall};

let mut mcp = McpServer::new();
mcp.register_app("assistant");
for s in McpServer::read_surfaces() {            // start, today, timeline, schedule, learn, settings
    mcp.grant_read("assistant", s).unwrap();
}
mcp.grant("assistant", "draft_observation").unwrap();

// 1. The LLM reads exactly what the human sees (byte-identical JSON):
let today = mcp.read_surface(&lab, "assistant", "today", now)?;   // == `labkit today`
let proof_inputs = mcp.read_surface(&lab, "assistant", "schedule", now)?;

// 2. The LLM PROPOSES (it does not decide): a tool call becomes a *draft* Act.
let draft = mcp.handle(&ToolCall {
    app_id: "assistant".into(),
    tool: "draft_observation".into(),
    arguments: serde_json::json!({"note": "tunnel looks healthy"}),
}, now)?;                                          // status == "draft"; who == "app:assistant"
// A human still authorizes/admits the draft. Model text is never evidence.
```

What the LLM is good for here: translate natural language into candidate Acts,
compare runs, explain why something is blocked or ghosted, warn where proof is
missing, and suggest the next study — all over the **same map** the human sees.
