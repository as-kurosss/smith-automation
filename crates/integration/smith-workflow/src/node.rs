// crates/smith-graph/src/node.rs
//! FlowGraph node and transition types.

use std::collections::HashMap;
use std::time::Duration;

use serde_json::Value;

// Forward-declare FlowGraph for SubGraph.
pub(crate) use crate::graph::FlowGraph;
pub use smith_core::RetryPolicy;

/// Node identifier in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

/// Node input/output metadata.
///
/// Allows checking type compatibility between nodes at `build()` time.
#[derive(Debug, Clone, Default)]
pub struct NodeIO {
    /// JSON Schema of expected input (if the node consumes data).
    pub expected_input: Option<Value>,
    /// JSON Schema of the output (what the node returns).
    pub output_schema: Option<Value>,
}

/// Graph node type.
#[derive(Debug, Clone)]
pub enum Node {
    /// Deterministic RPA call (no LLM).
    Rpa {
        tool: &'static str,
        args: Value,
        retry: RetryPolicy,
    },
    /// Nested subgraph (any nodes).
    SubGraph { graph: Box<FlowGraph> },
    /// LLM agent with ReAct loop, limited tool set.
    Ai {
        prompt: String,
        tools: Vec<String>,
        max_turns: usize,
    },
    /// LLM selects an option from an enum list, then routes via on_choice.
    Router {
        prompt: String,
        /// (label, description) — LLM chooses the label
        options: Vec<(String, String)>,
    },
    /// LLM generates data without tools (JSON per schema).
    Think {
        prompt: String,
        output_schema: Value,
    },
    /// Human-in-the-loop: pause until confirmed.
    Approval {
        message: String,
        timeout: Option<Duration>,
    },
    /// Loop: execute subgraph N times.
    Loop {
        body: Box<FlowGraph>,
        max_iterations: u32,
        output_key: String,
    },
}

impl Node {
    /// Node kind name for logging.
    pub fn kind_name(&self) -> &'static str {
        match self {
            Node::Rpa { .. } => "Rpa",
            Node::SubGraph { .. } => "SubGraph",
            Node::Ai { .. } => "Ai",
            Node::Router { .. } => "Router",
            Node::Think { .. } => "Think",
            Node::Approval { .. } => "Approval",
            Node::Loop { .. } => "Loop",
        }
    }
}

/// Type of transition between nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Success,
    Failure,
}

/// Transitions from a node.
#[derive(Debug, Clone)]
pub struct Edges {
    /// Node on success.
    pub on_success: Option<NodeId>,
    /// Node on failure.
    pub on_failure: Option<NodeId>,
    /// Choice routing (for Router/Decide).
    pub on_choice: HashMap<String, NodeId>,
}

impl Edges {
    /// Creates empty edges (terminal node).
    pub fn none() -> Self {
        Self {
            on_success: None,
            on_failure: None,
            on_choice: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smith_core::RetryPolicy;

    #[test]
    fn test_node_kind_name() {
        assert_eq!(
            Node::Rpa {
                tool: "t",
                args: Value::Null,
                retry: RetryPolicy::default()
            }
            .kind_name(),
            "Rpa"
        );
        assert_eq!(
            Node::Ai {
                prompt: "".into(),
                tools: vec![],
                max_turns: 3
            }
            .kind_name(),
            "Ai"
        );
        assert_eq!(
            Node::Router {
                prompt: "".into(),
                options: vec![]
            }
            .kind_name(),
            "Router"
        );
        assert_eq!(
            Node::Think {
                prompt: "".into(),
                output_schema: Value::Null
            }
            .kind_name(),
            "Think"
        );
        assert_eq!(
            Node::Approval {
                message: "".into(),
                timeout: None
            }
            .kind_name(),
            "Approval"
        );
    }

    #[test]
    fn test_edges_none() {
        let e = Edges::none();
        assert!(e.on_success.is_none());
        assert!(e.on_failure.is_none());
        assert!(e.on_choice.is_empty());
    }
}
