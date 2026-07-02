// crates/smith-windows/src/tools/set_text.rs
use async_trait::async_trait;
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult};
use tokio_util::sync::CancellationToken;

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
            "anyOf": [
                { "required": ["element_key"] },
                { "required": ["name"] },
                { "required": ["automation_id"] },
                { "required": ["control_type"] },
                { "required": ["class_name"] }
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
        let element = crate::tools::resolve_element_from_config(&config, ctx)
            .await?
            .ok_or_else(|| SmithError::InvalidParams("Missing 'element_key' or selector fields".into()))?;

        // 3. Устанавливаем текст через ValuePattern в блокирующем потоке
        tokio::task::spawn_blocking(move || {
            let pattern = element
                .inner()
                .get_pattern::<uiautomation::patterns::UIValuePattern>()
                .map_err(|e| SmithError::PlatformError {
                    message: "Get ValuePattern failed".into(),
                    source: Box::new(e),
                })?;
            pattern
                .set_value(&text)
                .map_err(|e| SmithError::PlatformError {
                    message: "Set value failed".into(),
                    source: Box::new(e),
                })?;
            Ok::<_, SmithError>(())
        })
        .await
        .map_err(|e| SmithError::PlatformError {
            message: "Set text blocking task join failed".into(),
            source: Box::new(e),
        })??;

        Ok(json!({ "status": "text_set" }))
    }
}
