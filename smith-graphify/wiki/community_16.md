# Community 16: test_execute_single_rpa()

**Members:** 11

## Nodes

- **.new()** (`crates_smith_graph_src_executor_rs_graphexecutor_a_new`, Method, degree: 6)
- **make_registry()** (`crates_smith_graph_src_executor_rs_make_registry`, Function, degree: 6)
- **MockTool** (`crates_smith_graph_src_executor_rs_mocktool`, Struct, degree: 5)
- **.description()** (`crates_smith_graph_src_executor_rs_mocktool_description`, Method, degree: 1)
- **.execute()** (`crates_smith_graph_src_executor_rs_mocktool_execute`, Method, degree: 7)
- **.name()** (`crates_smith_graph_src_executor_rs_mocktool_name`, Method, degree: 1)
- **.schema()** (`crates_smith_graph_src_executor_rs_mocktool_schema`, Method, degree: 1)
- **test_execute_cancelled()** (`crates_smith_graph_src_executor_rs_test_execute_cancelled`, Function, degree: 4)
- **test_execute_linear_rpa_then_agent()** (`crates_smith_graph_src_executor_rs_test_execute_linear_rpa_then_agent`, Function, degree: 4)
- **test_execute_router_choice()** (`crates_smith_graph_src_executor_rs_test_execute_router_choice`, Function, degree: 4)
- **test_execute_single_rpa()** (`crates_smith_graph_src_executor_rs_test_execute_single_rpa`, Function, degree: 4)

## Relationships

- crates_smith_graph_src_executor_rs_mocktool → crates_smith_graph_src_executor_rs_mocktool_name (defines)
- crates_smith_graph_src_executor_rs_mocktool → crates_smith_graph_src_executor_rs_mocktool_description (defines)
- crates_smith_graph_src_executor_rs_mocktool → crates_smith_graph_src_executor_rs_mocktool_schema (defines)
- crates_smith_graph_src_executor_rs_mocktool → crates_smith_graph_src_executor_rs_mocktool_execute (defines)
- crates_smith_graph_src_executor_rs_make_registry → crates_smith_graph_src_executor_rs_graphexecutor_a_new (calls)
- crates_smith_graph_src_executor_rs_test_execute_single_rpa → crates_smith_graph_src_executor_rs_make_registry (calls)
- crates_smith_graph_src_executor_rs_test_execute_single_rpa → crates_smith_graph_src_executor_rs_graphexecutor_a_new (calls)
- crates_smith_graph_src_executor_rs_test_execute_single_rpa → crates_smith_graph_src_executor_rs_mocktool_execute (calls)
- crates_smith_graph_src_executor_rs_test_execute_linear_rpa_then_agent → crates_smith_graph_src_executor_rs_make_registry (calls)
- crates_smith_graph_src_executor_rs_test_execute_linear_rpa_then_agent → crates_smith_graph_src_executor_rs_graphexecutor_a_new (calls)
- crates_smith_graph_src_executor_rs_test_execute_linear_rpa_then_agent → crates_smith_graph_src_executor_rs_mocktool_execute (calls)
- crates_smith_graph_src_executor_rs_test_execute_router_choice → crates_smith_graph_src_executor_rs_make_registry (calls)
- crates_smith_graph_src_executor_rs_test_execute_router_choice → crates_smith_graph_src_executor_rs_graphexecutor_a_new (calls)
- crates_smith_graph_src_executor_rs_test_execute_router_choice → crates_smith_graph_src_executor_rs_mocktool_execute (calls)
- crates_smith_graph_src_executor_rs_test_execute_cancelled → crates_smith_graph_src_executor_rs_make_registry (calls)
- crates_smith_graph_src_executor_rs_test_execute_cancelled → crates_smith_graph_src_executor_rs_graphexecutor_a_new (calls)
- crates_smith_graph_src_executor_rs_test_execute_cancelled → crates_smith_graph_src_executor_rs_mocktool_execute (calls)

