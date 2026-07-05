# Community 10: WorkflowExecutor

**Members:** 13

## Nodes

- **executor** (`crates_smith_workflow_src_executor_rs`, File, degree: 22)
- **async_trait::async_trait** (`crates_smith_workflow_src_executor_rs_import_async_trait_async_trait`, Module, degree: 1)
- **crate::context::WorkflowContext** (`crates_smith_workflow_src_executor_rs_import_crate_context_workflowcontext`, Module, degree: 1)
- **crate::error::{AgentResult, WorkflowError}** (`crates_smith_workflow_src_executor_rs_import_crate_error_agentresult_workflowerror`, Module, degree: 1)
- **crate::step::{Step, StepKind}** (`crates_smith_workflow_src_executor_rs_import_crate_step_step_stepkind`, Module, degree: 1)
- **crate::workflow::Workflow** (`crates_smith_workflow_src_executor_rs_import_crate_workflow_workflow`, Module, degree: 1)
- **serde_json::Value** (`crates_smith_workflow_src_executor_rs_import_serde_json_value`, Module, degree: 1)
- **smith_core::{AiHandler, ExecutionContext, ToolRegistry}** (`crates_smith_workflow_src_executor_rs_import_smith_core_aihandler_executioncontext_toolregistry`, Module, degree: 1)
- **smith_core::{ContextValue, SmithResult, Tool, ToolConfig, ToolResult}** (`crates_smith_workflow_src_executor_rs_import_smith_core_contextvalue_smithresult_tool_toolconfig_toolresult`, Module, degree: 1)
- **super::*** (`crates_smith_workflow_src_executor_rs_import_super`, Module, degree: 1)
- **tokio_util::sync::CancellationToken** (`crates_smith_workflow_src_executor_rs_import_tokio_util_sync_cancellationtoken`, Module, degree: 1)
- **tracing::{info, warn}** (`crates_smith_workflow_src_executor_rs_import_tracing_info_warn`, Module, degree: 1)
- **WorkflowExecutor** (`crates_smith_workflow_src_executor_rs_workflowexecutor`, Struct, degree: 1)

## Relationships

- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_import_serde_json_value (imports)
- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_import_smith_core_aihandler_executioncontext_toolregistry (imports)
- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_import_tokio_util_sync_cancellationtoken (imports)
- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_import_tracing_info_warn (imports)
- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_import_crate_context_workflowcontext (imports)
- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_import_crate_error_agentresult_workflowerror (imports)
- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_import_crate_step_step_stepkind (imports)
- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_import_crate_workflow_workflow (imports)
- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_workflowexecutor (defines)
- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_import_super (imports)
- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_import_async_trait_async_trait (imports)
- crates_smith_workflow_src_executor_rs → crates_smith_workflow_src_executor_rs_import_smith_core_contextvalue_smithresult_tool_toolconfig_toolresult (imports)

