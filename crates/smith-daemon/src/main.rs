//! smithd — HTTP daemon for Windows UI automation.
//!
//! Запускает HTTP-сервер, регистрирует все инструменты `smith-windows`
//! в `ToolRegistry` и предоставляет REST API для их выполнения.
//!
//! # API
//!
//! - `POST /execute` — выполнить инструмент
//! - `GET  /tools`   — список зарегистрированных инструментов
//! - `GET  /health`  — проверка состояния
//! - `POST /reset`   — сбросить `ExecutionContext`
//!
//! # Использование
//!
//! ```bash
//! # Запуск на Windows (порт по умолчанию 8742)
//! smithd
//!
//! # С указанием порта
//! smithd --port 8080
//!
//! # Доступ из WSL / другой машины в локальной сети
//! smithd --host 0.0.0.0 --port 8742
//!
//! # Пример запроса из WSL / любого клиента
//! curl -X POST localhost:8742/execute \
//!   -H 'Content-Type: application/json' \
//!   -d '{"tool":"windows.process","config":{"action":"start","command":"notepad.exe"}}'
//! ```

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, ToolRegistry};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

struct AppState {
    registry: ToolRegistry,
    ctx: ExecutionContext,
}

type SharedState = Arc<RwLock<AppState>>;

// ---------------------------------------------------------------------------
// Request / Response типы
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct ExecuteRequest {
    /// Имя инструмента (например, "windows.process", "windows.find")
    tool: String,
    /// JSON-конфигурация, специфичная для инструмента
    #[serde(default)]
    config: Value,
}

#[derive(Serialize)]
struct ExecuteResponse {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_type: Option<String>,
}

// ---------------------------------------------------------------------------
// Регистрация инструментов (Windows)
// ---------------------------------------------------------------------------

/// Регистрирует все инструменты Windows UI automation.
#[cfg(windows)]
fn register_windows_tools(registry: &mut ToolRegistry) {
    registry.register(smith_windows::ClickTool::new());
    registry.register(smith_windows::FindTool::new());
    registry.register(smith_windows::InputTextTool::new());
    registry.register(smith_windows::ProcessTool::new());
    registry.register(smith_windows::SetTextTool::new());
    info!("Registered 5 Windows UI automation tools");
}

/// Заглушка для не-Windows платформ.
#[cfg(not(windows))]
fn register_windows_tools(_registry: &mut ToolRegistry) {
    info!("Running on non-Windows — Windows UI tools are not available");
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// `POST /execute` — выполнить инструмент.
///
/// Блокировка удерживается только на время клонирования необходимых данных
/// и вызова execute, чтобы не блокировать другие запросы (health, tools).
async fn execute_handler(
    State(state): State<SharedState>,
    Json(req): Json<ExecuteRequest>,
) -> impl IntoResponse {
    let token = CancellationToken::new();

    // Захватываем write-lock только на время вызова execute.
    // ExecutionContext требует эксклюзивного доступа для мутации скоупов.
    let result = {
        let mut app = state.write().await;
        let AppState { registry, ctx } = &mut *app;
        registry.execute(&req.tool, req.config, ctx, token).await
    };

    match result {
        Ok(result) => {
            info!(tool = %req.tool, "execute OK");
            (
                StatusCode::OK,
                Json(ExecuteResponse {
                    status: "ok".into(),
                    result: Some(result),
                    error: None,
                    error_type: None,
                }),
            )
        }
        Err(err) => {
            let (status, error_type) = classify_error(&err);
            warn!(tool = %req.tool, error = %err, "execute failed");
            (
                status,
                Json(ExecuteResponse {
                    status: "error".into(),
                    result: None,
                    error: Some(err.to_string()),
                    error_type: Some(error_type),
                }),
            )
        }
    }
}

/// `GET /tools` — список доступных инструментов.
///
/// Использует read-lock — не блокирует другие read-only запросы.
async fn tools_handler(State(state): State<SharedState>) -> impl IntoResponse {
    let app = state.read().await;
    let tools = app.registry.list_tools();
    Json(json!({ "tools": tools, "count": tools.len() }))
}

/// `GET /health` — проверка состояния.
async fn health_handler() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "smithd",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// `POST /reset` — сбросить `ExecutionContext` (новая сессия).
async fn reset_handler(State(state): State<SharedState>) -> impl IntoResponse {
    let mut app = state.write().await;
    app.ctx = ExecutionContext::new();
    info!("ExecutionContext reset");
    Json(json!({ "status": "ok", "message": "Context reset" }))
}

// ---------------------------------------------------------------------------
// Утилиты
// ---------------------------------------------------------------------------

/// Преобразует `SmithError` в HTTP-статус и строку-идентификатор типа ошибки.
fn classify_error(err: &SmithError) -> (StatusCode, String) {
    match err {
        SmithError::InvalidParams(_)
        | SmithError::ContextError(_)
        | SmithError::ElementNotFound => (StatusCode::BAD_REQUEST, "BadRequest".into()),
        SmithError::PlatformError { .. } => {
            (StatusCode::INTERNAL_SERVER_ERROR, "PlatformError".into())
        }
        SmithError::Cancelled => (StatusCode::BAD_REQUEST, "Cancelled".into()),
        SmithError::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, "InternalError".into()),
    }
}

/// Аргументы командной строки для `smithd`.
#[derive(Parser, Debug)]
#[command(name = "smithd", about = "HTTP daemon for Windows UI automation")]
struct Args {
    /// Адрес для привязки (по умолчанию 127.0.0.1)
    #[arg(long, default_value = "127.0.0.1", value_parser = clap::value_parser!(std::net::IpAddr))]
    host: std::net::IpAddr,

    /// Порт для привязки (по умолчанию 8742)
    #[arg(long, default_value_t = 8742)]
    port: u16,
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    // Инициализация логгера
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    // Создаём реестр и регистрируем инструменты
    let mut registry = ToolRegistry::new();
    register_windows_tools(&mut registry);

    let tools = registry.list_tools();
    info!(count = tools.len(), tools = ?tools, "Tool registry initialized");

    // Состояние приложения
    let state: SharedState = Arc::new(RwLock::new(AppState {
        registry,
        ctx: ExecutionContext::new(),
    }));

    // Маршрутизация
    let app = Router::new()
        .route("/execute", post(execute_handler))
        .route("/tools", get(tools_handler))
        .route("/health", get(health_handler))
        .route("/reset", post(reset_handler))
        .with_state(state);

    // Парсинг аргументов и запуск
    let args = Args::parse();
    let addr = SocketAddr::new(args.host, args.port);
    info!("Starting smithd on {addr}");

    // Graceful shutdown по Ctrl+C
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap_or_else(|e| {
        error!("Failed to bind TCP listener on {addr}: {e}");
        std::process::exit(1);
    });

    if let Err(e) = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        error!("Server error: {e}");
        std::process::exit(1);
    }
}

/// Ожидает сигнал Ctrl+C и запускает graceful shutdown.
async fn shutdown_signal() {
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            info!("Shutdown signal received, stopping server");
        }
        Err(e) => {
            warn!("Failed to install Ctrl+C handler: {e}");
        }
    }
}
