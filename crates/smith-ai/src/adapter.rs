// crates/smith-ai/src/adapter.rs
//! Adapter: `smith_core::Tool` → `rig::tool::ToolDyn`.
//!
//! Allows using any smith-core tools inside a Rig agent
//! via the dyn-safe `ToolDyn` trait.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use rig::completion::request::ToolDefinition;
use rig::tool::{ToolDyn, ToolError};
use smith_core::{ExecutionContext, Tool as SmithTool, ToolConfig};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

/// Adapter wrapping a `smith_core::Tool` into `rig::tool::ToolDyn`.
pub struct ToolAdapter {
    inner: Arc<dyn SmithTool>,
    ctx: Arc<Mutex<ExecutionContext>>,
    token: CancellationToken,
}

impl ToolAdapter {
    /// Creates a new adapter.
    pub fn new(
        tool: impl SmithTool + 'static,
        ctx: Arc<Mutex<ExecutionContext>>,
        token: CancellationToken,
    ) -> Self {
        Self {
            inner: Arc::new(tool),
            ctx,
            token,
        }
    }

    /// Creates an adapter from Arc<dyn SmithTool>.
    pub fn from_arc(
        tool: Arc<dyn SmithTool>,
        ctx: Arc<Mutex<ExecutionContext>>,
        token: CancellationToken,
    ) -> Self {
        Self {
            inner: tool,
            ctx,
            token,
        }
    }
}

impl ToolDyn for ToolAdapter {
    fn name(&self) -> String {
        self.inner.name().to_string()
    }

    fn definition<'a>(
        &'a self,
        _prompt: String,
    ) -> Pin<Box<dyn Future<Output = ToolDefinition> + Send + 'a>> {
        Box::pin(async move {
            ToolDefinition {
                name: self.inner.name().to_string(),
                description: self.inner.description().to_string(),
                parameters: self.inner.schema(),
            }
        })
    }

    fn call<'a>(
        &'a self,
        args: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, ToolError>> + Send + 'a>> {
        Box::pin(async move {
            let config: ToolConfig = match serde_json::from_str(&args) {
                Ok(c) => c,
                Err(e) => return Err(ToolError::JsonError(e)),
            };

            let mut ctx = self.ctx.lock().await;

            match self
                .inner
                .execute(config, &mut ctx, self.token.clone())
                .await
            {
                Ok(result) => match serde_json::to_string(&result) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(ToolError::JsonError(e)),
                },
                Err(e) => Err(ToolError::ToolCallError(e.to_string().into())),
            }
        })
    }
}
