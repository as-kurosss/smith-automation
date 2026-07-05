// crates/smith-workflow/src/error.rs
use thiserror::Error;

/// Workflow execution errors.
#[derive(Error, Debug)]
pub enum WorkflowError {
    /// Workflow validation error (at build() stage).
    #[error("Workflow validation error: {0}")]
    ValidationError(String),

    /// RPA tool not found in the registry.
    #[error("Tool '{0}' not found in registry")]
    ToolNotFound(String),

    /// RPA step execution error.
    #[error("Step {} failed: {source}", step_idx)]
    StepError {
        /// Step index in the workflow.
        step_idx: usize,
        /// Error cause.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// AI agent execution error.
    #[error("Agent error: {0}")]
    AgentError(String),

    /// Agent not configured (attempting to execute Agent/Think/Decide without AI).
    #[error("Agent not configured but AI step was requested")]
    AgentNotConfigured,

    /// Workflow cancelled.
    #[error("Workflow cancelled")]
    Cancelled,

    /// Provider error (OpenAI/Anthropic etc.).
    #[error("Provider error: {0}")]
    ProviderError(String),

    /// Serialization/deserialization error.
    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    /// Other errors.
    #[error("{0}")]
    Other(String),
}

/// RPA step error context: tool name + arguments + original error.
#[derive(Debug)]
pub struct StepErrorContext {
    /// Tool name (e.g. "windows.click").
    pub tool: String,
    /// Call arguments.
    pub args: serde_json::Value,
    /// Original execution error.
    pub inner: smith_core::SmithError,
}

impl std::fmt::Display for StepErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tool '{}' with args {} failed: {}",
            self.tool, self.args, self.inner
        )
    }
}

impl std::error::Error for StepErrorContext {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}

/// Workflow execution result.
#[derive(Debug, Clone)]
pub struct AgentResult {
    /// Whether the workflow completed successfully.
    pub success: bool,
    /// Workflow name.
    pub workflow_name: String,
    /// Number of completed steps.
    pub steps_completed: usize,
    /// Output data (last result or final JSON).
    pub output: serde_json::Value,
    /// Results of all steps (step index → JSON).
    pub step_results: std::collections::HashMap<usize, serde_json::Value>,
    /// Execution time in milliseconds.
    pub execution_time_ms: u64,
}
