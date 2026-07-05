# Community 4: test_node_kind_name()

**Members:** 17

## Nodes

- **node** (`crates_smith_graph_src_node_rs`, File, degree: 14)
- **EdgeKind** (`crates_smith_graph_src_node_rs_edgekind`, Enum, degree: 1)
- **Edges** (`crates_smith_graph_src_node_rs_edges`, Struct, degree: 2)
- **.none()** (`crates_smith_graph_src_node_rs_edges_none`, Method, degree: 2)
- **pub(crate) use crate::graph::FlowGraph** (`crates_smith_graph_src_node_rs_import_pub_crate_use_crate_graph_flowgraph`, Module, degree: 1)
- **pub use smith_core::RetryPolicy** (`crates_smith_graph_src_node_rs_import_pub_use_smith_core_retrypolicy`, Module, degree: 1)
- **serde_json::Value** (`crates_smith_graph_src_node_rs_import_serde_json_value`, Module, degree: 1)
- **smith_core::RetryPolicy** (`crates_smith_graph_src_node_rs_import_smith_core_retrypolicy`, Module, degree: 1)
- **std::collections::HashMap** (`crates_smith_graph_src_node_rs_import_std_collections_hashmap`, Module, degree: 1)
- **std::time::Duration** (`crates_smith_graph_src_node_rs_import_std_time_duration`, Module, degree: 1)
- **super::*** (`crates_smith_graph_src_node_rs_import_super`, Module, degree: 1)
- **Node** (`crates_smith_graph_src_node_rs_node`, Enum, degree: 2)
- **.kind_name()** (`crates_smith_graph_src_node_rs_node_kind_name`, Method, degree: 1)
- **NodeId** (`crates_smith_graph_src_node_rs_nodeid`, Struct, degree: 1)
- **NodeIO** (`crates_smith_graph_src_node_rs_nodeio`, Struct, degree: 1)
- **test_edges_none()** (`crates_smith_graph_src_node_rs_test_edges_none`, Function, degree: 2)
- **test_node_kind_name()** (`crates_smith_graph_src_node_rs_test_node_kind_name`, Function, degree: 1)

## Relationships

- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_import_std_collections_hashmap (imports)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_import_std_time_duration (imports)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_import_serde_json_value (imports)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_import_pub_crate_use_crate_graph_flowgraph (imports)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_import_pub_use_smith_core_retrypolicy (imports)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_nodeid (defines)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_nodeio (defines)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_node (defines)
- crates_smith_graph_src_node_rs_node → crates_smith_graph_src_node_rs_node_kind_name (defines)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_edgekind (defines)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_edges (defines)
- crates_smith_graph_src_node_rs_edges → crates_smith_graph_src_node_rs_edges_none (defines)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_import_super (imports)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_import_smith_core_retrypolicy (imports)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_test_node_kind_name (defines)
- crates_smith_graph_src_node_rs → crates_smith_graph_src_node_rs_test_edges_none (defines)
- crates_smith_graph_src_node_rs_test_edges_none → crates_smith_graph_src_node_rs_edges_none (calls)

