# Community 27: WorkflowBuilder

**Members:** 8

## Nodes

- **.try_from()** (`crates_smith_workflow_src_workflow_rs_flowgraph_try_from`, Method, degree: 4)
- **step_to_node()** (`crates_smith_workflow_src_workflow_rs_step_to_node`, Function, degree: 3)
- **Workflow** (`crates_smith_workflow_src_workflow_rs_workflow`, Struct, degree: 2)
- **.new()** (`crates_smith_workflow_src_workflow_rs_workflow_new`, Method, degree: 3)
- **WorkflowBuilder** (`crates_smith_workflow_src_workflow_rs_workflowbuilder`, Struct, degree: 4)
- **.build()** (`crates_smith_workflow_src_workflow_rs_workflowbuilder_build`, Method, degree: 2)
- **.on_choice()** (`crates_smith_workflow_src_workflow_rs_workflowbuilder_on_choice`, Method, degree: 2)
- **.step()** (`crates_smith_workflow_src_workflow_rs_workflowbuilder_step`, Method, degree: 1)

## Relationships

- crates_smith_workflow_src_workflow_rs_workflow → crates_smith_workflow_src_workflow_rs_workflow_new (defines)
- crates_smith_workflow_src_workflow_rs_workflowbuilder → crates_smith_workflow_src_workflow_rs_workflowbuilder_step (defines)
- crates_smith_workflow_src_workflow_rs_workflowbuilder → crates_smith_workflow_src_workflow_rs_workflowbuilder_on_choice (defines)
- crates_smith_workflow_src_workflow_rs_workflowbuilder → crates_smith_workflow_src_workflow_rs_workflowbuilder_build (defines)
- crates_smith_workflow_src_workflow_rs_flowgraph_try_from → crates_smith_workflow_src_workflow_rs_workflow_new (calls)
- crates_smith_workflow_src_workflow_rs_flowgraph_try_from → crates_smith_workflow_src_workflow_rs_step_to_node (calls)
- crates_smith_workflow_src_workflow_rs_flowgraph_try_from → crates_smith_workflow_src_workflow_rs_workflowbuilder_on_choice (calls)
- crates_smith_workflow_src_workflow_rs_flowgraph_try_from → crates_smith_workflow_src_workflow_rs_workflowbuilder_build (calls)
- crates_smith_workflow_src_workflow_rs_step_to_node → crates_smith_workflow_src_workflow_rs_workflow_new (calls)

