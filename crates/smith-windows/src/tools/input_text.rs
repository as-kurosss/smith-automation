// crates/smith-windows/src/tools/input_text.rs
use async_trait::async_trait;
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult};
use tokio_util::sync::CancellationToken;

/// Tool for inputting text into a Windows UI element.
///
/// If `element_key` or `selector` is specified, first focuses the element,
/// then types text via UIA `send_keys`. If no element is specified,
/// simply sends keystrokes to the active window.
pub struct InputTextTool;

impl InputTextTool {
    /// Creates a new `InputTextTool` instance.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for InputTextTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for InputTextTool {
    fn name(&self) -> &'static str {
        "windows.input_text"
    }

    fn description(&self) -> &'static str {
        "Types text into a UI element (or sends keystrokes if no element specified)"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "text": {
                    "type": "string",
                    "description": "Text to type (use {} for special keys, e.g. {enter})"
                },
                "element_key": {
                    "type": "string",
                    "description": "Key in ExecutionContext containing a UIElement to focus first"
                },
                "name": {
                    "type": "string",
                    "description": "Element name to find (if element_key not set)"
                },
                "automation_id": {
                    "type": "string",
                    "description": "UI Automation identifier (if element_key not set)"
                },
                "control_type": {
                    "type": "string",
                    "description": "Control type (if element_key not set)"
                },
                "class_name": {
                    "type": "string",
                    "description": "Window class name (if element_key not set)"
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
            "required": ["text"]
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

        // 1. Parameter validation
        let text = config
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SmithError::InvalidParams("Missing 'text'".into()))?
            .to_string();

        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        // 2. Try to get element from context or find by selector
        let maybe_element = crate::tools::resolve_element_from_config(&config, ctx).await?;

        // 3. Input text in blocking thread
        tokio::task::spawn_blocking(move || {
            if let Some(element) = maybe_element {
                // Focus the element, then type text
                element
                    .inner()
                    .set_focus()
                    .map_err(|e| SmithError::PlatformError {
                        message: "Set focus failed".into(),
                        source: Box::new(e),
                    })?;
                // Small pause after set_focus to let UI Automation
                // process the focus and update the element state before send_keys.
                std::thread::sleep(std::time::Duration::from_millis(100));
                element
                    .inner()
                    .send_keys(&text, 0)
                    .map_err(|e| SmithError::PlatformError {
                        message: "send_keys failed".into(),
                        source: Box::new(e),
                    })?;
            } else {
                // Simply type text into the active window via the root element
                let automation = uiautomation::core::UIAutomation::new().map_err(|e| {
                    SmithError::PlatformError {
                        message: "UIAutomation init failed".into(),
                        source: Box::new(e),
                    }
                })?;
                let root =
                    automation
                        .get_root_element()
                        .map_err(|e| SmithError::PlatformError {
                            message: "Get root element failed".into(),
                            source: Box::new(e),
                        })?;
                root.send_keys(&text, 0)
                    .map_err(|e| SmithError::PlatformError {
                        message: "send_keys failed".into(),
                        source: Box::new(e),
                    })?;
            }
            Ok::<_, SmithError>(())
        })
        .await
        .map_err(|e| SmithError::PlatformError {
            message: "Input text blocking task join failed".into(),
            source: Box::new(e),
        })??;

        // 4. Optional delay after execution
        crate::tools::apply_delay_after(&config).await;

        Ok(json!({ "status": "input_sent" }))
    }
}
