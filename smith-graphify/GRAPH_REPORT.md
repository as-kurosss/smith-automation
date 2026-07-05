# 📊 Graph Analysis Report

**Root:** `.`

## Summary

| Metric | Value |
|--------|-------|
| Nodes | 581 |
| Edges | 757 |
| Communities | 66 |
| Hyperedges | 0 |

### Confidence Breakdown

| Level | Count | Percentage |
|-------|-------|------------|
| EXTRACTED | 528 | 69.7% |
| INFERRED | 229 | 30.3% |
| AMBIGUOUS | 0 | 0.0% |

## 🌟 God Nodes (Most Connected)

| Node | Degree | Community |
|------|--------|-----------|
| main | 32 | 2 |
| agent | 30 | 0 |
| executor | 22 | 10 |
| recorder | 21 | 1 |
| executor | 18 | 13 |
| registry | 18 | 15 |
| context | 16 | 19 |
| step | 15 | 9 |
| mod | 15 | 7 |
| mod | 15 | 5 |

## 🔮 Surprising Connections

- **apps_smith_context_src_main_rs_main** → **apps_smith_context_src_main_rs_collect_files** (calls)
- **apps_smith_context_src_main_rs_main** → **apps_smith_context_src_main_rs_build_tree** (calls)
- **apps_smith_context_src_main_rs_main** → **apps_smith_context_src_main_rs_build_stats** (calls)
- **crates_smith_ai_src_agent_rs_test_think_returns_plain_text** → **crates_smith_ai_src_agent_rs_make_agent** (calls)
- **crates_smith_ai_src_agent_rs_test_think_parses_json** → **crates_smith_ai_src_agent_rs_make_agent** (calls)

## 🏘️ Communities

### Community 0 — test_think_returns_plain_text() (28 nodes, cohesion: 0.08)

- agent
- AgentLike
- async_trait::async_trait
- crate::provider::ProviderConfig
- futures::future::BoxFuture
- rig::agent::Agent
- rig::agent::PromptHook
- rig::client::CompletionClient
- rig::completion::CompletionModel
- rig::completion::Prompt
- rig::providers::anthropic
- rig::providers::openai
- rig::tool::ToolDyn
- serde_json::Value
- smith_core::{AiHandler, ExecutionContext, SmithError, SmithResult}
- super::*
- tokio_util::sync::CancellationToken
- tracing::{info, warn}
- MockAgent
- SmithAgent
- _…and 8 more_

### Community 1 — wait_single_capture() (22 nodes, cohesion: 0.13)

- recorder
- append_capture()
- flush_input()
- chrono::Utc
- crate::capture
- crate::types::{
    Action, BestSelector, Capture, CaptureOutput, CapturedElement, SeriesRecording,
}
- rdev::{listen, EventType, Key}
- std::fs
- std::io::{self, Write}
- std::sync::mpsc
- std::sync::mpsc::{Receiver, Sender}
- is_modifier()
- is_printable_key()
- label()
- run_series_mode()
- run_single_mode()
- SeriesEvent
- SingleEvent
- spawn_series_listener()
- spawn_single_listener()
- _…and 2 more_

### Community 2 — TreeNode (19 nodes, cohesion: 0.12)

- main
- build_stats()
- build_tree()
- Cli
- extract_crate_name()
- FileEntry
- GraphifyArtifacts
- anyhow::Result
- chrono::Local
- clap::Parser
- ignore::gitignore::GitignoreBuilder
- std::collections::{BTreeMap, HashMap}
- std::fmt::Write
- std::path::{Path, PathBuf}
- walkdir::WalkDir
- MarkdownContext
- ProjectStats
- render_tree()
- TreeNode

### Community 3 — test_build_single_node() (18 nodes, cohesion: 0.22)

- FlowGraph
- .builder()
- .single()
- FlowGraphBuilder
- .add_node()
- .add_node_with_id()
- .build()
- .connect()
- .find_incoming()
- .new()
- .on_choice()
- .set_entry()
- .with_io()
- test_build_empty_fails()
- test_build_linear_two_nodes()
- test_build_router_requires_choice()
- test_build_router_with_success_is_error()
- test_build_single_node()

### Community 4 — test_node_kind_name() (17 nodes, cohesion: 0.13)

- node
- EdgeKind
- Edges
- .none()
- pub(crate) use crate::graph::FlowGraph
- pub use smith_core::RetryPolicy
- serde_json::Value
- smith_core::RetryPolicy
- std::collections::HashMap
- std::time::Duration
- super::*
- Node
- .kind_name()
- NodeId
- NodeIO
- test_edges_none()
- test_node_kind_name()

### Community 5 — resolve_element_from_config() (16 nodes, cohesion: 0.13)

- mod
- apply_delay_after()
- apply_delay_before()
- crate::element::SafeUIElement
- crate::selector::ElementSelector
- pub(crate) use self::helpers::{
    apply_delay_after, apply_delay_before, resolve_element_from_config,
}
- pub use click::ClickTool
- pub use find::FindTool
- pub use input_text::InputTextTool
- pub use process::ProcessTool
- pub use set_text::SetTextTool
- pub use wait::WaitTool
- serde_json::Value
- smith_core::{ExecutionContext, SmithError, SmithResult}
- super::{ExecutionContext, SmithError, SmithResult, Value}
- resolve_element_from_config()

### Community 6 — test_execute_think_step() (16 nodes, cohesion: 0.27)

- make_registry()
- MockTool
- .description()
- .execute()
- .name()
- .schema()
- test_execute_agent_not_configured()
- test_execute_agent_step()
- test_execute_cancellation()
- test_execute_decide_step()
- test_execute_empty_workflow()
- test_execute_one_rpa_step()
- test_execute_think_step()
- .execute_rpa()
- .new()
- .new_rpa()

### Community 7 — test_set_text_step() (16 nodes, cohesion: 0.18)

- mod
- click()
- find()
- serde_json::json
- smith_workflow::Step
- super::*
- input_text()
- process_start()
- process_start_with_args()
- set_text()
- test_click_uses_found_key()
- test_find_creates_step_with_output_key()
- test_input_text_step()
- test_process_start_step()
- test_process_start_with_args_step()
- test_set_text_step()

### Community 8 — test_decide_step_kind_name() (15 nodes, cohesion: 0.16)

- Step
- .agent()
- .agent_decide()
- .context()
- .kind_name()
- .max_steps()
- .options()
- .retry()
- .schema()
- .tools()
- .workflow()
- test_agent_step_kind_name()
- test_agent_tools_sets_tools()
- test_decide_options_are_set()
- test_decide_step_kind_name()

### Community 9 — test_think_step_kind_name() (14 nodes, cohesion: 0.15)

- step
- crate::workflow::Workflow
- pub use smith_core::RetryPolicy
- serde_json::Value
- super::*
- tracing::warn
- .agent_think()
- .args()
- .rpa()
- StepKind
- test_retry_policy_defaults()
- test_rpa_args_sets_args()
- test_rpa_step_kind_name()
- test_think_step_kind_name()

### Community 10 — WorkflowExecutor (13 nodes, cohesion: 0.15)

- executor
- async_trait::async_trait
- crate::context::WorkflowContext
- crate::error::{AgentResult, WorkflowError}
- crate::step::{Step, StepKind}
- crate::workflow::Workflow
- serde_json::Value
- smith_core::{AiHandler, ExecutionContext, ToolRegistry}
- smith_core::{ContextValue, SmithResult, Tool, ToolConfig, ToolResult}
- super::*
- tokio_util::sync::CancellationToken
- tracing::{info, warn}
- WorkflowExecutor

### Community 11 — test_decide_valid_choice() (13 nodes, cohesion: 0.26)

- .prompt()
- make_agent()
- .prompt()
- .agent_run()
- .decide()
- .prompt()
- test_agent_run_parses_json()
- test_agent_run_returns_plain_text()
- test_decide_cancelled()
- test_decide_empty_options()
- test_decide_invalid_choice()
- test_decide_trims_quotes()
- test_decide_valid_choice()

### Community 12 — read_node() (13 nodes, cohesion: 0.23)

- capture
- build_best_selector()
- capture_at_point()
- capture_focused_element()
- contains_point()
- cursor_position()
- find_deepest_at_point()
- crate::types::{BestSelector, CapturedElement, PathNode}
- uiautomation::core::UIAutomation
- uiautomation::core::{UIElement, UITreeWalker}
- uiautomation::types::ControlType
- windows::Win32::UI::WindowsAndMessaging::GetCursorPos
- read_node()

### Community 13 — GraphExecutor (12 nodes, cohesion: 0.17)

- executor
- GraphExecutor
- async_trait::async_trait
- crate::graph::FlowGraph
- crate::node::EdgeKind
- crate::node::{Node, NodeId, RetryPolicy}
- serde_json::Value
- smith_core::{AiHandler, ContextValue, Tool, ToolConfig, ToolResult}
- smith_core::{AiHandler, ExecutionContext, SmithError, SmithResult, ToolRegistry}
- super::*
- tokio_util::sync::CancellationToken
- tracing::{info, warn}

### Community 14 — test_set_and_get_variable() (12 nodes, cohesion: 0.29)

- ExecutionContext
- .default()
- .get()
- .new()
- .pop_scope()
- .push_scope()
- .set()
- test_get_returns_none_for_missing_key()
- test_new_creates_empty_scope()
- test_pop_scope_does_not_remove_global()
- test_push_scope_isolation()
- test_set_and_get_variable()

### Community 15 — test_default_is_empty() (12 nodes, cohesion: 0.17)

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
- .default()

### Community 16 — test_execute_single_rpa() (11 nodes, cohesion: 0.31)

- .new()
- make_registry()
- MockTool
- .description()
- .execute()
- .name()
- .schema()
- test_execute_cancelled()
- test_execute_linear_rpa_then_agent()
- test_execute_router_choice()
- test_execute_single_rpa()

### Community 17 — ElementSelector (11 nodes, cohesion: 0.29)

- ElementSelector
- .automation_id()
- .build_condition_with()
- .class_name()
- .control_type()
- .find_all()
- .find_first()
- .find_from_desktop()
- .name()
- .new()
- .pid()

### Community 18 — test_platform_error_display() (11 nodes, cohesion: 0.18)

- error
- std::error::Error
- super::*
- thiserror::Error
- SmithError
- test_cancelled_display()
- test_context_error_display()
- test_conversion_from_anyhow_error()
- test_element_not_found_display()
- test_invalid_params_display()
- test_platform_error_display()

### Community 19 — test_context_value_try_as_string() (11 nodes, cohesion: 0.18)

- context
- .try_as_number()
- crate::error::{SmithError, SmithResult}
- std::any::Any
- std::collections::HashMap
- std::sync::Arc
- super::*
- test_context_value_null()
- test_context_value_try_as_boolean()
- test_context_value_try_as_number()
- test_context_value_try_as_string()

### Community 20 — lib (20) (11 nodes, cohesion: 0.18)

- lib
- pub use context::WorkflowContext
- pub use crate::context::WorkflowContext
- pub use crate::error::WorkflowError
- pub use crate::step::{RetryPolicy, Step, StepKind}
- pub use crate::workflow::Workflow
- pub use error::{AgentResult, StepErrorContext, WorkflowError}
- pub use executor::WorkflowExecutor
- pub use serde_json::json
- pub use step::{RetryPolicy, Step, StepKind}
- pub use workflow::Workflow

### Community 21 — test_with_model_override() (11 nodes, cohesion: 0.29)

- provider
- super::*
- ProviderConfig
- .anthropic()
- .openai()
- .with_base_url()
- .with_model()
- test_anthropic_default_model()
- test_openai_default_model()
- test_with_base_url()
- test_with_model_override()

### Community 22 — WorkflowContext (11 nodes, cohesion: 0.24)

- context
- serde_json::Value
- smith_core::ExecutionContext
- std::collections::HashMap
- WorkflowContext
- .default()
- .elapsed_ms()
- .get_step_result()
- .new()
- .now()
- .set_step_result()

### Community 23 — SeriesRecording (9 nodes, cohesion: 0.22)

- types
- Action
- BestSelector
- Capture
- CapturedElement
- CaptureOutput
- serde::{Deserialize, Serialize}
- PathNode
- SeriesRecording

### Community 24 — adapter (9 nodes, cohesion: 0.22)

- adapter
- rig::completion::request::ToolDefinition
- rig::tool::{ToolDyn, ToolError}
- smith_core::{ExecutionContext, Tool as SmithTool, ToolConfig}
- std::future::Future
- std::pin::Pin
- std::sync::Arc
- tokio::sync::Mutex
- tokio_util::sync::CancellationToken

### Community 25 — run_graphify_build() (9 nodes, cohesion: 0.22)

- build_env_info()
- build_git_log()
- build_graph()
- collect_todos()
- format_markdown()
- load_graphify_artifacts()
- main()
- read_workspace_cargo()
- run_graphify_build()

### Community 26 — is_command_allowed() (8 nodes, cohesion: 0.25)

- process
- async_trait::async_trait
- serde_json::{Value, json}
- smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult}
- std::collections::HashSet
- std::time::Duration
- tokio_util::sync::CancellationToken
- is_command_allowed()

### Community 27 — WorkflowBuilder (8 nodes, cohesion: 0.32)

- .try_from()
- step_to_node()
- Workflow
- .new()
- WorkflowBuilder
- .build()
- .on_choice()
- .step()

### Community 28 — MockAi (28) (8 nodes, cohesion: 0.32)

- .execute()
- .execute_node()
- .execute_rpa()
- .resolve_next()
- MockAi
- .agent_run()
- .decide()
- .think()

### Community 29 — WaitTool (7 nodes, cohesion: 0.33)

- WaitTool
- .default()
- .description()
- .execute()
- .name()
- .new()
- .schema()

### Community 30 — Tool (7 nodes, cohesion: 0.29)

- tool
- async_trait::async_trait
- crate::context::ExecutionContext
- crate::error::SmithResult
- serde_json::Value
- tokio_util::sync::CancellationToken
- Tool

### Community 31 — AiHandler (7 nodes, cohesion: 0.29)

- ai
- AiHandler
- async_trait::async_trait
- crate::context::ExecutionContext
- crate::error::SmithResult
- serde_json::Value
- tokio_util::sync::CancellationToken

### Community 32 — InputTextTool (7 nodes, cohesion: 0.38)

- InputTextTool
- .default()
- .description()
- .execute()
- .name()
- .new()
- .schema()

### Community 33 — ClickTool (7 nodes, cohesion: 0.38)

- ClickTool
- .default()
- .description()
- .execute()
- .name()
- .new()
- .schema()

### Community 34 — StepErrorContext (7 nodes, cohesion: 0.29)

- error
- AgentResult
- thiserror::Error
- StepErrorContext
- .fmt()
- .source()
- WorkflowError

### Community 35 — SetTextTool (7 nodes, cohesion: 0.38)

- SetTextTool
- .default()
- .description()
- .execute()
- .name()
- .new()
- .schema()

### Community 36 — lib (7 nodes, cohesion: 0.29)

- lib
- pub use ai::AiHandler
- pub use context::{ContextValue, ExecutionContext}
- pub use error::{SmithError, SmithResult}
- pub use registry::ToolRegistry
- pub use retry::RetryPolicy
- pub use tool::{Tool, ToolConfig, ToolResult}

### Community 37 — graph (7 nodes, cohesion: 0.29)

- graph
- crate::node::{EdgeKind, RetryPolicy}
- crate::node::{Edges, Node, NodeIO, NodeId}
- serde_json::Value
- std::collections::HashMap
- super::*
- tracing::warn

### Community 38 — workflow (7 nodes, cohesion: 0.29)

- workflow
- crate::error::WorkflowError
- crate::step::Step
- smith_core::RetryPolicy
- smith_graph::{FlowGraph, FlowGraphBuilder}
- smith_graph::node::{EdgeKind, Node, NodeId}
- std::collections::HashMap

### Community 39 — find (7 nodes, cohesion: 0.29)

- find
- async_trait::async_trait
- crate::element::SafeUIElement
- crate::selector::ElementSelector
- serde_json::{Value, json}
- smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult}
- tokio_util::sync::CancellationToken

### Community 40 — SafeUIElement (7 nodes, cohesion: 0.29)

- element
- std::sync::Arc
- uiautomation::UIElement
- SafeUIElement
- .clone()
- .inner()
- .new()

### Community 41 — FindTool (7 nodes, cohesion: 0.43)

- FindTool
- .default()
- .description()
- .execute()
- .name()
- .new()
- .schema()

### Community 42 — MockAi (6 nodes, cohesion: 0.47)

- MockAi
- .agent_run()
- .decide()
- .think()
- .execute()
- .execute_step()

### Community 43 — click (6 nodes, cohesion: 0.33)

- click
- async_trait::async_trait
- crate::element::SafeUIElement
- serde_json::{Value, json}
- smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult}
- tokio_util::sync::CancellationToken

### Community 44 — TestTool (6 nodes, cohesion: 0.33)

- test_execute_unknown_tool()
- TestTool
- .description()
- .execute()
- .name()
- .schema()

### Community 45 — parse_control_type() (6 nodes, cohesion: 0.33)

- selector
- smith_core::SmithError
- uiautomation::core::{UIAutomation, UICondition, UIElement}
- uiautomation::types::{ControlType, PropertyConditionFlags, TreeScope, UIProperty}
- uiautomation::variants::Variant
- parse_control_type()

### Community 46 — ToolAdapter (6 nodes, cohesion: 0.40)

- ToolAdapter
- .call()
- .definition()
- .from_arc()
- .name()
- .new()

### Community 47 — test_register_and_get_tool() (6 nodes, cohesion: 0.53)

- test_execute_success()
- test_list_tools()
- test_new_creates_empty_registry()
- test_register_and_get_tool()
- .new()
- .register()

### Community 48 — set_text (5 nodes, cohesion: 0.40)

- set_text
- async_trait::async_trait
- serde_json::{Value, json}
- smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult}
- tokio_util::sync::CancellationToken

### Community 49 — action_stop() (5 nodes, cohesion: 0.60)

- action_sleep()
- action_start()
- action_stop()
- .execute()
- .new()

### Community 50 — .try_as_string() (5 nodes, cohesion: 0.40)

- ContextValue
- .eq()
- .try_as_boolean()
- .try_as_custom()
- .try_as_string()

### Community 51 — is_binary_extension() (5 nodes, cohesion: 0.40)

- collect_files()
- detect_language()
- is_always_excluded_dir()
- is_always_excluded_file()
- is_binary_extension()

### Community 52 — wait (5 nodes, cohesion: 0.40)

- wait
- async_trait::async_trait
- serde_json::{Value, json}
- smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult}
- tokio_util::sync::CancellationToken

### Community 53 — ToolRegistry (5 nodes, cohesion: 0.50)

- test_get_unknown_tool()
- ToolRegistry
- .execute()
- .get()
- .list_tools()

### Community 54 — test_retry_policy_defaults() (5 nodes, cohesion: 0.40)

- retry
- super::*
- RetryPolicy
- test_retry_policy_custom()
- test_retry_policy_defaults()

### Community 55 — lib (55) (5 nodes, cohesion: 0.40)

- lib
- pub use executor::GraphExecutor
- pub use graph::{FlowGraph, FlowGraphBuilder}
- pub use node::{EdgeKind, Edges, Node, NodeId, NodeIO}
- pub use smith_core::RetryPolicy

### Community 56 — ProcessTool (5 nodes, cohesion: 0.40)

- ProcessTool
- .default()
- .description()
- .name()
- .schema()

### Community 57 — main() (57) (5 nodes, cohesion: 0.40)

- main
- Cli
- Commands
- clap::{Parser, Subcommand}
- main()

### Community 58 — input_text (5 nodes, cohesion: 0.40)

- input_text
- async_trait::async_trait
- serde_json::{Value, json}
- smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult}
- tokio_util::sync::CancellationToken

### Community 59 — lib (59) (4 nodes, cohesion: 0.50)

- lib
- pub use adapter::ToolAdapter
- pub use agent::SmithAgent
- pub use provider::ProviderConfig

### Community 60 — main() (2 nodes, cohesion: 1.00)

- rpa_notepad
- main()

### Community 61 — main() (61) (2 nodes, cohesion: 1.00)

- workflow_notepad
- main()

### Community 62 — main() (62) (2 nodes, cohesion: 1.00)

- agent_notepad
- main()

### Community 63 — lib (63) (2 nodes, cohesion: 1.00)

- lib
- pub use {
    element::SafeUIElement,
    selector::ElementSelector,
    tools::{ClickTool, FindTool, InputTextTool, ProcessTool, SetTextTool, WaitTool},
}

### Community 64 — main() (64) (2 nodes, cohesion: 1.00)

- flowgraph_notepad
- main()

### Community 65 — lib (65) (1 nodes, cohesion: 1.00)

- lib

## 🕳️ Knowledge Gaps

**Isolated nodes** (1):
- lib

**Thin communities** (< 3 nodes): 6 communities

## 💰 Token Cost

| File | Tokens |
|------|--------|
| input | 0 |
| output | 0 |
| **Total** | **0** |

## ❓ Suggested Questions

1. How does 'crates_smith_core_src_registry_rs_test_get_unknown_tool' relate to 3 different communities (ToolRegistry, test_register_and_get_tool(), test_default_is_empty())?
1. How does 'crates_smith_core_src_registry_rs_test_register_and_get_tool' relate to 3 different communities (test_default_is_empty(), test_register_and_get_tool(), ToolRegistry)?
1. How does 'crates_smith_core_src_registry_rs_toolregistry' relate to 3 different communities (ToolRegistry, test_register_and_get_tool(), test_default_is_empty())?
1. How does 'crates_smith_graph_src_executor_rs' relate to 3 different communities (GraphExecutor, test_execute_single_rpa(), MockAi (28))?
1. How does 'crates_smith_core_src_registry_rs_toolregistry_register' relate to 3 different communities (TestTool, ToolRegistry, test_register_and_get_tool())?
1. How does 'crates_smith_workflow_src_executor_rs' relate to 3 different communities (test_execute_think_step(), MockAi, WorkflowExecutor)?
1. How does 'crates_smith_core_src_registry_rs_toolregistry_default' relate to 3 different communities (ToolRegistry, test_register_and_get_tool(), test_default_is_empty())?

---
_Generated by graphify-rs_
