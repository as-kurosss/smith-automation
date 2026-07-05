// crates/smith-windows/src/tools/click.rs
use async_trait::async_trait;
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult};
use tokio_util::sync::CancellationToken;

use crate::element::SafeUIElement;

/// Tool for performing a click on a Windows UI element.
pub struct ClickTool;

impl ClickTool {
    /// Creates a new `ClickTool` instance.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ClickTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for ClickTool {
    fn name(&self) -> &'static str {
        "windows.click"
    }

    fn description(&self) -> &'static str {
        "Performs a click on a UI element stored in the execution context"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "element_key": {
                    "type": "string",
                    "description": "Key in ExecutionContext containing the UIElement"
                },
                "delay_before_ms": {
                    "type": "integer",
                    "minimum": 0,
                    "description": "Delay before execution in milliseconds"
                },
                "delay_after_ms": {
                    "type": "integer",
                    "minimum": 0,
                    "description": "Delay after execution in milliseconds"
                }
            },
            "required": ["element_key"]
        })
    }

    async fn execute(
        &self,
        config: ToolConfig,
        ctx: &mut ExecutionContext,
        token: CancellationToken,
    ) -> SmithResult<ToolResult> {
        // 0. Optional delay before execution
        crate::tools::apply_delay_before(&config).await;

        // 1. Parameter validation (Canon 10.1)
        let element_key = config
            .get("element_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SmithError::InvalidParams("Missing 'element_key'".into()))?;

        // 2. Cancellation check before heavy operation (Canon 5.4)
        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        // 3. Retrieve element from context
        let value = ctx.get(element_key).ok_or_else(|| {
            SmithError::ContextError(format!("Key '{element_key}' not found in context"))
        })?;

        let wrapper = value.try_as_custom::<SafeUIElement>()?;
        let element_clone = wrapper.clone(); // Clone the Arc, not the element itself

        // Use inner() inside spawn_blocking:
        tokio::task::spawn_blocking(move || {
            element_clone
                .inner()
                .click()
                .map_err(|e| SmithError::PlatformError {
                    message: "Click failed".into(),
                    source: Box::new(e),
                })
        })
        .await
        .map_err(|e| SmithError::PlatformError {
            message: "Blocking task join failed".into(),
            source: Box::new(e),
        })??;

        // 4. Optional delay after execution
        crate::tools::apply_delay_after(&config).await;

        Ok(json!({ "status": "clicked" }))
    }
}
