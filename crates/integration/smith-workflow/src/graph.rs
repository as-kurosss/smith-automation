// crates/smith-graph/src/graph.rs
//! FlowGraph — directed graph of typed nodes.

use std::collections::HashMap;

use tracing::warn;

use crate::node::{Edges, Node, NodeIO, NodeId};

/// Directed execution graph.
///
/// Consists of nodes (see [`Node`]) and edges ([`Edges`]).
/// Each node has an ID that edges reference.
/// The graph is validated at `build()`.
#[derive(Debug, Clone)]
pub struct FlowGraph {
    /// Graph name (for logging).
    pub name: String,
    /// Entry point.
    pub entry: NodeId,
    /// All graph nodes.
    pub nodes: HashMap<NodeId, Node>,
    /// Edges: NodeId → Edges.
    pub edges: HashMap<NodeId, Edges>,
    /// Typed input/output contracts.
    pub node_io: HashMap<NodeId, NodeIO>,
}

impl FlowGraph {
    /// Creates a builder for the graph.
    pub fn builder(name: impl Into<String>) -> FlowGraphBuilder {
        FlowGraphBuilder::new(name)
    }

    /// Creates a simple linear graph with a single node (convenient for From<Workflow>).
    pub fn single(name: impl Into<String>, node: Node) -> Self {
        let entry = NodeId(0);
        let mut nodes = HashMap::new();
        let mut edges = HashMap::new();
        nodes.insert(entry, node);
        edges.insert(entry, Edges::none());
        Self {
            name: name.into(),
            entry,
            nodes,
            edges,
            node_io: HashMap::new(),
        }
    }
}

/// Builder for FlowGraph.
///
/// ```ignore
/// let mut g = FlowGraph::builder("demo");
/// let find = g.add_node(Node::rpa("windows.find", json!({...})));
/// let click = g.add_node(Node::rpa("windows.click", json!({...})));
/// g.connect(find, EdgeKind::Success, click);
/// g.set_entry(find);
/// let graph = g.build().unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct FlowGraphBuilder {
    name: String,
    nodes: HashMap<NodeId, Node>,
    edges: HashMap<NodeId, Edges>,
    node_io: HashMap<NodeId, NodeIO>,
    next_id: usize,
}

impl FlowGraphBuilder {
    /// Creates a new builder.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nodes: HashMap::new(),
            edges: HashMap::new(),
            node_io: HashMap::new(),
            next_id: 0,
        }
    }

    /// Adds a node and returns its ID.
    pub fn add_node(&mut self, node: Node) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        self.nodes.insert(id, node);
        self.edges.insert(id, Edges::none());
        id
    }

    /// Adds a node with the specified ID (for conversion from Workflow).
    pub fn add_node_with_id(&mut self, id: NodeId, node: Node) {
        self.nodes.insert(id, node);
        self.edges.entry(id).or_insert(Edges::none());
    }

    /// Connects two nodes with a success or failure transition.
    pub fn connect(&mut self, from: NodeId, kind: crate::node::EdgeKind, to: NodeId) {
        if let Some(edges) = self.edges.get_mut(&from) {
            match kind {
                crate::node::EdgeKind::Success => edges.on_success = Some(to),
                crate::node::EdgeKind::Failure => edges.on_failure = Some(to),
            }
        }
    }

    /// Adds a conditional transition for a Router/Decide node.
    pub fn on_choice(&mut self, from: NodeId, label: impl Into<String>, to: NodeId) {
        if let Some(edges) = self.edges.get_mut(&from) {
            edges.on_choice.insert(label.into(), to);
        }
    }

    /// Sets the graph's entry point.
    pub fn set_entry(&mut self, _id: NodeId) {
        // Entry auto-detected on build() as the node with no incoming edges.
    }

    /// Sets IO metadata for a node.
    pub fn with_io(&mut self, id: NodeId, io: NodeIO) {
        self.node_io.insert(id, io);
    }

    /// Builds the graph with validation.
    ///
    /// # Errors
    ///
    /// Returns a string describing the error if validation fails.
    pub fn build(self) -> Result<FlowGraph, String> {
        // Find entry — first added node, if not overridden
        let entry = if self.nodes.is_empty() {
            return Err("Graph has no nodes".into());
        } else if self.nodes.len() == 1 {
            *self.nodes.keys().next().unwrap()
        } else {
            // Entry — node with no incoming edges (success/failure/choice)
            let has_incoming = self.find_incoming();
            let candidates: Vec<NodeId> = self
                .nodes
                .keys()
                .filter(|id| !has_incoming.contains(id))
                .copied()
                .collect();
            if candidates.len() != 1 {
                return Err(format!(
                    "Graph must have exactly one entry node (no incoming edges). Found {} candidates: {:?}",
                    candidates.len(),
                    candidates,
                ));
            }
            candidates[0]
        };

        // Validation: all target nodes in edges exist
        for (from_id, edges) in &self.edges {
            if let Some(to) = edges.on_success
                && !self.nodes.contains_key(&to)
            {
                return Err(format!(
                    "Edge from node {:?} points to non-existent node {:?}",
                    from_id, to
                ));
            }
            if let Some(to) = edges.on_failure
                && !self.nodes.contains_key(&to)
            {
                return Err(format!(
                    "Failure edge from node {:?} points to non-existent node {:?}",
                    from_id, to
                ));
            }
            for (label, to) in &edges.on_choice {
                if !self.nodes.contains_key(to) {
                    return Err(format!(
                        "Choice edge '{label}' from node {:?} points to non-existent node {:?}",
                        from_id, to
                    ));
                }
            }
        }

        // Validation: Router nodes must have on_choice, not on_success
        for (id, node) in &self.nodes {
            let edges = &self.edges[id];
            match node {
                Node::Router { options, .. } => {
                    if edges.on_success.is_some() {
                        return Err(format!(
                            "Router node {:?} has on_success edge, but Router must use on_choice",
                            id
                        ));
                    }
                    // All options must have a matching choice edge
                    for (label, _) in options {
                        if !edges.on_choice.contains_key(label) {
                            warn!(
                                "Router node {:?} option '{label}' has no matching on_choice edge",
                                id
                            );
                        }
                    }
                }
                _ => {
                    // Non-Router may have on_choice only if erroneous
                    // (not prohibited, but a warning is emitted)
                    if !edges.on_choice.is_empty() {
                        warn!(
                            "Non-Router node {:?} ({}) has on_choice edges, expected on_success",
                            id,
                            node.kind_name()
                        );
                    }
                }
            }
        }

        Ok(FlowGraph {
            name: self.name,
            entry,
            nodes: self.nodes,
            edges: self.edges,
            node_io: self.node_io,
        })
    }

    /// Finds all nodes with incoming edges (success, failure, choice).
    fn find_incoming(&self) -> Vec<NodeId> {
        let mut incoming = Vec::new();
        for edges in self.edges.values() {
            if let Some(to) = edges.on_success {
                incoming.push(to);
            }
            if let Some(to) = edges.on_failure {
                incoming.push(to);
            }
            for to in edges.on_choice.values() {
                incoming.push(*to);
            }
        }
        incoming
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{EdgeKind, RetryPolicy};
    use serde_json::Value;

    #[test]
    fn test_build_empty_fails() {
        let err = FlowGraph::builder("empty").build().unwrap_err();
        assert!(err.contains("no nodes"));
    }

    #[test]
    fn test_build_single_node() {
        let mut b = FlowGraph::builder("single");
        let n = b.add_node(Node::Rpa {
            tool: "test",
            args: Value::Null,
            retry: RetryPolicy::default(),
        });
        b.set_entry(n);
        let g = b.build().unwrap();
        assert_eq!(g.nodes.len(), 1);
        assert_eq!(g.entry, n);
    }

    #[test]
    fn test_build_linear_two_nodes() {
        let mut b = FlowGraph::builder("linear");
        let n1 = b.add_node(Node::Rpa {
            tool: "a",
            args: Value::Null,
            retry: RetryPolicy::default(),
        });
        let n2 = b.add_node(Node::Rpa {
            tool: "b",
            args: Value::Null,
            retry: RetryPolicy::default(),
        });
        b.connect(n1, EdgeKind::Success, n2);
        let g = b.build().unwrap();
        assert_eq!(g.nodes.len(), 2);
        assert_eq!(g.entry, n1);
        assert_eq!(g.edges[&n1].on_success, Some(n2));
    }

    #[test]
    fn test_build_router_requires_choice() {
        let mut b = FlowGraph::builder("router_test");
        let router = b.add_node(Node::Router {
            prompt: "choose".into(),
            options: vec![("a".into(), "Option A".into())],
        });
        let exit = b.add_node(Node::Rpa {
            tool: "exit",
            args: Value::Null,
            retry: RetryPolicy::default(),
        });
        // on_success is forbidden for Router — but we don't set it, just check that build passed
        b.on_choice(router, "a", exit);
        let g = b.build().unwrap();
        assert_eq!(g.entry, router);
        assert_eq!(g.edges[&router].on_choice.get("a"), Some(&exit));
    }

    #[test]
    fn test_build_router_with_success_is_error() {
        let mut b = FlowGraph::builder("bad_router");
        let router = b.add_node(Node::Router {
            prompt: "choose".into(),
            options: vec![],
        });
        let exit = b.add_node(Node::Rpa {
            tool: "exit",
            args: Value::Null,
            retry: RetryPolicy::default(),
        });
        b.connect(router, EdgeKind::Success, exit);
        let err = b.build().unwrap_err();
        assert!(err.contains("Router") && err.contains("on_success"));
    }
}
