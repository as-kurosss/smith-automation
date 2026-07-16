//! **SmithToolAdapter** — bridge that wraps any `smith_core::DynTool` as a
//! `smith_agent::agent::tool::Tool`.
//!
//! This adapter is the standard integration point between the deterministic
//! RPA layer (smith-core / smith-windows) and the agent layer (smith-agent).
//! It lives in smith-agent so that agent consumers can use any smith-core tool
//! without depending on smith-core directly.
//!
//! # Example
//!
//! ```rust
//! use smith_agent::agent::tool::{Tool, ToolCategory, ToolSet};
//! use smith_agent::tools::SmithToolAdapter;
//! use serde_json::json;
//!
//! // Wrap a smith_core::Tool via the adapter.
//! let adapter = SmithToolAdapter::from(DummyTool);
//!
//! // Use it as a regular smith_agent Tool.
//! let spec = adapter.spec();
//! assert_eq!(spec.name, "dummy");
//! assert_eq!(spec.category, ToolCategory::Generic);
//!
//! // Add to a ToolSet like any other agent tool.
//! let mut set = ToolSet::new();
//! set.add(adapter);
//!
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! let result = set.execute("dummy", json!({"msg": "hi"})).await.unwrap();
//! assert_eq!(result, json!({"echo": "hi"}));
//! # });
//!
//! /// A minimal smith_core tool used for illustration.
//! struct DummyTool;
//!
//! #[async_trait::async_trait]
//! impl smith_core::Tool for DummyTool {
//!     type Input = serde_json::Value;
//!     type Output = serde_json::Value;
//!     fn name(&self) -> &'static str { "dummy" }
//!     fn description(&self) -> &'static str { "A dummy tool" }
//!     fn schema(&self) -> serde_json::Value { json!({"type": "object", "properties": {"msg": {"type": "string"}}}) }
//!     async fn execute(
//!         &self,
//!         input: Self::Input,
//!         _ctx: &mut smith_core::ExecutionContext,
//!         _token: tokio_util::sync::CancellationToken,
//!     ) -> Result<Self::Output, smith_core::ToolError> {
//!         Ok(json!({"echo": input["msg"]}))
//!     }
//! }
//! ```

use async_trait::async_trait;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use smith_core::tool::DynTool;
use smith_core::{ExecutionContext, Ready, Unvalidated};

use crate::agent::tool::{Tool as AgentTool, ToolCategory, ToolError as AgentToolError, ToolSpec};

/// Wraps any `smith_core::DynTool` as a `smith_agent::agent::tool::Tool`.
///
/// On every [`call()`](AgentTool::call) it creates a fresh
/// [`ExecutionContext<Ready>`] and [`CancellationToken`] and delegates to the
/// underlying smith-core tool.
pub struct SmithToolAdapter {
    inner: Box<dyn DynTool>,
}

impl SmithToolAdapter {
    /// Wrap a smith-core tool already boxed as `DynTool`.
    pub fn new(inner: Box<dyn DynTool>) -> Self {
        Self { inner }
    }

    /// Convenience constructor for any `T: smith_core::Tool + 'static`.
    pub fn from<T: smith_core::Tool + 'static>(tool: T) -> Self {
        Self::new(Box::new(tool))
    }
}

#[async_trait]
impl AgentTool for SmithToolAdapter {
    fn spec(&self) -> ToolSpec {
        ToolSpec {
            name: self.inner.name().to_string(),
            description: self.inner.description().to_string(),
            parameters: self.inner.schema(),
            category: ToolCategory::Generic,
        }
    }

    async fn call(&self, args: Value) -> Result<Value, AgentToolError> {
        let mut ctx: ExecutionContext<Ready> = ExecutionContext::<Unvalidated>::new().validate();
        let token = CancellationToken::new();
        self.inner
            .execute(args, &mut ctx, token)
            .await
            .map_err(|e| AgentToolError::Execution {
                tool: self.inner.name().to_string(),
                message: e.to_string(),
            })
    }
}

/// A helper tool that delegates execution to an `Box<dyn DynTool>` stored
/// behind the adapter. Created via `SmithToolAdapter::new()`.
///
/// This struct exists so that agent tooling can introspect the inner smith
/// tool through the adapter if needed.
impl std::fmt::Debug for SmithToolAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmithToolAdapter")
            .field("inner_name", &self.inner.name())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::tool::ToolSet;
    use serde_json::json;
    use smith_core::tool::Tool as SmithTool;

    // -----------------------------------------------------------------------
    // A deterministic smith-core tool for testing
    // -----------------------------------------------------------------------

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct EchoInput {
        message: String,
    }

    #[derive(Debug, serde::Serialize)]
    struct EchoOutput {
        result: String,
    }

    struct EchoSmithTool;

    #[async_trait]
    impl SmithTool for EchoSmithTool {
        type Input = EchoInput;
        type Output = EchoOutput;

        fn name(&self) -> &'static str {
            "smith.echo"
        }

        fn description(&self) -> &'static str {
            "Echoes a message back to the caller"
        }

        fn schema(&self) -> Value {
            json!({
                "type": "object",
                "properties": {
                    "message": {
                        "type": "string",
                        "description": "The message to echo"
                    }
                },
                "required": ["message"]
            })
        }

        async fn execute(
            &self,
            input: EchoInput,
            _ctx: &mut ExecutionContext,
            _token: CancellationToken,
        ) -> Result<EchoOutput, smith_core::ToolError> {
            Ok(EchoOutput {
                result: input.message,
            })
        }
    }

    // -----------------------------------------------------------------------
    // Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_adapter_spec_is_correct() {
        let adapter = SmithToolAdapter::from(EchoSmithTool);
        let spec = adapter.spec();

        assert_eq!(spec.name, "smith.echo");
        assert_eq!(spec.description, "Echoes a message back to the caller");
        assert!(spec.parameters.is_object());
        assert_eq!(spec.category, ToolCategory::Generic);
    }

    #[tokio::test]
    async fn test_adapter_call_via_dyntool() {
        let adapter = SmithToolAdapter::new(Box::new(EchoSmithTool));

        let result = adapter
            .call(json!({"message": "hello from smith!"}))
            .await
            .unwrap();

        assert_eq!(result, json!({"result": "hello from smith!"}));
    }

    #[tokio::test]
    async fn test_adapter_in_agent_toolset() {
        let mut toolset = ToolSet::new();
        toolset.add(SmithToolAdapter::from(EchoSmithTool));

        let specs = toolset.specs();
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].name, "smith.echo");

        let result = toolset
            .execute("smith.echo", json!({"message": "via ToolSet"}))
            .await
            .unwrap();
        assert_eq!(result, json!({"result": "via ToolSet"}));
    }

    #[tokio::test]
    async fn test_adapter_error_propagation() {
        struct FailSmithTool;

        #[async_trait]
        impl SmithTool for FailSmithTool {
            type Input = EchoInput;
            type Output = EchoOutput;
            fn name(&self) -> &'static str {
                "smith.fail"
            }
            fn description(&self) -> &'static str {
                "Always fails"
            }
            fn schema(&self) -> Value {
                json!({})
            }
            async fn execute(
                &self,
                _input: EchoInput,
                _ctx: &mut ExecutionContext,
                _token: CancellationToken,
            ) -> Result<EchoOutput, smith_core::ToolError> {
                Err(smith_core::ToolError::platform_error(
                    "something broke",
                    std::io::Error::other("oh no"),
                    None,
                ))
            }
        }

        let adapter = SmithToolAdapter::from(FailSmithTool);
        let result = adapter.call(json!({"message": "x"})).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AgentToolError::Execution { .. }));
        assert!(err.to_string().contains("something broke"));
    }
}
