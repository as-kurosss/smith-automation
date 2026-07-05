// crates/smith-workflow/src/executor.rs

use serde_json::Value;
use smith_core::{AiHandler, ExecutionContext, ToolRegistry};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::context::WorkflowContext;
use crate::error::{AgentResult, WorkflowError};
use crate::step::{Step, StepKind};
use crate::workflow::Workflow;

/// Workflow executor.
///
/// Does not depend on AI — executes RPA steps via `ToolRegistry`,
/// and delegates Agent/Think/Decide steps to an external `AiHandler`.
pub struct WorkflowExecutor<'a> {
    registry: &'a ToolRegistry,
    ai_handler: Option<&'a dyn AiHandler>,
}

impl<'a> WorkflowExecutor<'a> {
    /// Creates a new executor.
    pub fn new(registry: &'a ToolRegistry, ai_handler: Option<&'a dyn AiHandler>) -> Self {
        Self {
            registry,
            ai_handler,
        }
    }

    /// Creates an executor for RPA-only steps (without AI).
    ///
    /// Convenient when the workflow consists only of `Step::rpa(...)`.
    #[must_use]
    pub fn new_rpa(registry: &'a ToolRegistry) -> Self {
        Self {
            registry,
            ai_handler: None,
        }
    }

    /// Executes the entire workflow.
    #[async_recursion::async_recursion]
    pub async fn execute(
        &self,
        workflow: Workflow,
        ctx: &mut ExecutionContext,
        token: CancellationToken,
    ) -> Result<AgentResult, WorkflowError> {
        let mut wf_ctx = WorkflowContext::new();
        wf_ctx.inner = std::mem::replace(ctx, ExecutionContext::new());

        let total_steps = workflow.steps.len();
        let mut step_idx = 0;

        while step_idx < total_steps {
            if token.is_cancelled() {
                return Err(WorkflowError::Cancelled);
            }

            wf_ctx.current_step = step_idx;
            info!(
                "Step {}/{}: {:?}",
                step_idx + 1,
                total_steps,
                workflow.steps[step_idx].kind_name()
            );

            match self
                .execute_step(
                    &workflow.steps[step_idx],
                    &workflow.choices,
                    &mut wf_ctx,
                    &token,
                )
                .await
            {
                Ok(result) => {
                    wf_ctx.set_step_result(step_idx, result);
                    step_idx += 1;
                }
                Err(WorkflowError::Cancelled) => return Err(WorkflowError::Cancelled),
                Err(e) => return Err(e),
            }
        }

        // Return ExecutionContext back
        std::mem::swap(ctx, &mut wf_ctx.inner);

        let elapsed = wf_ctx.elapsed_ms();
        let last_idx = step_idx.checked_sub(1);
        Ok(AgentResult {
            success: true,
            workflow_name: workflow.name,
            steps_completed: step_idx,
            output: last_idx
                .and_then(|i| wf_ctx.step_results.get(&i))
                .cloned()
                .unwrap_or(Value::Null),
            step_results: wf_ctx.step_results.clone(),
            execution_time_ms: elapsed,
        })
    }

    /// Executes a single step with access to choices for conditional routing.
    #[allow(clippy::needless_pass_by_value)]
    async fn execute_step(
        &self,
        step: &Step,
        choices: &std::collections::HashMap<usize, std::collections::HashMap<String, Workflow>>,
        ctx: &mut WorkflowContext,
        token: &CancellationToken,
    ) -> Result<Value, WorkflowError> {
        match &step.kind {
            StepKind::Rpa { name, args, retry } => {
                ctx.rpa_count += 1;
                self.execute_rpa(name, args, retry, ctx, token).await
            }
            StepKind::Agent {
                prompt,
                tools,
                max_steps,
            } => {
                ctx.agent_count += 1;
                let handler = self.ai_handler.ok_or(WorkflowError::AgentNotConfigured)?;
                handler
                    .agent_run(prompt, tools, *max_steps, &mut ctx.inner, token)
                    .await
                    .map_err(|e| WorkflowError::AgentError(e.to_string()))
            }
            StepKind::Think {
                prompt,
                output_schema,
            } => {
                ctx.agent_count += 1;
                let handler = self.ai_handler.ok_or(WorkflowError::AgentNotConfigured)?;
                handler
                    .think(prompt, output_schema, &mut ctx.inner, token)
                    .await
                    .map_err(|e| WorkflowError::AgentError(e.to_string()))
            }
            StepKind::Decide { prompt, options } => {
                ctx.agent_count += 1;
                let handler = self.ai_handler.ok_or(WorkflowError::AgentNotConfigured)?;
                let choice = handler
                    .decide(prompt, options, &mut ctx.inner, token)
                    .await
                    .map_err(|e| WorkflowError::AgentError(e.to_string()))?;

                // Check conditional routing
                if let Some(sub) = choices.get(&ctx.current_step).and_then(|c| c.get(&choice)) {
                    let mut sub_ctx = ExecutionContext::new();
                    let sub_result =
                        Box::pin(self.execute(sub.clone(), &mut sub_ctx, token.clone())).await?;
                    return Ok(serde_json::json!({
                        "choice": choice,
                        "sub_workflow": sub_result.output,
                    }));
                }

                Ok(serde_json::json!({ "choice": choice }))
            }
            StepKind::Workflow(sub) => {
                let mut sub_ctx = ExecutionContext::new();
                let sub_result =
                    Box::pin(self.execute(sub.clone(), &mut sub_ctx, token.clone())).await?;
                Ok(serde_json::json!({
                    "sub_workflow": sub_result.workflow_name,
                    "result": sub_result.output,
                }))
            }
        }
    }

    /// Executes an RPA step via ToolRegistry with retry.
    async fn execute_rpa(
        &self,
        name: &str,
        args: &Value,
        retry: &smith_core::RetryPolicy,
        ctx: &mut WorkflowContext,
        token: &CancellationToken,
    ) -> Result<Value, WorkflowError> {
        let max_retries = retry.max_retries;

        for attempt in 0..=max_retries {
            if token.is_cancelled() {
                return Err(WorkflowError::Cancelled);
            }

            match self
                .registry
                .execute(name, args.clone(), &mut ctx.inner, token.clone())
                .await
            {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == max_retries {
                        return Err(WorkflowError::StepError {
                            step_idx: ctx.current_step,
                            source: Box::new(crate::error::StepErrorContext {
                                tool: name.to_string(),
                                args: args.clone(),
                                inner: e,
                            }),
                        });
                    }
                    warn!(
                        "RPA tool '{name}' failed (attempt {}/{max_retries}): {e}",
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
    use async_trait::async_trait;
    use smith_core::{ContextValue, SmithResult, Tool, ToolConfig, ToolResult};

    /// Mock tool that returns a fixed result.
    struct MockTool;

    #[async_trait]
    impl Tool for MockTool {
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
            _config: ToolConfig,
            ctx: &mut ExecutionContext,
            _token: CancellationToken,
        ) -> SmithResult<ToolResult> {
            ctx.set("executed", ContextValue::Boolean(true));
            Ok(serde_json::json!({ "status": "ok" }))
        }
    }

    /// Mock AiHandler that returns canned responses.
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
    async fn test_execute_empty_workflow() {
        let registry = make_registry();
        let executor = WorkflowExecutor::new_rpa(&registry);
        let mut ctx = ExecutionContext::new();
        let token = CancellationToken::new();

        let wf = Workflow::new("empty").build().unwrap();
        let result = executor.execute(wf, &mut ctx, token).await.unwrap();

        assert!(result.success);
        assert_eq!(result.steps_completed, 0);
    }

    #[tokio::test]
    async fn test_execute_one_rpa_step() {
        let registry = make_registry();
        let executor = WorkflowExecutor::new_rpa(&registry);
        let mut ctx = ExecutionContext::new();
        let token = CancellationToken::new();

        let wf = Workflow::new("single")
            .step(Step::rpa("mock.tool"))
            .build()
            .unwrap();
        let result = executor.execute(wf, &mut ctx, token).await.unwrap();

        assert!(result.success);
        assert_eq!(result.steps_completed, 1);
        assert!(ctx.get("executed").is_some());
    }

    #[tokio::test]
    async fn test_execute_agent_step() {
        let registry = make_registry();
        let ai = MockAi;
        let executor = WorkflowExecutor::new(&registry, Some(&ai));
        let mut ctx = ExecutionContext::new();
        let token = CancellationToken::new();

        let wf = Workflow::new("with_ai")
            .step(Step::agent("do something"))
            .build()
            .unwrap();
        let result = executor.execute(wf, &mut ctx, token).await.unwrap();

        assert!(result.success);
        assert_eq!(result.steps_completed, 1);
        assert_eq!(result.output, serde_json::json!({ "result": "ai_ok" }));
    }

    #[tokio::test]
    async fn test_execute_cancellation() {
        let registry = make_registry();
        let executor = WorkflowExecutor::new_rpa(&registry);
        let mut ctx = ExecutionContext::new();
        let token = CancellationToken::new();
        token.cancel();

        let wf = Workflow::new("cancelled")
            .step(Step::rpa("mock.tool"))
            .build()
            .unwrap();
        let result = executor.execute(wf, &mut ctx, token).await;

        assert!(matches!(result, Err(WorkflowError::Cancelled)));
    }

    #[tokio::test]
    async fn test_execute_agent_not_configured() {
        let registry = make_registry();
        let executor = WorkflowExecutor::new_rpa(&registry);
        let mut ctx = ExecutionContext::new();
        let token = CancellationToken::new();

        let wf = Workflow::new("no_ai")
            .step(Step::agent("do something"))
            .build()
            .unwrap();
        let result = executor.execute(wf, &mut ctx, token).await;

        assert!(matches!(result, Err(WorkflowError::AgentNotConfigured)));
    }

    #[tokio::test]
    async fn test_execute_think_step() {
        let registry = make_registry();
        let ai = MockAi;
        let executor = WorkflowExecutor::new(&registry, Some(&ai));
        let mut ctx = ExecutionContext::new();
        let token = CancellationToken::new();

        let wf = Workflow::new("think_test")
            .step(Step::agent_think("analyze"))
            .build()
            .unwrap();
        let result = executor.execute(wf, &mut ctx, token).await.unwrap();

        assert_eq!(result.output, serde_json::json!({ "analysis": "done" }));
    }

    #[tokio::test]
    async fn test_execute_decide_step() {
        let registry = make_registry();
        let ai = MockAi;
        let executor = WorkflowExecutor::new(&registry, Some(&ai));
        let mut ctx = ExecutionContext::new();
        let token = CancellationToken::new();

        let wf = Workflow::new("decide_test")
            .step(Step::agent_decide("choose").options(&["a", "b"]))
            .build()
            .unwrap();
        let result = executor.execute(wf, &mut ctx, token).await.unwrap();

        assert_eq!(result.output, serde_json::json!({ "choice": "a" }));
    }
}
