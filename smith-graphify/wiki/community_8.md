# Community 8: test_decide_step_kind_name()

**Members:** 15

## Nodes

- **Step** (`crates_smith_workflow_src_step_rs_step`, Struct, degree: 14)
- **.agent()** (`crates_smith_workflow_src_step_rs_step_agent`, Method, degree: 3)
- **.agent_decide()** (`crates_smith_workflow_src_step_rs_step_agent_decide`, Method, degree: 3)
- **.context()** (`crates_smith_workflow_src_step_rs_step_context`, Method, degree: 1)
- **.kind_name()** (`crates_smith_workflow_src_step_rs_step_kind_name`, Method, degree: 1)
- **.max_steps()** (`crates_smith_workflow_src_step_rs_step_max_steps`, Method, degree: 1)
- **.options()** (`crates_smith_workflow_src_step_rs_step_options`, Method, degree: 3)
- **.retry()** (`crates_smith_workflow_src_step_rs_step_retry`, Method, degree: 1)
- **.schema()** (`crates_smith_workflow_src_step_rs_step_schema`, Method, degree: 1)
- **.tools()** (`crates_smith_workflow_src_step_rs_step_tools`, Method, degree: 2)
- **.workflow()** (`crates_smith_workflow_src_step_rs_step_workflow`, Method, degree: 1)
- **test_agent_step_kind_name()** (`crates_smith_workflow_src_step_rs_test_agent_step_kind_name`, Function, degree: 2)
- **test_agent_tools_sets_tools()** (`crates_smith_workflow_src_step_rs_test_agent_tools_sets_tools`, Function, degree: 3)
- **test_decide_options_are_set()** (`crates_smith_workflow_src_step_rs_test_decide_options_are_set`, Function, degree: 3)
- **test_decide_step_kind_name()** (`crates_smith_workflow_src_step_rs_test_decide_step_kind_name`, Function, degree: 3)

## Relationships

- crates_smith_workflow_src_step_rs_step → crates_smith_workflow_src_step_rs_step_retry (defines)
- crates_smith_workflow_src_step_rs_step → crates_smith_workflow_src_step_rs_step_agent (defines)
- crates_smith_workflow_src_step_rs_step → crates_smith_workflow_src_step_rs_step_tools (defines)
- crates_smith_workflow_src_step_rs_step → crates_smith_workflow_src_step_rs_step_max_steps (defines)
- crates_smith_workflow_src_step_rs_step → crates_smith_workflow_src_step_rs_step_schema (defines)
- crates_smith_workflow_src_step_rs_step → crates_smith_workflow_src_step_rs_step_agent_decide (defines)
- crates_smith_workflow_src_step_rs_step → crates_smith_workflow_src_step_rs_step_context (defines)
- crates_smith_workflow_src_step_rs_step → crates_smith_workflow_src_step_rs_step_options (defines)
- crates_smith_workflow_src_step_rs_step → crates_smith_workflow_src_step_rs_step_workflow (defines)
- crates_smith_workflow_src_step_rs_step → crates_smith_workflow_src_step_rs_step_kind_name (defines)
- crates_smith_workflow_src_step_rs_test_agent_step_kind_name → crates_smith_workflow_src_step_rs_step_agent (calls)
- crates_smith_workflow_src_step_rs_test_decide_step_kind_name → crates_smith_workflow_src_step_rs_step_options (calls)
- crates_smith_workflow_src_step_rs_test_decide_step_kind_name → crates_smith_workflow_src_step_rs_step_agent_decide (calls)
- crates_smith_workflow_src_step_rs_test_agent_tools_sets_tools → crates_smith_workflow_src_step_rs_step_tools (calls)
- crates_smith_workflow_src_step_rs_test_agent_tools_sets_tools → crates_smith_workflow_src_step_rs_step_agent (calls)
- crates_smith_workflow_src_step_rs_test_decide_options_are_set → crates_smith_workflow_src_step_rs_step_options (calls)
- crates_smith_workflow_src_step_rs_test_decide_options_are_set → crates_smith_workflow_src_step_rs_step_agent_decide (calls)

