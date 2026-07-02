# Architecture

## Workspace structure and crate responsibilities

```
smith-automation/
├── crates/
│   ├── smith-core/          # Кросс-платформенное ядро
│   │   ├── src/
│   │   │   ├── lib.rs       # Публичный API (flat re-exports)
│   │   │   ├── tool.rs      # Trait Tool, типы ToolConfig/ToolResult
│   │   │   ├── context.rs   # ExecutionContext, ContextValue
│   │   │   ├── registry.rs  # ToolRegistry
│   │   │   └── error.rs     # SmithError, SmithResult
│   │   └── Cargo.toml
│   ├── smith-windows/       # Windows UI automation
│   │   ├── src/
│   │   │   ├── lib.rs       # Реэкспорт под cfg(windows)
│   │   │   ├── tools/mod.rs # Модуль инструментов
│   │   │   ├── tools/       # Реализации инструментов
│   │   │   ├── selector.rs  # ElementSelector
│   │   │   └── element.rs   # SafeUIElement
│   │   └── Cargo.toml
│   └── smith-daemon/        # HTTP daemon
│       ├── src/
│       │   └── main.rs      # axum-сервер smithd
│       └── Cargo.toml
├── apps/
│   └── smith-context/       # Утилита сбора контекста (отдельная)
├── docs/
│   ├── adr/                 # ADR
│   └── templates/
├── Cargo.toml               # Workspace manifest
└── ARCHITECTURE.md
```

### smith-core

Ядро не имеет платформенных зависимостей. Содержит:
- **Tool trait** — интерфейс для всех инструментов автоматизации.
- **ExecutionContext** — scoped-хранилище переменных, через которое инструменты обмениваются данными.
- **ContextValue** — типобезопасное представление произвольных значений (String, Number, Boolean, List, Bytes, Custom).
- **SmithError** — иерархия ошибок с `thiserror`.

### smith-windows

Реализация инструментов для Windows через UIAutomation API. Весь платформенный код изолирован за `#[cfg(windows)]`, что позволяет компилировать крейт на любой платформе.

## Tool trait and ToolRegistry

### Tool trait

Базовый интерфейс для всех инструментов:

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

- **Send + Sync** — инструменты исполняются в многопоточном Tokio runtime.
- **Stateless** — инструмент не хранит состояние выполнения, только конфигурацию.
- **CancellationToken** — поддержка graceful shutdown и timeout.
- **ToolConfig/ToolResult** — тип `serde_json::Value` (гибкий транспорт).

### ToolRegistry

Реализован в `crates/smith-core/src/registry.rs`. Инструменты регистрируются статически (`HashMap<&str, Box<dyn Tool>>`). Предоставляет:

- Регистрацию инструментов по имени (`register`).
- Поиск инструмента (`get`).
- Централизованный `execute` с единой обработкой ошибок.
- Список доступных инструментов (`list_tools`).

Динамическая загрузка через библиотеки отложена (см. ADR-0001).

## ExecutionContext and scoped variables

`ExecutionContext` — это стек областей видимости (`Vec<HashMap<String, ContextValue>>`).

```
Global scope (index 0)   ← создаётся при new()
  └─ Local scope 1       ← push_scope()
      └─ Local scope 2   ← push_scope()
```

**Операции:**
- `set(key, value)` — запись в верхнюю (текущую) область.
- `get(key)` — поиск от верхней области к глобальной (LIFO). Возвращает первое найденное значение.
- `push_scope()` / `pop_scope()` — управление областями для изоляции вложенных вызовов.

**ContextValue** — алгебраический тип для типобезопасного хранения:

```rust
pub enum ContextValue {
    String(String), Number(f64), Boolean(bool),
    List(Vec<ContextValue>), Bytes(Vec<u8>),
    Custom(Arc<dyn Any + Send + Sync>), Null,
}
```

Методы `try_as_string()`, `try_as_number()`, `try_as_boolean()`, `try_as_custom::<T>()` возвращают `Result` для безопасного извлечения.

## ElementSelector approach for UI automation

Элементы UI идентифицируются и извлекаются через UIAutomation API. `SafeUIElement` — потокобезопасная обёртка над `UIElement`:

1. **Поиск** — элементы находятся через дерево UIA (по AutomationId, Name, ControlType, условиям).
2. **PID-привязка** — поиск может быть ограничен конкретным процессом (PID) для изоляции.
3. **SafeUIElement** — `Arc<UIElement>` с `unsafe impl Send + Sync` (UI Automation COM-объекты free-threaded).
4. **spawn_blocking** — все мутирующие операции (клики, ввод) выполняются в выделенном потоке, чтобы не блокировать async runtime.

```rust
// Извлечение из контекста и выполнение
let wrapper = value.try_as_custom::<SafeUIElement>()?;
let element_clone = wrapper.clone();
tokio::task::spawn_blocking(move || {
    element_clone.inner().click()
}).await??;
```

## cfg(windows) strategy for cross-platform code

Платформенная изоляция реализована на двух уровнях:

### 1. Модульный уровень (в lib.rs)

```rust
// smith-windows/src/lib.rs
#[cfg(windows)]
pub mod element;
#[cfg(windows)]
pub use element::SafeUIElement;
```

На не-Windows платформах эти модули и типы не существуют — код не компилируется.

### 2. Зависимости (в Cargo.toml)

```toml
[target.'cfg(windows)'.dependencies]
uiautomation = "0.25.0"
```

Библиотека `uiautomation` (и транзитивные зависимости через `windows`) загружаются только при сборке под Windows.

### smith-daemon

HTTP-сервер `smithd` (`crates/smith-daemon`) предоставляет удалённый доступ к инструментам:

- Запускается на Windows и регистрирует все `smith-windows` инструменты.
- Слушает на настраиваемом host/port (`--host`, `--port`, по умолчанию `127.0.0.1:8742`).
- Endpoints: `POST /execute`, `GET /tools`, `GET /health`, `POST /reset`.
- Позволяет управлять Windows UI из WSL или другого клиента по HTTP.

### 3. Планы на будущее

- `smith-core` остаётся полностью платформонезависимым.
- Для Linux: крейт `smith-linux` (X11/Wayland через AT-SPI).
- Для macOS: крейт `smith-macos` (Accessibility API).
- Выбор backend'а через feature flags в `smith-core` или через динамический registry.
