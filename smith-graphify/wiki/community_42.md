# Community 42: MockAi

**Members:** 6

## Nodes

- **MockAi** (`crates_smith_workflow_src_executor_rs_mockai`, Struct, degree: 4)
- **.agent_run()** (`crates_smith_workflow_src_executor_rs_mockai_agent_run`, Method, degree: 2)
- **.decide()** (`crates_smith_workflow_src_executor_rs_mockai_decide`, Method, degree: 2)
- **.think()** (`crates_smith_workflow_src_executor_rs_mockai_think`, Method, degree: 2)
- **.execute()** (`crates_smith_workflow_src_executor_rs_workflowexecutor_a_execute`, Method, degree: 2)
- **.execute_step()** (`crates_smith_workflow_src_executor_rs_workflowexecutor_a_execute_step`, Method, degree: 7)

## Relationships

- crates_smith_workflow_src_executor_rs_mockai → crates_smith_workflow_src_executor_rs_mockai_agent_run (defines)
- crates_smith_workflow_src_executor_rs_mockai → crates_smith_workflow_src_executor_rs_mockai_think (defines)
- crates_smith_workflow_src_executor_rs_mockai → crates_smith_workflow_src_executor_rs_mockai_decide (defines)
- crates_smith_workflow_src_executor_rs_workflowexecutor_a_execute → crates_smith_workflow_src_executor_rs_workflowexecutor_a_execute_step (calls)
- crates_smith_workflow_src_executor_rs_workflowexecutor_a_execute_step → crates_smith_workflow_src_executor_rs_mockai_agent_run (calls)
- crates_smith_workflow_src_executor_rs_workflowexecutor_a_execute_step → crates_smith_workflow_src_executor_rs_mockai_think (calls)
- crates_smith_workflow_src_executor_rs_workflowexecutor_a_execute_step → crates_smith_workflow_src_executor_rs_mockai_decide (calls)

