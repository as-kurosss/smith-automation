# Community 9: test_think_step_kind_name()

**Members:** 14

## Nodes

- **step** (`crates_smith_workflow_src_step_rs`, File, degree: 15)
- **crate::workflow::Workflow** (`crates_smith_workflow_src_step_rs_import_crate_workflow_workflow`, Module, degree: 1)
- **pub use smith_core::RetryPolicy** (`crates_smith_workflow_src_step_rs_import_pub_use_smith_core_retrypolicy`, Module, degree: 1)
- **serde_json::Value** (`crates_smith_workflow_src_step_rs_import_serde_json_value`, Module, degree: 1)
- **super::*** (`crates_smith_workflow_src_step_rs_import_super`, Module, degree: 1)
- **tracing::warn** (`crates_smith_workflow_src_step_rs_import_tracing_warn`, Module, degree: 1)
- **.agent_think()** (`crates_smith_workflow_src_step_rs_step_agent_think`, Method, degree: 2)
- **.args()** (`crates_smith_workflow_src_step_rs_step_args`, Method, degree: 2)
- **.rpa()** (`crates_smith_workflow_src_step_rs_step_rpa`, Method, degree: 3)
- **StepKind** (`crates_smith_workflow_src_step_rs_stepkind`, Enum, degree: 1)
- **test_retry_policy_defaults()** (`crates_smith_workflow_src_step_rs_test_retry_policy_defaults`, Function, degree: 1)
- **test_rpa_args_sets_args()** (`crates_smith_workflow_src_step_rs_test_rpa_args_sets_args`, Function, degree: 3)
- **test_rpa_step_kind_name()** (`crates_smith_workflow_src_step_rs_test_rpa_step_kind_name`, Function, degree: 2)
- **test_think_step_kind_name()** (`crates_smith_workflow_src_step_rs_test_think_step_kind_name`, Function, degree: 2)

## Relationships

- crates_smith_workflow_src_step_rs → crates_smith_workflow_src_step_rs_import_serde_json_value (imports)
- crates_smith_workflow_src_step_rs → crates_smith_workflow_src_step_rs_import_pub_use_smith_core_retrypolicy (imports)
- crates_smith_workflow_src_step_rs → crates_smith_workflow_src_step_rs_import_tracing_warn (imports)
- crates_smith_workflow_src_step_rs → crates_smith_workflow_src_step_rs_import_crate_workflow_workflow (imports)
- crates_smith_workflow_src_step_rs → crates_smith_workflow_src_step_rs_stepkind (defines)
- crates_smith_workflow_src_step_rs → crates_smith_workflow_src_step_rs_import_super (imports)
- crates_smith_workflow_src_step_rs → crates_smith_workflow_src_step_rs_test_rpa_step_kind_name (defines)
- crates_smith_workflow_src_step_rs → crates_smith_workflow_src_step_rs_test_think_step_kind_name (defines)
- crates_smith_workflow_src_step_rs → crates_smith_workflow_src_step_rs_test_rpa_args_sets_args (defines)
- crates_smith_workflow_src_step_rs → crates_smith_workflow_src_step_rs_test_retry_policy_defaults (defines)
- crates_smith_workflow_src_step_rs_test_rpa_step_kind_name → crates_smith_workflow_src_step_rs_step_rpa (calls)
- crates_smith_workflow_src_step_rs_test_think_step_kind_name → crates_smith_workflow_src_step_rs_step_agent_think (calls)
- crates_smith_workflow_src_step_rs_test_rpa_args_sets_args → crates_smith_workflow_src_step_rs_step_args (calls)
- crates_smith_workflow_src_step_rs_test_rpa_args_sets_args → crates_smith_workflow_src_step_rs_step_rpa (calls)

