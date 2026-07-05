// crates/smith-windows/src/tools/find.rs
use async_trait::async_trait;
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult};
use tokio_util::sync::CancellationToken;

use crate::element::SafeUIElement;
use crate::selector::ElementSelector;

/// Tool for finding a UI element on the Windows desktop.
///
/// Accepts selector parameters (name, `automation_id`, `control_type`, `class_name`, pid)
/// and saves the found element in the context under the specified key.
pub struct FindTool;

impl FindTool {
    /// Creates a new `FindTool` instance.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for FindTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for FindTool {
    fn name(&self) -> &'static str {
        "windows.find"
    }

    fn description(&self) -> &'static str {
        "Finds a Windows UI element matching the specified selectors and stores it in context"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Element name to match"
                },
                "automation_id": {
                    "type": "string",
                    "description": "UI Automation identifier"
                },
                "control_type": {
                    "type": "string",
                    "description": "Control type (e.g. Button, Edit, Window)"
                },
                "class_name": {
                    "type": "string",
                    "description": "Window class name"
                },
                "pid": {
                    "type": "integer",
                    "description": "Process ID filter"
                },
                "output_key": {
                    "type": "string",
                    "description": "Key to store the found element in execution context"
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
            "required": ["output_key"]
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
        let output_key = config
            .get("output_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SmithError::InvalidParams("Missing 'output_key'".into()))?;

        // 2. Cancellation check (Canon 5.4)
        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        // 3. Build selector from config
        let mut selector = ElementSelector::new();

        if let Some(name) = config.get("name").and_then(|v| v.as_str()) {
            selector = selector.name(name);
        }
        if let Some(aid) = config.get("automation_id").and_then(|v| v.as_str()) {
            selector = selector.automation_id(aid);
        }
        if let Some(ct) = config.get("control_type").and_then(|v| v.as_str()) {
            selector = selector.control_type(ct);
        }
        if let Some(cn) = config.get("class_name").and_then(|v| v.as_str()) {
            selector = selector.class_name(cn);
        }
        if let Some(pid) = config.get("pid").and_then(serde_json::Value::as_u64) {
            let pid = u32::try_from(pid).map_err(|_| {
                SmithError::InvalidParams(format!("PID {pid} out of range (max u32)"))
            })?;
            selector = selector.pid(pid);
        }

        // 4. Search in blocking thread (COM calls).
        //    SafeUIElement is created inside spawn_blocking because UIElement is not Send.
        let safe_element = tokio::task::spawn_blocking(move || {
            selector.find_from_desktop().map(SafeUIElement::new)
        })
        .await
        .map_err(|e| SmithError::PlatformError {
            message: "Find element blocking task failed".into(),
            source: Box::new(e),
        })??;

        // 5. Save result to context
        ctx.set(
            output_key.to_string(),
            smith_core::ContextValue::Custom(std::sync::Arc::new(safe_element)),
        );

        // 6. Optional delay after execution
        crate::tools::apply_delay_after(&config).await;

        Ok(json!({ "status": "found" }))
    }
}
