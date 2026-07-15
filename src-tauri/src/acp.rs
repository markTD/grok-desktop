//! ACP (Agent Client Protocol) client for `grok agent stdio`.
//!
//! Phase 0: module + types only. Phase 1 will spawn the process and speak
//! JSON-RPC over stdin/stdout (see docs/PRODUCT.md).

use serde::{Deserialize, Serialize};

/// High-level connection state exposed to the UI later.
#[allow(dead_code)] // wired up in the ACP spawn milestone
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AcpConnectionState {
    pub connected: bool,
    pub session_id: Option<String>,
    pub cwd: Option<String>,
    pub last_error: Option<String>,
}

/// Streaming update kinds from Grok ACP `session/update` notifications.
/// Mirrors the documented `sessionUpdate` field values.
#[allow(dead_code)] // wired up in the ACP spawn milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionUpdateKind {
    AgentMessageChunk,
    AgentThoughtChunk,
    ToolCall,
    ToolCallUpdate,
    Plan,
    Unknown(String),
}

/// Placeholder for a future `spawn_agent` command.
///
/// Planned flow:
/// 1. `Command::new(bin).args(["agent", "stdio"])`
/// 2. JSON-RPC `initialize`
/// 3. `session/new` with `cwd`
/// 4. `session/prompt` + parse `session/update` lines
pub fn planned_stdio_args() -> &'static [&'static str] {
    &["agent", "stdio"]
}
