// crates/smith-graph/src/lib.rs
//! FlowGraph — graph-based execution engine for hybrid Agent + RPA.
//!
//! # Key types
//! - [`Node`] — node types (Rpa, Agent, Router, Think, SubGraph, Loop, Approval)
//! - [`Edges`] — transitions on success/failure/choice
//! - [`FlowGraph`] — directed graph with an entry node
//! - [`GraphExecutor`] — executes the graph, dispatches nodes to ToolRegistry or AiHandler

pub mod executor;
pub mod graph;
pub mod node;

pub use executor::GraphExecutor;
pub use graph::{FlowGraph, FlowGraphBuilder};
pub use node::{EdgeKind, Edges, Node, NodeId, NodeIO};
pub use smith_core::RetryPolicy;
