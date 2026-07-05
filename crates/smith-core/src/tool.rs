// crates/smith-core/src/tool.rs
use async_trait::async_trait;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use crate::context::ExecutionContext;
use crate::error::SmithResult;

/// Universal transport for tool parameters.
pub type ToolConfig = Value;

/// Tool execution result.
pub type ToolResult = Value;

/// Base trait for all automation tools.
///
/// # Requirements
/// - `Send + Sync`: Tools may be executed in Tokio's multi-thread runtime.
/// - Stateless: The tool itself does not store execution state, only configuration.
#[async_trait]
pub trait Tool: Send + Sync {
    /// Unique tool name (e.g., `windows.click`).
    fn name(&self) -> &'static str;

    /// Description for documentation and LLM agents.
    fn description(&self) -> &'static str;

    /// JSON Schema for `ToolConfig` validation.
    fn schema(&self) -> Value;

    /// Asynchronous tool execution.
    ///
    /// # Arguments
    /// * `config` - Call parameters (validated via schema)
    /// * `ctx` - Execution context (read/write variables)
    /// * `token` - Cancellation token for graceful shutdown
    async fn execute(
        &self,
        config: ToolConfig,
        ctx: &mut ExecutionContext,
        token: CancellationToken,
    ) -> SmithResult<ToolResult>;
}
