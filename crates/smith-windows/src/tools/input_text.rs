// crates/smith-windows/src/tools/input_text.rs
use async_trait::async_trait;
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult};
use tokio_util::sync::CancellationToken;

use crate::element::SafeUIElement;
use crate::selector::ElementSelector;

/// Инструмент для ввода текста в UI-элемент Windows.
///
/// Если указан `element_key` или `selector`, сначала фокусирует элемент,
/// затем вводит текст через UIA `send_keys`. Если элемент не указан,
/// просто отправляет нажатия клавиш в активное окно.
pub struct InputTextTool;

impl InputTextTool {
    /// Создаёт новый экземпляр `InputTextTool`.
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
        // 1. Валидация параметров
        let text = config
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SmithError::InvalidParams("Missing 'text'".into()))?
            .to_string();

        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        // 2. Пытаемся получить элемент из контекста или найти по селектору
        let maybe_element: Option<SafeUIElement> =
            if let Some(element_key) = config.get("element_key").and_then(|v| v.as_str()) {
                // Извлекаем из контекста
                let value = ctx.get(element_key).ok_or_else(|| {
                    SmithError::ContextError(format!("Key '{element_key}' not found in context"))
                })?;
                Some(value.try_as_custom::<SafeUIElement>()?.clone())
            } else if config.get("name").is_some()
                || config.get("automation_id").is_some()
                || config.get("control_type").is_some()
                || config.get("class_name").is_some()
            {
                // Строим селектор и ищем
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

                Some(safe_element)
            } else {
                None
            };

        // 3. Ввод текста в блокирующем потоке
        tokio::task::spawn_blocking(move || {
            if let Some(element) = maybe_element {
                // Фокусируем элемент, затем вводим текст
                element
                    .inner()
                    .set_focus()
                    .map_err(|e| SmithError::PlatformWithCause {
                        message: "Set focus failed".into(),
                        source: Box::new(e),
                    })?;
                std::thread::sleep(std::time::Duration::from_millis(100));
                element
                    .inner()
                    .send_keys(&text, 0)
                    .map_err(|e| SmithError::PlatformWithCause {
                        message: "send_keys failed".into(),
                        source: Box::new(e),
                    })?;
            } else {
                // Просто вводим текст в активное окно через корневой элемент
                let automation = uiautomation::core::UIAutomation::new()
                    .map_err(|e| SmithError::PlatformWithCause {
                        message: "UIAutomation init failed".into(),
                        source: Box::new(e),
                    })?;
                let root = automation
                    .get_root_element()
                    .map_err(|e| SmithError::PlatformWithCause {
                        message: "Get root element failed".into(),
                        source: Box::new(e),
                    })?;
                root.send_keys(&text, 0)
                    .map_err(|e| SmithError::PlatformWithCause {
                        message: "send_keys failed".into(),
                        source: Box::new(e),
                    })?;
            }
            Ok::<_, SmithError>(())
        })
        .await
        .map_err(|e| SmithError::PlatformWithCause {
            message: "Input text blocking task join failed".into(),
            source: Box::new(e),
        })??;

        Ok(json!({ "status": "input_sent" }))
    }
}
