// crates/smith-windows/src/tools/click.rs
use async_trait::async_trait;
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult};
use tokio_util::sync::CancellationToken;

use crate::element::SafeUIElement;

/// Инструмент для выполнения клика по UI-элементу Windows.
pub struct ClickTool;

impl ClickTool {
    /// Создаёт новый экземпляр `ClickTool`.
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
        // 1. Валидация параметров (Канон 10.1)
        let element_key = config
            .get("element_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SmithError::InvalidParams("Missing 'element_key'".into()))?;

        // 2. Проверка отмены перед тяжелой операцией (Канон 5.4)
        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        // 3. Извлекаем элемент из контекста
        let value = ctx.get(element_key).ok_or_else(|| {
            SmithError::ContextError(format!("Key '{element_key}' not found in context"))
        })?;

        let wrapper = value.try_as_custom::<SafeUIElement>()?;
        let element_clone = wrapper.clone(); // Клонируем Arc, а не сам элемент

        // В spawn_blocking используем inner():
        tokio::task::spawn_blocking(move || {
            element_clone
                .inner()
                .click()
                .map_err(|e| SmithError::PlatformWithCause {
                    message: "Click failed".into(),
                    source: Box::new(e),
                })
        })
        .await
        .map_err(|e| SmithError::PlatformWithCause {
            message: "Blocking task join failed".into(),
            source: Box::new(e),
        })??;

        Ok(Value::Null)
    }
}
