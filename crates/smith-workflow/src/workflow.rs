// crates/smith-workflow/src/workflow.rs
use std::collections::HashMap;

use smith_core::RetryPolicy;
use smith_graph::node::{EdgeKind, Node, NodeId};
use smith_graph::{FlowGraph, FlowGraphBuilder};

use crate::error::WorkflowError;
use crate::step::Step;

/// Workflow — a sequence of steps with conditional routing.
///
/// Created via the builder:
/// ```ignore
/// let wf = Workflow::new("name")
///     .step(Step::rpa("..."))
///     .step(Step::agent_decide("...").options(&["a", "b"]))
///     .on_choice("a", sub_wf_a)
///     .on_choice("b", sub_wf_b)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Workflow {
    pub(crate) name: String,
    pub(crate) steps: Vec<Step>,
    /// choices[step_index][option] = sub_workflow
    pub(crate) choices: HashMap<usize, HashMap<String, Workflow>>,
}

/// Builder for Workflow.
#[derive(Debug, Clone)]
pub struct WorkflowBuilder {
    name: String,
    steps: Vec<Step>,
    choices: HashMap<usize, HashMap<String, Workflow>>,
}

#[allow(clippy::new_ret_no_self)]
impl Workflow {
    /// Creates a new builder.
    pub fn new(name: impl Into<String>) -> WorkflowBuilder {
        WorkflowBuilder {
            name: name.into(),
            steps: vec![],
            choices: HashMap::new(),
        }
    }
}

impl WorkflowBuilder {
    /// Adds a step to the workflow.
    pub fn step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    /// Adds conditional routing for the last added Decide step.
    ///
    /// # Panics
    ///
    /// Panics if there is no last step (called before `.step()`).
    pub fn on_choice(mut self, option: &str, workflow: Workflow) -> Self {
        let step_idx = self
            .steps
            .len()
            .checked_sub(1)
            .expect("on_choice must follow a step");

        self.choices
            .entry(step_idx)
            .or_default()
            .insert(option.to_string(), workflow);
        self
    }

    /// Builds the Workflow with validation.
    ///
    /// # Errors
    ///
    /// Returns `WorkflowError::ValidationError` if:
    /// - The Decide step has an empty options list.
    pub fn build(self) -> Result<Workflow, WorkflowError> {
        // Validation: Decide steps must have non-empty options.
        for (idx, step) in self.steps.iter().enumerate() {
            if let crate::step::StepKind::Decide { options, .. } = &step.kind
                && options.is_empty()
            {
                return Err(WorkflowError::ValidationError(format!(
                    "Step {idx}: Decide must have at least one option",
                )));
            }
        }

        Ok(Workflow {
            name: self.name,
            steps: self.steps,
            choices: self.choices,
        })
    }
}

impl TryFrom<Workflow> for FlowGraph {
    type Error = String;

    fn try_from(wf: Workflow) -> Result<Self, Self::Error> {
        let mut builder = FlowGraphBuilder::new(&wf.name);
        let mut node_ids: Vec<NodeId> = Vec::new();

        // 1. Convert each Step → Node
        for step in &wf.steps {
            let node = step_to_node(step)?;
            let id = builder.add_node(node);
            node_ids.push(id);
        }

        // 2. Connect linearly: step[i] → step[i+1] (on_success)
        //    For Decide steps, choices come from wf.choices
        for (i, &id) in node_ids.iter().enumerate() {
            // If this step has conditional choices, use on_choice
            if let Some(choices) = wf.choices.get(&i) {
                for (label, sub_wf) in choices {
                    // Convert sub_workflow to FlowGraph and add as SubGraph node
                    let sub_graph: FlowGraph = sub_wf.clone().try_into()?;
                    let sub_id = builder.add_node(Node::SubGraph {
                        graph: Box::new(sub_graph),
                    });
                    builder.on_choice(id, label, sub_id);
                }
            } else if let Some(&next_id) = node_ids.get(i + 1) {
                // Linear progression
                builder.connect(id, EdgeKind::Success, next_id);
            }
        }

        builder.build()
    }
}

/// Converts StepKind to Node.
fn step_to_node(step: &Step) -> Result<Node, String> {
    match &step.kind {
        crate::step::StepKind::Rpa { name, args, retry } => Ok(Node::Rpa {
            tool: name,
            args: args.clone(),
            retry: RetryPolicy {
                max_retries: retry.max_retries,
                delay_ms: retry.delay_ms,
            },
        }),
        crate::step::StepKind::Agent {
            prompt,
            tools,
            max_steps,
        } => Ok(Node::Agent {
            prompt: prompt.clone(),
            tools: tools.clone(),
            max_turns: *max_steps,
        }),
        crate::step::StepKind::Think {
            prompt,
            output_schema,
        } => Ok(Node::Think {
            prompt: prompt.clone(),
            output_schema: output_schema.clone(),
        }),
        crate::step::StepKind::Decide { prompt, options } => Ok(Node::Router {
            prompt: prompt.clone(),
            options: options.iter().map(|o| (o.clone(), String::new())).collect(),
        }),
        crate::step::StepKind::Workflow(sub) => {
            let sub_graph: FlowGraph = sub.clone().try_into()?;
            Ok(Node::SubGraph {
                graph: Box::new(sub_graph),
            })
        }
    }
}
