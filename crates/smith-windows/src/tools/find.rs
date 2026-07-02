// crates/smith-windows/src/tools/find.rs
use async_trait::async_trait;
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult};
use tokio_util::sync::CancellationToken;

use crate::element::SafeUIElement;
use crate::selector::ElementSelector;

/// Инструмент для поиска UI-элемента на рабочем столе Windows.
///
/// Принимает параметры селектора (name, automation_id, control_type, class_name, pid)
/// и сохраняет найденный элемент в контексте под указанным ключом.
pub struct FindTool;

impl FindTool {
    /// Создаёт новый экземпляр `FindTool`.
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
        // 1. Валидация параметров (Канон 10.1)
        let output_key = config
            .get("output_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SmithError::InvalidParams("Missing 'output_key'".into()))?;

        // 2. Проверка отмены (Канон 5.4)
        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        // 3. Строим селектор из конфига
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
        if let Some(pid) = config.get("pid").and_then(|v| v.as_u64()) {
            selector = selector.pid(pid as u32);
        }

        // 4. Поиск в блокирующем потоке (COM-вызовы).
        //    SafeUIElement создаётся внутри spawn_blocking, т.к. UIElement не является Send.
        let safe_element = tokio::task::spawn_blocking(move || {
            selector
                .find_from_desktop()
                .map(SafeUIElement::new)
        })
        .await
        .map_err(|e| SmithError::PlatformError {
            message: "Find element blocking task failed".into(),
            source: Box::new(e),
        })??;

        // 5. Сохраняем результат в контекст
        ctx.set(
            output_key.to_string(),
            smith_core::ContextValue::Custom(std::sync::Arc::new(safe_element)),
        );

        Ok(json!({ "status": "found" }))
    }
}
