# Community 22: WorkflowContext

**Members:** 11

## Nodes

- **context** (`crates_smith_workflow_src_context_rs`, File, degree: 4)
- **serde_json::Value** (`crates_smith_workflow_src_context_rs_import_serde_json_value`, Module, degree: 1)
- **smith_core::ExecutionContext** (`crates_smith_workflow_src_context_rs_import_smith_core_executioncontext`, Module, degree: 1)
- **std::collections::HashMap** (`crates_smith_workflow_src_context_rs_import_std_collections_hashmap`, Module, degree: 1)
- **WorkflowContext** (`crates_smith_workflow_src_context_rs_workflowcontext`, Struct, degree: 7)
- **.default()** (`crates_smith_workflow_src_context_rs_workflowcontext_default`, Method, degree: 2)
- **.elapsed_ms()** (`crates_smith_workflow_src_context_rs_workflowcontext_elapsed_ms`, Method, degree: 2)
- **.get_step_result()** (`crates_smith_workflow_src_context_rs_workflowcontext_get_step_result`, Method, degree: 1)
- **.new()** (`crates_smith_workflow_src_context_rs_workflowcontext_new`, Method, degree: 3)
- **.now()** (`crates_smith_workflow_src_context_rs_workflowcontext_now`, Method, degree: 3)
- **.set_step_result()** (`crates_smith_workflow_src_context_rs_workflowcontext_set_step_result`, Method, degree: 1)

## Relationships

- crates_smith_workflow_src_context_rs → crates_smith_workflow_src_context_rs_import_std_collections_hashmap (imports)
- crates_smith_workflow_src_context_rs → crates_smith_workflow_src_context_rs_import_serde_json_value (imports)
- crates_smith_workflow_src_context_rs → crates_smith_workflow_src_context_rs_import_smith_core_executioncontext (imports)
- crates_smith_workflow_src_context_rs → crates_smith_workflow_src_context_rs_workflowcontext (defines)
- crates_smith_workflow_src_context_rs_workflowcontext → crates_smith_workflow_src_context_rs_workflowcontext_new (defines)
- crates_smith_workflow_src_context_rs_workflowcontext → crates_smith_workflow_src_context_rs_workflowcontext_set_step_result (defines)
- crates_smith_workflow_src_context_rs_workflowcontext → crates_smith_workflow_src_context_rs_workflowcontext_get_step_result (defines)
- crates_smith_workflow_src_context_rs_workflowcontext → crates_smith_workflow_src_context_rs_workflowcontext_elapsed_ms (defines)
- crates_smith_workflow_src_context_rs_workflowcontext → crates_smith_workflow_src_context_rs_workflowcontext_now (defines)
- crates_smith_workflow_src_context_rs_workflowcontext → crates_smith_workflow_src_context_rs_workflowcontext_default (defines)
- crates_smith_workflow_src_context_rs_workflowcontext_new → crates_smith_workflow_src_context_rs_workflowcontext_now (calls)
- crates_smith_workflow_src_context_rs_workflowcontext_elapsed_ms → crates_smith_workflow_src_context_rs_workflowcontext_now (calls)
- crates_smith_workflow_src_context_rs_workflowcontext_default → crates_smith_workflow_src_context_rs_workflowcontext_new (calls)

