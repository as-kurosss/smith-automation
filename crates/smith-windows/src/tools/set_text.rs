// crates/smith-windows/src/tools/set_text.rs
use async_trait::async_trait;
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult};
use tokio_util::sync::CancellationToken;

use crate::element::SafeUIElement;
use crate::selector::ElementSelector;

/// Инструмент для установки текста через UI Automation ValuePattern.
///
/// В отличие от `InputTextTool`, этот инструмент напрямую устанавливает
/// значение текстового поля через `ValuePattern::set_value()`, что
/// работает быстрее, но не имитирует реальный ввод с клавиатуры.
pub struct SetTextTool;

impl SetTextTool {
    /// Создаёт новый экземпляр `SetTextTool`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for SetTextTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for SetTextTool {
    fn name(&self) -> &'static str {
        "windows.set_text"
    }

    fn description(&self) -> &'static str {
        "Sets the text value of a UI element via ValuePattern"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "text": {
                    "type": "string",
                    "description": "Text value to set"
                },
                "element_key": {
                    "type": "string",
                    "description": "Key in ExecutionContext containing a UIElement"
                },
                "name": {
                    "type": "string",
                    "description": "Element name to find (if element_key not set)"
                },
                "automation_id": {
                    "type": "string",
                    "description": "UI Automation identifier"
                },
                "control_type": {
                    "type": "string",
                    "description": "Control type"
                },
                "class_name": {
                    "type": "string",
                    "description": "Window class name"
                }
            },
            "required": ["text"],
            "oneOf": [
                { "required": ["element_key"] },
                { "required": ["name"] },
                { "required": ["automation_id"] }
            ]
        })
    }

    async fn execute(
        &self,
        config: ToolConfig,
        ctx: &mut ExecutionContext,
        token: CancellationToken,
    ) -> SmithResult<ToolResult> {
        // 1. Валидация параметров
        let text = config
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SmithError::InvalidParams("Missing 'text'".into()))?
            .to_string();

        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        // 2. Получаем элемент
        let element: SafeUIElement =
            if let Some(element_key) = config.get("element_key").and_then(|v| v.as_str()) {
                let value = ctx.get(element_key).ok_or_else(|| {
                    SmithError::ContextError(format!("Key '{element_key}' not found in context"))
                })?;
                value.try_as_custom::<SafeUIElement>()?.clone()
            } else {
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

                let safe_element = tokio::task::spawn_blocking(move || {
                    selector
                        .find_from_desktop()
                        .map(SafeUIElement::new)
                })
                .await
                .map_err(|e| SmithError::PlatformWithCause {
                    message: "Find element blocking task failed".into(),
                    source: Box::new(e),
                })??;

                safe_element
            };

        // 3. Устанавливаем текст через ValuePattern в блокирующем потоке
        tokio::task::spawn_blocking(move || {
            let pattern = element
                .inner()
                .get_pattern::<uiautomation::patterns::UIValuePattern>()
                .map_err(|e| SmithError::PlatformWithCause {
                    message: "Get ValuePattern failed".into(),
                    source: Box::new(e),
                })?;
            pattern
                .set_value(&text)
                .map_err(|e| SmithError::PlatformWithCause {
                    message: "Set value failed".into(),
                    source: Box::new(e),
                })?;
            Ok::<_, SmithError>(())
        })
        .await
        .map_err(|e| SmithError::PlatformWithCause {
            message: "Set text blocking task join failed".into(),
            source: Box::new(e),
        })??;

        Ok(json!({ "status": "text_set" }))
    }
}
