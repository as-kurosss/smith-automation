// crates/smith-windows/src/tools/wait.rs
use async_trait::async_trait;
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult};
use tokio_util::sync::CancellationToken;

/// Tool for pausing (sleep) in automation scenarios.
///
/// Allows inserting a delay between steps, for example to wait for
/// UI animation, window opening, or data processing.
pub struct WaitTool;

impl WaitTool {
    /// Creates a new `WaitTool` instance.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for WaitTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for WaitTool {
    fn name(&self) -> &'static str {
        "windows.wait"
    }

    fn description(&self) -> &'static str {
        "Pauses execution for the specified duration (in milliseconds)"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "duration_ms": {
                    "type": "integer",
                    "description": "Delay duration in milliseconds"
                }
            },
            "required": ["duration_ms"]
        })
    }

    async fn execute(
        &self,
        config: ToolConfig,
        _ctx: &mut ExecutionContext,
        token: CancellationToken,
    ) -> SmithResult<ToolResult> {
        // 1. Parameter validation (Canon 10.1)
        let duration_ms = config
            .get("duration_ms")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| SmithError::InvalidParams("Missing or invalid 'duration_ms'".into()))?;

        // 2. Cancellation check (Canon 5.4)
        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        // 3. Pause with cancellation support
        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(duration_ms)) => {}
            _ = token.cancelled() => {
                return Err(SmithError::Cancelled);
            }
        }

        Ok(json!({ "status": "waited", "duration_ms": duration_ms }))
    }
}
