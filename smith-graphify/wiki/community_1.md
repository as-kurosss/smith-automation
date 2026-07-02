# Community 1: tools_handler()

**Members:** 22

## Nodes

- **main** (`crates_smith_daemon_src_main_rs`, File, degree: 21)
- **AppState** (`crates_smith_daemon_src_main_rs_appstate`, Struct, degree: 1)
- **classify_error()** (`crates_smith_daemon_src_main_rs_classify_error`, Function, degree: 2)
- **execute_handler()** (`crates_smith_daemon_src_main_rs_execute_handler`, Function, degree: 2)
- **ExecuteRequest** (`crates_smith_daemon_src_main_rs_executerequest`, Struct, degree: 1)
- **ExecuteResponse** (`crates_smith_daemon_src_main_rs_executeresponse`, Struct, degree: 1)
- **health_handler()** (`crates_smith_daemon_src_main_rs_health_handler`, Function, degree: 1)
- **axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
}** (`crates_smith_daemon_src_main_rs_import_axum_router_extract_state_http_statuscode_response_intoresponse_routing_get_post_json`, Module, degree: 1)
- **serde::{Deserialize, Serialize}** (`crates_smith_daemon_src_main_rs_import_serde_deserialize_serialize`, Module, degree: 1)
- **serde_json::{json, Value}** (`crates_smith_daemon_src_main_rs_import_serde_json_json_value`, Module, degree: 1)
- **smith_core::{ExecutionContext, SmithError, ToolRegistry}** (`crates_smith_daemon_src_main_rs_import_smith_core_executioncontext_smitherror_toolregistry`, Module, degree: 1)
- **std::net::SocketAddr** (`crates_smith_daemon_src_main_rs_import_std_net_socketaddr`, Module, degree: 1)
- **std::sync::Arc** (`crates_smith_daemon_src_main_rs_import_std_sync_arc`, Module, degree: 1)
- **tokio::sync::Mutex** (`crates_smith_daemon_src_main_rs_import_tokio_sync_mutex`, Module, degree: 1)
- **tokio_util::sync::CancellationToken** (`crates_smith_daemon_src_main_rs_import_tokio_util_sync_cancellationtoken`, Module, degree: 1)
- **tracing::{info, warn}** (`crates_smith_daemon_src_main_rs_import_tracing_info_warn`, Module, degree: 1)
- **main()** (`crates_smith_daemon_src_main_rs_main`, Function, degree: 4)
- **parse_args()** (`crates_smith_daemon_src_main_rs_parse_args`, Function, degree: 2)
- **register_windows_tools()** (`crates_smith_daemon_src_main_rs_register_windows_tools`, Function, degree: 2)
- **reset_handler()** (`crates_smith_daemon_src_main_rs_reset_handler`, Function, degree: 1)
- **shutdown_signal()** (`crates_smith_daemon_src_main_rs_shutdown_signal`, Function, degree: 2)
- **tools_handler()** (`crates_smith_daemon_src_main_rs_tools_handler`, Function, degree: 1)

## Relationships

- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_import_std_net_socketaddr (imports)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_import_std_sync_arc (imports)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_import_axum_router_extract_state_http_statuscode_response_intoresponse_routing_get_post_json (imports)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_import_serde_deserialize_serialize (imports)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_import_serde_json_json_value (imports)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_import_smith_core_executioncontext_smitherror_toolregistry (imports)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_import_tokio_sync_mutex (imports)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_import_tokio_util_sync_cancellationtoken (imports)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_import_tracing_info_warn (imports)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_appstate (defines)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_executerequest (defines)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_executeresponse (defines)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_register_windows_tools (defines)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_execute_handler (defines)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_tools_handler (defines)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_health_handler (defines)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_reset_handler (defines)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_classify_error (defines)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_parse_args (defines)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_main (defines)
- crates_smith_daemon_src_main_rs → crates_smith_daemon_src_main_rs_shutdown_signal (defines)
- crates_smith_daemon_src_main_rs_execute_handler → crates_smith_daemon_src_main_rs_classify_error (calls)
- crates_smith_daemon_src_main_rs_main → crates_smith_daemon_src_main_rs_register_windows_tools (calls)
- crates_smith_daemon_src_main_rs_main → crates_smith_daemon_src_main_rs_parse_args (calls)
- crates_smith_daemon_src_main_rs_main → crates_smith_daemon_src_main_rs_shutdown_signal (calls)

