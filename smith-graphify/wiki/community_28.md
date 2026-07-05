# Community 28: MockAi (28)

**Members:** 8

## Nodes

- **.execute()** (`crates_smith_graph_src_executor_rs_graphexecutor_a_execute`, Method, degree: 2)
- **.execute_node()** (`crates_smith_graph_src_executor_rs_graphexecutor_a_execute_node`, Method, degree: 7)
- **.execute_rpa()** (`crates_smith_graph_src_executor_rs_graphexecutor_a_execute_rpa`, Method, degree: 2)
- **.resolve_next()** (`crates_smith_graph_src_executor_rs_graphexecutor_a_resolve_next`, Method, degree: 1)
- **MockAi** (`crates_smith_graph_src_executor_rs_mockai`, Struct, degree: 4)
- **.agent_run()** (`crates_smith_graph_src_executor_rs_mockai_agent_run`, Method, degree: 2)
- **.decide()** (`crates_smith_graph_src_executor_rs_mockai_decide`, Method, degree: 2)
- **.think()** (`crates_smith_graph_src_executor_rs_mockai_think`, Method, degree: 2)

## Relationships

- crates_smith_graph_src_executor_rs_mockai → crates_smith_graph_src_executor_rs_mockai_agent_run (defines)
- crates_smith_graph_src_executor_rs_mockai → crates_smith_graph_src_executor_rs_mockai_think (defines)
- crates_smith_graph_src_executor_rs_mockai → crates_smith_graph_src_executor_rs_mockai_decide (defines)
- crates_smith_graph_src_executor_rs_graphexecutor_a_execute → crates_smith_graph_src_executor_rs_graphexecutor_a_execute_node (calls)
- crates_smith_graph_src_executor_rs_graphexecutor_a_execute → crates_smith_graph_src_executor_rs_graphexecutor_a_resolve_next (calls)
- crates_smith_graph_src_executor_rs_graphexecutor_a_execute_node → crates_smith_graph_src_executor_rs_graphexecutor_a_execute_rpa (calls)
- crates_smith_graph_src_executor_rs_graphexecutor_a_execute_node → crates_smith_graph_src_executor_rs_mockai_agent_run (calls)
- crates_smith_graph_src_executor_rs_graphexecutor_a_execute_node → crates_smith_graph_src_executor_rs_mockai_decide (calls)
- crates_smith_graph_src_executor_rs_graphexecutor_a_execute_node → crates_smith_graph_src_executor_rs_mockai_think (calls)

