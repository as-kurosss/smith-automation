// crates/smith-windows/src/tools/mod.rs
#[cfg(windows)]
pub mod click;
#[cfg(windows)]
pub mod find;
#[cfg(windows)]
pub mod input_text;
#[cfg(windows)]
pub mod process;
#[cfg(windows)]
pub mod set_text;
#[cfg(windows)]
pub mod wait;

#[cfg(windows)]
pub use click::ClickTool;
#[cfg(windows)]
pub use find::FindTool;
#[cfg(windows)]
pub use input_text::InputTextTool;
#[cfg(windows)]
pub use process::ProcessTool;
#[cfg(windows)]
pub use set_text::SetTextTool;
#[cfg(windows)]
pub use wait::WaitTool;

#[cfg(windows)]
use serde_json::Value;
#[cfg(windows)]
use smith_core::{ExecutionContext, SmithError, SmithResult};

#[cfg(windows)]
pub(crate) use self::helpers::{
    apply_delay_after, apply_delay_before, resolve_element_from_config,
};

#[cfg(windows)]
mod helpers {
    use super::{ExecutionContext, SmithError, SmithResult, Value};
    use crate::element::SafeUIElement;
    use crate::selector::ElementSelector;

    /// Extracts `delay_before_ms` from the config and applies the delay if specified.
    pub(crate) async fn apply_delay_before(config: &Value) {
        if let Some(ms) = config.get("delay_before_ms").and_then(|v| v.as_u64())
            && ms > 0
        {
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
        }
    }

    /// Extracts `delay_after_ms` from the config and applies the delay if specified.
    pub(crate) async fn apply_delay_after(config: &Value) {
        if let Some(ms) = config.get("delay_after_ms").and_then(|v| v.as_u64())
            && ms > 0
        {
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
        }
    }

    /// Resolves a UI element from tool config, trying in order:
    /// 1. Look up `element_key` in the execution context
    /// 2. Build an `ElementSelector` from selector fields and find from desktop
    ///
    /// Returns `Ok(None)` if neither method has enough parameters.
    pub(crate) async fn resolve_element_from_config(
        config: &Value,
        ctx: &ExecutionContext,
    ) -> SmithResult<Option<SafeUIElement>> {
        // 1. Try to get element from context by element_key
        if let Some(element_key) = config.get("element_key").and_then(|v| v.as_str()) {
            let value = ctx.get(element_key).ok_or_else(|| {
                SmithError::ContextError(format!("Key '{element_key}' not found in context"))
            })?;
            return value
                .try_as_custom::<SafeUIElement>()
                .map(|e| Some(e.clone()));
        }

        // 2. Try to find element by selector fields
        let has_selector = config.get("name").is_some()
            || config.get("automation_id").is_some()
            || config.get("control_type").is_some()
            || config.get("class_name").is_some();

        if !has_selector {
            return Ok(None);
        }

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
            selector.find_from_desktop().map(SafeUIElement::new)
        })
        .await
        .map_err(|e| SmithError::PlatformError {
            message: "Find element blocking task failed".into(),
            source: Box::new(e),
        })??;

        Ok(Some(safe_element))
    }
}
