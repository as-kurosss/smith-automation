# Community 53: ToolRegistry

**Members:** 5

## Nodes

- **test_get_unknown_tool()** (`crates_smith_core_src_registry_rs_test_get_unknown_tool`, Function, degree: 3)
- **ToolRegistry** (`crates_smith_core_src_registry_rs_toolregistry`, Struct, degree: 7)
- **.execute()** (`crates_smith_core_src_registry_rs_toolregistry_execute`, Method, degree: 3)
- **.get()** (`crates_smith_core_src_registry_rs_toolregistry_get`, Method, degree: 4)
- **.list_tools()** (`crates_smith_core_src_registry_rs_toolregistry_list_tools`, Method, degree: 2)

## Relationships

- crates_smith_core_src_registry_rs_toolregistry → crates_smith_core_src_registry_rs_toolregistry_get (defines)
- crates_smith_core_src_registry_rs_toolregistry → crates_smith_core_src_registry_rs_toolregistry_execute (defines)
- crates_smith_core_src_registry_rs_toolregistry → crates_smith_core_src_registry_rs_toolregistry_list_tools (defines)
- crates_smith_core_src_registry_rs_toolregistry_execute → crates_smith_core_src_registry_rs_toolregistry_get (calls)
- crates_smith_core_src_registry_rs_test_get_unknown_tool → crates_smith_core_src_registry_rs_toolregistry_get (calls)

