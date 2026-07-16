// crates/smith-graph/src/executor.rs
//! GraphExecutor — executes FlowGraph, dispatches nodes to ToolRegistry or AiHandler.

use serde_json::Value;
use smith_core::{
    AiHandler, ExecutionContext, Ready, SmithError, SmithResult, ToolRegistry, Unvalidated,
};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::graph::FlowGraph;
use crate::node::{Node, NodeId, RetryPolicy};

/// FlowGraph executor.
pub struct GraphExecutor<'a> {
    registry: &'a ToolRegistry,
    ai_handler: Option<&'a dyn AiHandler>,
}

impl<'a> GraphExecutor<'a> {
    /// Creates a new executor.
    pub fn new(registry: &'a ToolRegistry, ai_handler: Option<&'a dyn AiHandler>) -> Self {
        Self {
            registry,
            ai_handler,
        }
    }

    /// Executes the entire graph.
    ///
    /// Returns the result of the last executed node (or an error).
    pub async fn execute(
        &self,
        graph: &FlowGraph,
        ctx: &mut ExecutionContext<Ready>,
        token: CancellationToken,
    ) -> SmithResult<Value> {
        let mut current = Some(graph.entry);
        let mut last_result: SmithResult<Value> = Ok(Value::Null);

        while let Some(node_id) = current {
            if token.is_cancelled() {
                return Err(SmithError::Cancelled);
            }

            let node = &graph.nodes[&node_id];
            info!("Executing node {:?}: {}", node_id, node.kind_name());

            let result = self.execute_node(node, ctx, &token).await;

            // Save the result in the context
            let result_key = format!("node_{node_id:?}");
            match &result {
                Ok(val) => {
                    ctx.set(
                        &result_key,
                        smith_core::ContextValue::String(
                            serde_json::to_string(val).unwrap_or_default(),
                        ),
                    );
                }
                Err(e) => {
                    warn!("Node {:?} ({}) failed: {e}", node_id, node.kind_name());
                    ctx.set(
                        &result_key,
                        smith_core::ContextValue::String(format!("ERROR: {e}")),
                    );
                }
            }

            // Determine the next node based on the result
            current = self.resolve_next(node_id, &result, graph);
            // Save the result as the last one before the transition
            last_result = result;
        }

        last_result
    }

    /// Determines the next node based on execution result.
    fn resolve_next(
        &self,
        node_id: NodeId,
        result: &SmithResult<Value>,
        graph: &FlowGraph,
    ) -> Option<NodeId> {
        let edges = &graph.edges[&node_id];
        match result {
            Ok(val) => {
                // For Router — check the choice
                if let Node::Router { .. } = &graph.nodes[&node_id] {
                    if let Some(choice_str) = val.as_str() {
                        if let Some(next) = edges.on_choice.get(choice_str) {
                            return Some(*next);
                        }
                        // Fallback: if choice not found — go to the first available
                        warn!(
                            "Router node {:?}: choice '{choice_str}' not found in edges ({:?}), taking first available",
                            node_id,
                            edges.on_choice.keys()
                        );
                        return edges.on_choice.values().next().copied();
                    }
                    // If Router returned a non-string — take the first choice
                    return edges.on_choice.values().next().copied();
                }
                edges.on_success
            }
            Err(_) => edges.on_failure.or(None),
        }
    }

    /// Executes a single node.
    async fn execute_node(
        &self,
        node: &Node,
        ctx: &mut ExecutionContext<Ready>,
        token: &CancellationToken,
    ) -> SmithResult<Value> {
        match node {
            Node::Rpa { tool, args, retry } => {
                self.execute_rpa(tool, args, retry, ctx, token).await
            }
            Node::SubGraph { graph } => {
                let mut sub_ctx: ExecutionContext<Ready> =
                    ExecutionContext::<Unvalidated>::new().validate();
                let result = Box::pin(self.execute(graph, &mut sub_ctx, token.clone())).await?;
                Ok(result)
            }
            Node::Ai {
                prompt,
                tools,
                max_turns,
            } => {
                let handler = self
                    .ai_handler
                    .ok_or_else(|| SmithError::InvalidParams("AI handler not configured".into()))?;
                handler
                    .agent_run(prompt, tools, *max_turns, ctx, token)
                    .await
            }
            Node::Router { prompt, options } => {
                let handler = self.ai_handler.ok_or_else(|| {
                    SmithError::InvalidParams("Router: AI handler not configured".into())
                })?;
                let labels: Vec<String> = options.iter().map(|(l, _)| l.clone()).collect();
                let decision = handler.decide(prompt, &labels, ctx, token).await?;
                Ok(Value::String(decision))
            }
            Node::Think {
                prompt,
                output_schema,
            } => {
                let handler = self.ai_handler.ok_or_else(|| {
                    SmithError::InvalidParams("Think: Agent not configured".into())
                })?;
                handler.think(prompt, output_schema, ctx, token).await
            }
            Node::Approval {
                message,
                timeout: _,
            } => {
                // For now just log and continue (HITL will come later)
                info!("Approval requested: {message}");
                // TODO: real HITL — wait for confirmation from an external channel
                Ok(serde_json::json!({ "approved": true }))
            }
            Node::Loop {
                body,
                max_iterations,
                output_key,
            } => {
                let mut last_result = Value::Null;
                for i in 0..*max_iterations {
                    if token.is_cancelled() {
                        return Err(SmithError::Cancelled);
                    }
                    info!("Loop iteration {}/{}", i + 1, max_iterations);
                    let mut sub_ctx: ExecutionContext<Ready> =
                        ExecutionContext::<Unvalidated>::new().validate();
                    last_result = Box::pin(self.execute(body, &mut sub_ctx, token.clone())).await?;
                }
                ctx.set(
                    output_key,
                    smith_core::ContextValue::String(
                        serde_json::to_string(&last_result).unwrap_or_default(),
                    ),
                );
                Ok(last_result)
            }
        }
    }

    /// Executes an RPA step via ToolRegistry with retry.
    ///
    /// # Errors
    ///
    /// Returns `SmithError` converted from `ToolError` for compatibility.
    async fn execute_rpa(
        &self,
        tool: &str,
        args: &Value,
        retry: &RetryPolicy,
        ctx: &mut ExecutionContext<Ready>,
        token: &CancellationToken,
    ) -> SmithResult<Value> {
        let max_retries = retry.max_retries;

        for attempt in 0..=max_retries {
            if token.is_cancelled() {
                return Err(SmithError::Cancelled);
            }

            match self
                .registry
                .execute(tool, args.clone(), ctx, token.clone())
                .await
            {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == max_retries {
                        return Err(SmithError::from(e));
                    }
                    warn!(
                        "RPA tool '{tool}' failed (attempt {}/{max_retries}): {e}",
                        attempt + 1,
                    );
                    let delay = std::time::Duration::from_millis(retry.delay_ms);
                    tokio::time::sleep(delay).await;
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::EdgeKind;
    use async_trait::async_trait;
    use serde::Deserialize;
    use serde::Serialize;
    use smith_core::{AiHandler, ContextValue, Tool, ToolError};

    #[derive(Debug, Serialize, Deserialize)]
    struct MockInput {}

    #[derive(Debug, Serialize)]
    struct MockOutput {
        status: &'static str,
    }

    /// Mock tool that returns a fixed result.
    struct MockTool;

    #[async_trait]
    impl Tool for MockTool {
        type Input = MockInput;
        type Output = MockOutput;

        fn name(&self) -> &'static str {
            "mock.tool"
        }
        fn description(&self) -> &'static str {
            "mock"
        }
        fn schema(&self) -> Value {
            serde_json::json!({})
        }
        async fn execute(
            &self,
            _input: MockInput,
            ctx: &mut ExecutionContext,
            _token: CancellationToken,
        ) -> Result<MockOutput, ToolError> {
            ctx.set("executed", ContextValue::Boolean(true));
            Ok(MockOutput { status: "ok" })
        }
    }

    /// Mock AiHandler.
    struct MockAi;

    #[async_trait]
    impl AiHandler for MockAi {
        async fn agent_run(
            &self,
            _prompt: &str,
            _tools: &[String],
            _max_steps: usize,
            ctx: &mut ExecutionContext,
            _token: &CancellationToken,
        ) -> SmithResult<Value> {
            ctx.set("ai_ran", ContextValue::Boolean(true));
            Ok(serde_json::json!({ "result": "ai_ok" }))
        }

        async fn think(
            &self,
            _prompt: &str,
            _schema: &Value,
            ctx: &mut ExecutionContext,
            _token: &CancellationToken,
        ) -> SmithResult<Value> {
            ctx.set("think_ran", ContextValue::Boolean(true));
            Ok(serde_json::json!({ "analysis": "done" }))
        }

        async fn decide(
            &self,
            _prompt: &str,
            options: &[String],
            ctx: &mut ExecutionContext,
            _token: &CancellationToken,
        ) -> SmithResult<String> {
            ctx.set("decide_ran", ContextValue::Boolean(true));
            Ok(options.first().cloned().unwrap_or_default())
        }
    }

    fn make_registry() -> ToolRegistry {
        let mut reg = ToolRegistry::new();
        reg.register(MockTool);
        reg
    }

    #[tokio::test]
    async fn test_execute_single_rpa() {
        let registry = make_registry();
        let executor = GraphExecutor::new(&registry, None);
        let mut ctx: ExecutionContext<Ready> = ExecutionContext::<Unvalidated>::new().validate();
        let token = CancellationToken::new();

        let graph = FlowGraph::single(
            "test",
            Node::Rpa {
                tool: "mock.tool",
                args: serde_json::json!({}),
                retry: RetryPolicy::default(),
            },
        );

        let result = executor.execute(&graph, &mut ctx, token).await;
        assert!(result.is_ok());
        assert!(ctx.get("executed").is_some());
    }

    #[tokio::test]
    async fn test_execute_linear_rpa_then_agent() {
        let registry = make_registry();
        let ai = MockAi;
        let executor = GraphExecutor::new(&registry, Some(&ai));
        let mut ctx: ExecutionContext<Ready> = ExecutionContext::<Unvalidated>::new().validate();
        let token = CancellationToken::new();

        let mut b = FlowGraph::builder("linear_mixed");
        let rpa = b.add_node(Node::Rpa {
            tool: "mock.tool",
            args: serde_json::json!({}),
            retry: RetryPolicy::default(),
        });
        let agent = b.add_node(Node::Ai {
            prompt: "analyze".into(),
            tools: vec![],
            max_turns: 3,
        });
        b.connect(rpa, EdgeKind::Success, agent);
        let graph = b.build().unwrap();

        let result = executor.execute(&graph, &mut ctx, token).await;
        assert!(result.is_ok());
        assert!(ctx.get("executed").is_some());
        assert!(ctx.get("ai_ran").is_some());
    }

    #[tokio::test]
    async fn test_execute_router_choice() {
        let registry = make_registry();
        let ai = MockAi;
        let executor = GraphExecutor::new(&registry, Some(&ai));
        let mut ctx: ExecutionContext<Ready> = ExecutionContext::<Unvalidated>::new().validate();
        let token = CancellationToken::new();

        let mut b = FlowGraph::builder("router_test");
        let router = b.add_node(Node::Router {
            prompt: "choose a".into(),
            options: vec![("a".into(), "go to a".into())],
        });
        let target = b.add_node(Node::Rpa {
            tool: "mock.tool",
            args: serde_json::json!({}),
            retry: RetryPolicy::default(),
        });
        b.on_choice(router, "a", target);
        let graph = b.build().unwrap();

        let result = executor.execute(&graph, &mut ctx, token).await;
        assert!(result.is_ok());
        assert!(ctx.get("decide_ran").is_some());
        assert!(ctx.get("executed").is_some());
    }

    #[tokio::test]
    async fn test_execute_cancelled() {
        let registry = make_registry();
        let executor = GraphExecutor::new(&registry, None);
        let mut ctx: ExecutionContext<Ready> = ExecutionContext::<Unvalidated>::new().validate();
        let cancelled = CancellationToken::new();
        cancelled.cancel();

        let graph = FlowGraph::single(
            "cancelled",
            Node::Rpa {
                tool: "mock.tool",
                args: serde_json::json!({}),
                retry: RetryPolicy::default(),
            },
        );

        let result = executor.execute(&graph, &mut ctx, cancelled).await;
        assert!(matches!(result, Err(SmithError::Cancelled)));
    }
}
