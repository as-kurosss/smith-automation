# Community 25: run_graphify_build()

**Members:** 9

## Nodes

- **build_env_info()** (`apps_smith_context_src_main_rs_build_env_info`, Function, degree: 2)
- **build_git_log()** (`apps_smith_context_src_main_rs_build_git_log`, Function, degree: 2)
- **build_graph()** (`apps_smith_context_src_main_rs_build_graph`, Function, degree: 2)
- **collect_todos()** (`apps_smith_context_src_main_rs_collect_todos`, Function, degree: 2)
- **format_markdown()** (`apps_smith_context_src_main_rs_format_markdown`, Function, degree: 2)
- **load_graphify_artifacts()** (`apps_smith_context_src_main_rs_load_graphify_artifacts`, Function, degree: 2)
- **main()** (`apps_smith_context_src_main_rs_main`, Function, degree: 12)
- **read_workspace_cargo()** (`apps_smith_context_src_main_rs_read_workspace_cargo`, Function, degree: 2)
- **run_graphify_build()** (`apps_smith_context_src_main_rs_run_graphify_build`, Function, degree: 2)

## Relationships

- apps_smith_context_src_main_rs_main → apps_smith_context_src_main_rs_collect_todos (calls)
- apps_smith_context_src_main_rs_main → apps_smith_context_src_main_rs_build_graph (calls)
- apps_smith_context_src_main_rs_main → apps_smith_context_src_main_rs_run_graphify_build (calls)
- apps_smith_context_src_main_rs_main → apps_smith_context_src_main_rs_load_graphify_artifacts (calls)
- apps_smith_context_src_main_rs_main → apps_smith_context_src_main_rs_build_git_log (calls)
- apps_smith_context_src_main_rs_main → apps_smith_context_src_main_rs_build_env_info (calls)
- apps_smith_context_src_main_rs_main → apps_smith_context_src_main_rs_read_workspace_cargo (calls)
- apps_smith_context_src_main_rs_main → apps_smith_context_src_main_rs_format_markdown (calls)

