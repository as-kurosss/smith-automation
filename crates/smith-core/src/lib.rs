// crates/smith-core/src/lib.rs
pub mod ai;
pub mod context;
pub mod error;
pub mod registry;
pub mod retry;
pub mod tool;

// Flat API
pub use ai::AiHandler;
pub use context::{ContextValue, ExecutionContext};
pub use error::{SmithError, SmithResult};
pub use registry::ToolRegistry;
pub use retry::RetryPolicy;
pub use tool::{Tool, ToolConfig, ToolResult};
