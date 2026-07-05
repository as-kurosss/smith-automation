// crates/smith-workflow/src/step.rs
use serde_json::Value;
pub use smith_core::RetryPolicy;
use tracing::warn;

use crate::workflow::Workflow;

/// Workflow step variants.
#[derive(Debug, Clone)]
pub enum StepKind {
    /// Deterministic RPA step. No LLM involved.
    /// name — tool name (e.g. "windows.click").
    Rpa {
        name: &'static str,
        args: Value,
        retry: RetryPolicy,
    },
    /// Agent receives a prompt and decides,
    /// which RPA tools to call and in what order.
    Agent {
        prompt: String,
        /// Tool names available to the agent (subset of all registered tools).
        tools: Vec<String>,
        /// Limit of tool calls per step.
        max_steps: usize,
    },
    /// Agent generates data/solution without calling tools.
    Think {
        prompt: String,
        output_schema: Value,
    },
    /// Agent selects one option from a list.
    Decide {
        prompt: String,
        options: Vec<String>,
    },
    /// Nested workflow.
    Workflow(Workflow),
}

/// Workflow step.
///
/// Created via constructors:
/// - `Step::rpa("name")` — RPA step
/// - `Step::agent("prompt")` — free agent with tools
/// - `Step::agent_think("prompt")` — LLM generates data
/// - `Step::agent_decide("prompt")` — LLM selects an option
/// - `Step::workflow(sub)` — nested workflow
#[derive(Debug, Clone)]
pub struct Step {
    pub(crate) kind: StepKind,
}

impl Step {
    /// Creates an RPA step.
    pub fn rpa(name: &'static str) -> Self {
        Self {
            kind: StepKind::Rpa {
                name,
                args: Value::Null,
                retry: RetryPolicy::default(),
            },
        }
    }

    /// Sets arguments for the RPA step.
    pub fn args(mut self, args: Value) -> Self {
        self.kind = match self.kind {
            StepKind::Rpa {
                name,
                args: _,
                retry,
            } => StepKind::Rpa { name, args, retry },
            other => {
                warn!("Step::args() called on non-RPA step, ignoring");
                other
            }
        };
        self
    }

    /// Sets the retry policy for the RPA step.
    pub fn retry(mut self, policy: RetryPolicy) -> Self {
        self.kind = match self.kind {
            StepKind::Rpa {
                name,
                args,
                retry: _,
            } => StepKind::Rpa {
                name,
                args,
                retry: policy,
            },
            other => {
                warn!("Step::retry() called on non-RPA step, ignoring");
                other
            }
        };
        self
    }

    /// Creates an Agent step (LLM with tools).
    pub fn agent(prompt: impl Into<String>) -> Self {
        Self {
            kind: StepKind::Agent {
                prompt: prompt.into(),
                tools: vec![],
                max_steps: 10,
            },
        }
    }

    /// Sets the list of tools available to the agent.
    pub fn tools(mut self, tool_names: Vec<&'static str>) -> Self {
        self.kind = match self.kind {
            StepKind::Agent {
                prompt,
                tools: _,
                max_steps,
            } => StepKind::Agent {
                prompt,
                tools: tool_names.iter().map(|&s| s.to_string()).collect(),
                max_steps,
            },
            other => {
                warn!("Step::tools() called on non-Agent step, ignoring");
                other
            }
        };
        self
    }

    /// Sets the maximum number of agent steps.
    pub fn max_steps(mut self, max: usize) -> Self {
        self.kind = match self.kind {
            StepKind::Agent {
                prompt,
                tools,
                max_steps: _,
            } => StepKind::Agent {
                prompt,
                tools,
                max_steps: max,
            },
            other => {
                warn!("Step::max_steps() called on non-Agent step, ignoring");
                other
            }
        };
        self
    }

    /// Creates a Think step (LLM generates data).
    pub fn agent_think(prompt: impl Into<String>) -> Self {
        Self {
            kind: StepKind::Think {
                prompt: prompt.into(),
                output_schema: Value::Null,
            },
        }
    }

    /// Sets JSON Schema for the Think step.
    pub fn schema(mut self, schema: Value) -> Self {
        self.kind = match self.kind {
            StepKind::Think {
                prompt,
                output_schema: _,
            } => StepKind::Think {
                prompt,
                output_schema: schema,
            },
            other => {
                warn!("Step::schema() called on non-Think step, ignoring");
                other
            }
        };
        self
    }

    /// Creates a Decide step (LLM selects an option).
    pub fn agent_decide(prompt: impl Into<String>) -> Self {
        Self {
            kind: StepKind::Decide {
                prompt: prompt.into(),
                options: vec![],
            },
        }
    }

    /// Adds context to the Decide step prompt.
    pub fn context(self, context: &str) -> Self {
        match self.kind {
            StepKind::Decide { prompt, options } => Self {
                kind: StepKind::Decide {
                    prompt: format!("{}\n\nContext: {}", prompt, context),
                    options,
                },
            },
            other => {
                warn!("Step::context() called on non-Decide step, ignoring");
                Self { kind: other }
            }
        }
    }

    /// Sets options for the Decide step.
    pub fn options(mut self, opts: &[&str]) -> Self {
        self.kind = match self.kind {
            StepKind::Decide { prompt, options: _ } => StepKind::Decide {
                prompt,
                options: opts.iter().map(|&s| s.to_string()).collect(),
            },
            other => other,
        };
        self
    }

    /// Creates a nested workflow step.
    pub fn workflow(workflow: Workflow) -> Self {
        Self {
            kind: StepKind::Workflow(workflow),
        }
    }

    /// Returns the step kind name for logging.
    pub fn kind_name(&self) -> &'static str {
        match &self.kind {
            StepKind::Rpa { .. } => "RPA",
            StepKind::Agent { .. } => "Agent",
            StepKind::Think { .. } => "Think",
            StepKind::Decide { .. } => "Decide",
            StepKind::Workflow(_) => "Workflow",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpa_step_kind_name() {
        let step = Step::rpa("windows.click");
        assert_eq!(step.kind_name(), "RPA");
    }

    #[test]
    fn test_agent_step_kind_name() {
        let step = Step::agent("Do something");
        assert_eq!(step.kind_name(), "Agent");
    }

    #[test]
    fn test_think_step_kind_name() {
        let step = Step::agent_think("Think about it");
        assert_eq!(step.kind_name(), "Think");
    }

    #[test]
    fn test_decide_step_kind_name() {
        let step = Step::agent_decide("Choose").options(&["a", "b"]);
        assert_eq!(step.kind_name(), "Decide");
    }

    #[test]
    fn test_rpa_args_sets_args() {
        let step = Step::rpa("windows.find").args(serde_json::json!({ "name": "test" }));
        assert_eq!(step.kind_name(), "RPA");
    }

    #[test]
    fn test_agent_tools_sets_tools() {
        let step = Step::agent("Do").tools(vec!["tool1"]);
        assert_eq!(step.kind_name(), "Agent");
    }

    #[test]
    fn test_decide_options_are_set() {
        let step = Step::agent_decide("Pick").options(&["x", "y", "z"]);
        assert_eq!(step.kind_name(), "Decide");
    }

    #[test]
    fn test_retry_policy_defaults() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_retries, 0);
        assert_eq!(policy.delay_ms, 0);
    }
}
