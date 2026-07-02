# Community 7: test_set_and_get_variable()

**Members:** 10

## Nodes

- **ExecutionContext** (`crates_smith_core_src_context_rs_executioncontext`, Struct, degree: 7)
- **.default()** (`crates_smith_core_src_context_rs_executioncontext_default`, Method, degree: 2)
- **.get()** (`crates_smith_core_src_context_rs_executioncontext_get`, Method, degree: 1)
- **.new()** (`crates_smith_core_src_context_rs_executioncontext_new`, Method, degree: 8)
- **.pop_scope()** (`crates_smith_core_src_context_rs_executioncontext_pop_scope`, Method, degree: 3)
- **.push_scope()** (`crates_smith_core_src_context_rs_executioncontext_push_scope`, Method, degree: 3)
- **.set()** (`crates_smith_core_src_context_rs_executioncontext_set`, Method, degree: 4)
- **test_pop_scope_does_not_remove_global()** (`crates_smith_core_src_context_rs_test_pop_scope_does_not_remove_global`, Function, degree: 4)
- **test_push_scope_isolation()** (`crates_smith_core_src_context_rs_test_push_scope_isolation`, Function, degree: 5)
- **test_set_and_get_variable()** (`crates_smith_core_src_context_rs_test_set_and_get_variable`, Function, degree: 3)

## Relationships

- crates_smith_core_src_context_rs_executioncontext → crates_smith_core_src_context_rs_executioncontext_new (defines)
- crates_smith_core_src_context_rs_executioncontext → crates_smith_core_src_context_rs_executioncontext_push_scope (defines)
- crates_smith_core_src_context_rs_executioncontext → crates_smith_core_src_context_rs_executioncontext_pop_scope (defines)
- crates_smith_core_src_context_rs_executioncontext → crates_smith_core_src_context_rs_executioncontext_set (defines)
- crates_smith_core_src_context_rs_executioncontext → crates_smith_core_src_context_rs_executioncontext_get (defines)
- crates_smith_core_src_context_rs_executioncontext → crates_smith_core_src_context_rs_executioncontext_default (defines)
- crates_smith_core_src_context_rs_executioncontext_push_scope → crates_smith_core_src_context_rs_executioncontext_new (calls)
- crates_smith_core_src_context_rs_executioncontext_default → crates_smith_core_src_context_rs_executioncontext_new (calls)
- crates_smith_core_src_context_rs_test_set_and_get_variable → crates_smith_core_src_context_rs_executioncontext_new (calls)
- crates_smith_core_src_context_rs_test_set_and_get_variable → crates_smith_core_src_context_rs_executioncontext_set (calls)
- crates_smith_core_src_context_rs_test_push_scope_isolation → crates_smith_core_src_context_rs_executioncontext_new (calls)
- crates_smith_core_src_context_rs_test_push_scope_isolation → crates_smith_core_src_context_rs_executioncontext_set (calls)
- crates_smith_core_src_context_rs_test_push_scope_isolation → crates_smith_core_src_context_rs_executioncontext_push_scope (calls)
- crates_smith_core_src_context_rs_test_push_scope_isolation → crates_smith_core_src_context_rs_executioncontext_pop_scope (calls)
- crates_smith_core_src_context_rs_test_pop_scope_does_not_remove_global → crates_smith_core_src_context_rs_executioncontext_new (calls)
- crates_smith_core_src_context_rs_test_pop_scope_does_not_remove_global → crates_smith_core_src_context_rs_executioncontext_set (calls)
- crates_smith_core_src_context_rs_test_pop_scope_does_not_remove_global → crates_smith_core_src_context_rs_executioncontext_pop_scope (calls)

