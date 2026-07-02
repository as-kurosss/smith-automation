# 📊 Graph Analysis Report

**Root:** `.`

## Summary

| Metric | Value |
|--------|-------|
| Nodes | 245 |
| Edges | 307 |
| Communities | 26 |
| Hyperedges | 0 |

### Confidence Breakdown

| Level | Count | Percentage |
|-------|-------|------------|
| EXTRACTED | 229 | 74.6% |
| INFERRED | 78 | 25.4% |
| AMBIGUOUS | 0 | 0.0% |

## 🌟 God Nodes (Most Connected)

| Node | Degree | Community |
|------|--------|-----------|
| smith-context::main | 32 | 0 |
| smith-daemon::main | 21 | 1 |
| registry | 18 | 5 |
| context | 16 | 4 |
| ElementSelector | 13 | 3 |
| main() | 12 | 0 |
| error | 9 | 6 |
| process | 8 | 24 |
| selector | 8 | 10 |
| FindTool | 7 | 15 |

## 🔮 Surprising Connections

- **crates_smith_core_src_context_rs** → **crates_smith_core_src_context_rs_contextvalue** (defines)
- **crates_smith_core_src_context_rs** → **crates_smith_core_src_context_rs_executioncontext** (defines)
- **crates_smith_core_src_context_rs** → **crates_smith_core_src_context_rs_test_set_and_get_variable** (defines)
- **crates_smith_core_src_context_rs** → **crates_smith_core_src_context_rs_test_push_scope_isolation** (defines)
- **crates_smith_core_src_context_rs** → **crates_smith_core_src_context_rs_test_pop_scope_does_not_remove_global** (defines)

## 🏘️ Communities

### Community 0 — TreeNode (33 nodes, cohesion: 0.09)

- main
- build_env_info()
- build_git_log()
- build_graph()
- build_stats()
- build_tree()
- Cli
- collect_files()
- collect_todos()
- detect_language()
- extract_crate_name()
- FileEntry
- format_markdown()
- GraphifyArtifacts
- anyhow::Result
- chrono::Local
- clap::Parser
- ignore::gitignore::GitignoreBuilder
- std::collections::{BTreeMap, HashMap}
- std::fmt::Write
- _…and 13 more_

### Community 1 — tools_handler() (22 nodes, cohesion: 0.11)

- main
- AppState
- classify_error()
- execute_handler()
- ExecuteRequest
- ExecuteResponse
- health_handler()
- axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
}
- serde::{Deserialize, Serialize}
- serde_json::{json, Value}
- smith_core::{ExecutionContext, SmithError, ToolRegistry}
- std::net::SocketAddr
- std::sync::Arc
- tokio::sync::Mutex
- tokio_util::sync::CancellationToken
- tracing::{info, warn}
- main()
- parse_args()
- register_windows_tools()
- reset_handler()
- _…and 2 more_

### Community 2 — ToolRegistry (17 nodes, cohesion: 0.21)

- test_execute_success()
- test_execute_unknown_tool()
- test_get_unknown_tool()
- test_list_tools()
- test_register_and_get_tool()
- TestTool
- .description()
- .execute()
- .name()
- .schema()
- ToolRegistry
- .default()
- .execute()
- .get()
- .list_tools()
- .new()
- .register()

### Community 3 — parse_control_type() (14 nodes, cohesion: 0.21)

- ElementSelector
- .automation_id()
- .build_condition()
- .class_name()
- .control_type()
- .default()
- .find_all()
- .find_first()
- .find_from_desktop()
- .name()
- .new()
- .pid()
- .timeout()
- parse_control_type()

### Community 4 — test_new_creates_empty_scope() (12 nodes, cohesion: 0.17)

- context
- crate::error::{SmithError, SmithResult}
- std::any::Any
- std::collections::HashMap
- std::sync::Arc
- super::*
- test_context_value_null()
- test_context_value_try_as_boolean()
- test_context_value_try_as_number()
- test_context_value_try_as_string()
- test_get_returns_none_for_missing_key()
- test_new_creates_empty_scope()

### Community 5 — test_new_creates_empty_registry() (12 nodes, cohesion: 0.17)

- registry
- async_trait::async_trait
- crate::context::ContextValue
- crate::context::ExecutionContext
- crate::error::{SmithError, SmithResult}
- crate::tool::{Tool, ToolConfig, ToolResult}
- serde_json::json
- std::collections::HashMap
- super::*
- tokio_util::sync::CancellationToken
- test_default_is_empty()
- test_new_creates_empty_registry()

### Community 6 — test_platform_error_display() (10 nodes, cohesion: 0.20)

- error
- super::*
- thiserror::Error
- SmithError
- test_cancelled_display()
- test_context_error_display()
- test_conversion_from_anyhow_error()
- test_element_not_found_display()
- test_invalid_params_display()
- test_platform_error_display()

### Community 7 — test_set_and_get_variable() (10 nodes, cohesion: 0.38)

- ExecutionContext
- .default()
- .get()
- .new()
- .pop_scope()
- .push_scope()
- .set()
- test_pop_scope_does_not_remove_global()
- test_push_scope_isolation()
- test_set_and_get_variable()

### Community 8 — lib (8 nodes, cohesion: 0.25)

- lib
- pub use element::SafeUIElement
- pub use selector::ElementSelector
- pub use tools::ClickTool
- pub use tools::FindTool
- pub use tools::InputTextTool
- pub use tools::ProcessTool
- pub use tools::SetTextTool

### Community 9 — set_text (7 nodes, cohesion: 0.29)

- set_text
- async_trait::async_trait
- crate::element::SafeUIElement
- crate::selector::ElementSelector
- serde_json::{Value, json}
- smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult}
- tokio_util::sync::CancellationToken

### Community 10 — selector (7 nodes, cohesion: 0.29)

- selector
- smith_core::SmithError
- std::time::Duration
- uiautomation::{Condition, UIElement}
- uiautomation::core::UIAutomation
- uiautomation::types::{ControlType, PropertyConditionFlags, TreeScope, UIProperty}
- uiautomation::variants::Variant

### Community 11 — input_text (7 nodes, cohesion: 0.29)

- input_text
- async_trait::async_trait
- crate::element::SafeUIElement
- crate::selector::ElementSelector
- serde_json::{Value, json}
- smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult}
- tokio_util::sync::CancellationToken

### Community 12 — SetTextTool (7 nodes, cohesion: 0.43)

- SetTextTool
- .default()
- .description()
- .execute()
- .name()
- .new()
- .schema()

### Community 13 — InputTextTool (7 nodes, cohesion: 0.43)

- InputTextTool
- .default()
- .description()
- .execute()
- .name()
- .new()
- .schema()

### Community 14 — SafeUIElement (7 nodes, cohesion: 0.29)

- element
- std::sync::Arc
- uiautomation::UIElement
- SafeUIElement
- .clone()
- .inner()
- .new()

### Community 15 — FindTool (7 nodes, cohesion: 0.43)

- FindTool
- .default()
- .description()
- .execute()
- .name()
- .new()
- .schema()

### Community 16 — ClickTool (7 nodes, cohesion: 0.33)

- ClickTool
- .default()
- .description()
- .execute()
- .name()
- .new()
- .schema()

### Community 17 — Tool (7 nodes, cohesion: 0.29)

- tool
- async_trait::async_trait
- crate::context::ExecutionContext
- crate::error::SmithResult
- serde_json::Value
- tokio_util::sync::CancellationToken
- Tool

### Community 18 — find (7 nodes, cohesion: 0.29)

- find
- async_trait::async_trait
- crate::element::SafeUIElement
- crate::selector::ElementSelector
- serde_json::{Value, json}
- smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult}
- tokio_util::sync::CancellationToken

### Community 19 — mod (6 nodes, cohesion: 0.33)

- mod
- pub use click::ClickTool
- pub use find::FindTool
- pub use input_text::InputTextTool
- pub use process::ProcessTool
- pub use set_text::SetTextTool

### Community 20 — click (6 nodes, cohesion: 0.33)

- click
- async_trait::async_trait
- crate::element::SafeUIElement
- serde_json::{Value, json}
- smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult}
- tokio_util::sync::CancellationToken

### Community 21 — lib (21) (5 nodes, cohesion: 0.40)

- lib
- pub use context::{ContextValue, ExecutionContext}
- pub use error::{SmithError, SmithResult}
- pub use registry::ToolRegistry
- pub use tool::{Tool, ToolConfig, ToolResult}

### Community 22 — action_stop() (5 nodes, cohesion: 0.60)

- action_list()
- action_start()
- action_stop()
- .execute()
- .new()

### Community 23 — ProcessTool (5 nodes, cohesion: 0.40)

- ProcessTool
- .default()
- .description()
- .name()
- .schema()

### Community 24 — process (5 nodes, cohesion: 0.40)

- process
- async_trait::async_trait
- serde_json::{Value, json}
- smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult}
- tokio_util::sync::CancellationToken

### Community 25 — .try_as_string() (5 nodes, cohesion: 0.40)

- ContextValue
- .try_as_boolean()
- .try_as_custom()
- .try_as_number()
- .try_as_string()

## 🕳️ Knowledge Gaps

No isolated nodes.

## 💰 Token Cost

| File | Tokens |
|------|--------|
| input | 0 |
| output | 0 |
| **Total** | **0** |

## ❓ Suggested Questions

1. How does 'crates_smith_windows_src_tools_process_rs_processtool' relate to 3 different communities (ProcessTool, action_stop(), process)?
1. How does 'crates_smith_windows_src_tools_process_rs' relate to 3 different communities (ProcessTool, action_stop(), process)?
1. How does 'crates_smith_core_src_context_rs' relate to 3 different communities (test_new_creates_empty_scope(), test_set_and_get_variable(), .try_as_string())?
1. Why is 'ToolRegistry' (17 nodes) loosely connected (cohesion 0.21)? Should it be split?
1. Why is 'lib' (8 nodes) loosely connected (cohesion 0.25)? Should it be split?
1. Why is 'parse_control_type()' (14 nodes) loosely connected (cohesion 0.21)? Should it be split?
1. Why is 'set_text' (7 nodes) loosely connected (cohesion 0.29)? Should it be split?

---
_Generated by graphify-rs_
