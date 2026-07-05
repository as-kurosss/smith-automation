// crates/smith-workflow/src/context.rs
use std::collections::HashMap;

use serde_json::Value;
use smith_core::ExecutionContext;

/// Workflow execution context.
///
/// Wraps `smith_core::ExecutionContext` and adds:
/// - Results of completed steps (step_results)
/// - Current step index
/// - Execution start time
pub struct WorkflowContext {
    /// Inner smith-core context (variables, scope).
    pub inner: ExecutionContext,
    /// Results of completed steps: step_index → result JSON.
    pub step_results: HashMap<usize, Value>,
    /// Current step index.
    pub current_step: usize,
    /// Workflow start timestamp (ms since epoch).
    pub started_at: u64,
    /// Number of RPA steps completed.
    pub rpa_count: usize,
    /// Number of Agent steps completed.
    pub agent_count: usize,
}

impl WorkflowContext {
    /// Creates a new context.
    pub fn new() -> Self {
        Self {
            inner: ExecutionContext::new(),
            step_results: HashMap::new(),
            current_step: 0,
            started_at: Self::now(),
            rpa_count: 0,
            agent_count: 0,
        }
    }

    /// Saves the step result.
    pub fn set_step_result(&mut self, index: usize, result: Value) {
        self.step_results.insert(index, result);
    }

    /// Gets the saved step result.
    #[must_use]
    pub fn get_step_result(&self, index: usize) -> Option<&Value> {
        self.step_results.get(&index)
    }

    /// Returns elapsed time in milliseconds.
    #[must_use]
    pub fn elapsed_ms(&self) -> u64 {
        Self::now() - self.started_at
    }

    fn now() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }
}

impl Default for WorkflowContext {
    fn default() -> Self {
        Self::new()
    }
}
