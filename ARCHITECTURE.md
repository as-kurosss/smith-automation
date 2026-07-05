# Architecture

## Workspace structure and crate responsibilities

```
smith-automation/
├── crates/
│   ├── smith-core/          # Cross-platform core
│   │   ├── src/
│   │   │   ├── lib.rs       # Public API (flat re-exports)
│   │   │   ├── tool.rs      # Trait Tool, ToolConfig/ToolResult types
│   │   │   ├── context.rs   # ExecutionContext, ContextValue
│   │   │   ├── registry.rs  # ToolRegistry
│   │   │   └── error.rs     # SmithError, SmithResult
│   │   └── Cargo.toml
│   ├── smith-windows/       # Windows UI automation
│   │   ├── src/
│   │   │   ├── lib.rs       # Re-export under cfg(windows)
│   │   │   ├── tools/mod.rs # Tool module
│   │   │   ├── tools/       # Tool implementations
│   │   │   ├── selector.rs  # ElementSelector
│   │   │   └── element.rs   # SafeUIElement
│   │   └── Cargo.toml
│   └── smith-daemon/        # HTTP daemon
│       ├── src/
│       │   └── main.rs      # axum server for smithd
│       └── Cargo.toml
├── apps/
│   └── smith-context/       # Context gathering utility (separate)
├── docs/
│   ├── adr/                 # ADR
│   └── templates/
├── Cargo.toml               # Workspace manifest
└── ARCHITECTURE.md
```

### smith-core

The core has no platform dependencies. Contains:
- **Tool trait** — interface for all automation tools.
- **ExecutionContext** — scoped variable storage through which tools exchange data.
- **ContextValue** — type-safe representation of arbitrary values (String, Number, Boolean, List, Bytes, Custom).
- **SmithError** — error hierarchy with `thiserror`.

### smith-windows

Implementation of Windows tools via UIAutomation API. All platform-specific code is isolated behind `#[cfg(windows)]`, allowing the crate to compile on any platform.

## Tool trait and ToolRegistry

### Tool trait

Base interface for all tools:

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn schema(&self) -> Value;
    async fn execute(&self, config: ToolConfig, ctx: &mut ExecutionContext,
                     token: CancellationToken) -> SmithResult<ToolResult>;
}
```

- **Send + Sync** — tools execute in a multi-threaded Tokio runtime.
- **Stateless** — the tool does not store execution state, only configuration.
- **CancellationToken** — support for graceful shutdown and timeout.
- **ToolConfig/ToolResult** — type `serde_json::Value` (flexible transport).

### ToolRegistry

Implemented in `crates/smith-core/src/registry.rs`. Tools are registered statically (`HashMap<&str, Box<dyn Tool>>`). Provides:

- Registration of tools by name (`register`).
- Tool lookup (`get`).
- Centralized `execute` with unified error handling.
- List of available tools (`list_tools`).

Dynamic loading via libraries is deferred (see ADR-0001).

## ExecutionContext and scoped variables

`ExecutionContext` is a stack of scopes (`Vec<HashMap<String, ContextValue>>`).

```
Global scope (index 0)   ← created with new()
  └─ Local scope 1       ← push_scope()
      └─ Local scope 2   ← push_scope()
```

**Operations:**
- `set(key, value)` — write to the top (current) scope.
- `get(key)` — search from top scope to global (LIFO). Returns the first found value.
- `push_scope()` / `pop_scope()` — manage scopes for isolating nested calls.

**ContextValue** — an algebraic type for type-safe storage:

```rust
pub enum ContextValue {
    String(String), Number(f64), Boolean(bool),
    List(Vec<ContextValue>), Bytes(Vec<u8>),
    Custom(Arc<dyn Any + Send + Sync>), Null,
}
```

Methods `try_as_string()`, `try_as_number()`, `try_as_boolean()`, `try_as_custom::<T>()` return `Result` for safe extraction.

## ElementSelector approach for UI automation

UI elements are identified and retrieved via the UIAutomation API. `SafeUIElement` is a thread-safe wrapper over `UIElement`:

1. **Search** — elements are found via the UIA tree (by AutomationId, Name, ControlType, conditions).
2. **PID binding** — search can be limited to a specific process (PID) for isolation.
3. **SafeUIElement** — `Arc<UIElement>` with `unsafe impl Send + Sync` (UI Automation COM objects are free-threaded).
4. **spawn_blocking** — all mutating operations (clicks, input) execute in a dedicated thread to avoid blocking the async runtime.

```rust
// Extract from context and execute
let wrapper = value.try_as_custom::<SafeUIElement>()?;
let element_clone = wrapper.clone();
tokio::task::spawn_blocking(move || {
    element_clone.inner().click()
}).await??;
```

## cfg(windows) strategy for cross-platform code

Platform isolation is implemented at two levels:

### 1. Module level (in lib.rs)

```rust
// smith-windows/src/lib.rs
#[cfg(windows)]
pub mod element;
#[cfg(windows)]
pub use element::SafeUIElement;
```

On non-Windows platforms, these modules and types do not exist — the code does not compile.

### 2. Dependencies (in Cargo.toml)

```toml
[target.'cfg(windows)'.dependencies]
uiautomation = "0.25.0"
```

The `uiautomation` library (and transitive dependencies via `windows`) are only loaded when building for Windows.

### smith-daemon

The HTTP server `smithd` (`crates/smith-daemon`) provides remote access to tools:

- Runs on Windows and registers all `smith-windows` tools.
- Listens on a configurable host/port (`--host`, `--port`, default `127.0.0.1:8742`).
- Endpoints: `POST /execute`, `GET /tools`, `GET /health`, `POST /reset`.
- Allows controlling Windows UI from WSL or another HTTP client.

### 3. Future plans

- `smith-core` remains fully platform-independent.
- For Linux: a `smith-linux` crate (X11/Wayland via AT-SPI).
- For macOS: a `smith-macos` crate (Accessibility API).
- Backend selection via feature flags in `smith-core` or through dynamic registry.
