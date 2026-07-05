// crates/smith-workflow/src/lib.rs
pub mod context;
pub mod error;
pub mod executor;
pub mod step;
pub mod workflow;

pub use context::WorkflowContext;
pub use error::{AgentResult, StepErrorContext, WorkflowError};
pub use executor::WorkflowExecutor;
pub use step::{RetryPolicy, Step, StepKind};
pub use workflow::Workflow;

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::context::WorkflowContext;
    pub use crate::error::WorkflowError;
    pub use crate::step::{RetryPolicy, Step, StepKind};
    pub use crate::workflow::Workflow;
    pub use serde_json::json;
}
