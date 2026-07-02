# smith-automation

**Smith** — платформа для программной автоматизации UI на Windows. Позволяет описывать сценарии взаимодействия с графическим интерфейсом через декларативные конфигурации и выполнять их в изолированной среде Runtime.

## Goals

- Предоставить type-safe, async-first API для UI automation на Rust.
- Поддерживать платформу Windows (UI Automation) с возможностью расширения на Linux/macOS.
- Гарантировать безопасное выполнение автоматизации через cancellation, timeouts и scoped-переменные.
- Обеспечить расширяемость через plugin-архитектуру на трейтах.

## Workspace structure

Проект организован как Cargo workspace с тремя крейтами:

```text
smith-automation/
├── crates/
│   ├── smith-core/          # Ядро: Tool trait, ExecutionContext, ошибки
│   ├── smith-windows/       # Windows UI automation (cfg(windows))
│   └── smith-daemon/        # HTTP daemon для удалённого выполнения инструментов
├── apps/
│   └── smith-context/       # Утилита сбора контекста (отдельная, вне автоматизации)
├── docs/
│   ├── adr/                 # Architecture Decision Records
│   └── templates/           # Шаблоны документов
└── Cargo.toml               # Workspace manifest
```

### crates

| Crate | Description |
|-------|-------------|
| **smith-core** | Базовые абстракции: трейт `Tool`, `ExecutionContext` со scoped-переменными, `SmithError`, `ContextValue`. Не зависит от платформы. |
| **smith-windows** | Инструменты для Windows UI Automation: `ClickTool`, `FindTool`, `InputTextTool`, `ProcessTool`, `SetTextTool`. Весь Windows-специфичный код изолирован за `#[cfg(windows)]`. |
| **smith-daemon** | HTTP-сервер (`smithd`), который регистрирует инструменты и предоставляет REST API для их удалённого выполнения. Запускается на Windows, вызывается из WSL или другого клиента. |

## Build

```bash
# Сборка всего workspace
cargo build

# Только кросс-платформенное ядро
cargo build -p smith-core

# Windows-инструменты (только на Windows)
cargo build -p smith-windows

# HTTP daemon (только на Windows, т.к. регистрирует Windows-инструменты)
cargo build -p smith-daemon
```

## HTTP daemon (smithd)

`smithd` запускается на Windows и предоставляет REST API для удалённого выполнения инструментов из WSL или другого клиента.

```bash
# На Windows:
smithd --host 0.0.0.0 --port 8742

# Из WSL (если localhost forwarding не работает, используй IP Windows-хоста):
WIN_IP=$(ip route show | awk '/default/ {print $3}')
curl -X POST "${WIN_IP}:8742/execute" \
  -H 'Content-Type: application/json' \
  -d '{"tool":"windows.process","config":{"action":"start","command":"notepad.exe"}}'
```

### Пример: полный сценарий с Notepad

WSL2 обычно не может обратиться к Windows по `localhost`, поэтому используем IP Windows-хоста (шлюз по умолчанию):

```bash
# IP Windows-хоста из WSL2
WIN_IP=$(ip route show | awk '/default/ {print $3}')

# 1. Запуск Notepad
START=$(curl -s -X POST "${WIN_IP}:8742/execute" \
  -H 'Content-Type: application/json' \
  -d '{"tool":"windows.process","config":{"action":"start","command":"notepad.exe"}}')
PID=$(echo "$START" | python3 -c 'import sys,json; print(json.load(sys.stdin)["result"]["pid"])')

# 2. Дождаться открытия окна
sleep 2

# 3. Найти текстовое поле по PID и control_type
curl -s -X POST "${WIN_IP}:8742/execute" \
  -H 'Content-Type: application/json' \
  -d "{\"tool\":\"windows.find\",\"config\":{\"control_type\":\"Edit\",\"pid\":$PID,\"output_key\":\"editor\"}}"

# 4. Ввести текст
curl -s -X POST "${WIN_IP}:8742/execute" \
  -H 'Content-Type: application/json' \
  -d '{"tool":"windows.input_text","config":{"element_key":"editor","text":"Hello from WSL!"}}'
```

Готовый скрипт: `crates/smith-daemon/examples/notepad_scenario.sh`.

> **Безопасность:** `smithd` может запускать процессы и управлять UI. По умолчанию он слушает только `127.0.0.1`. Используйте `--host 0.0.0.0` только в доверенной локальной сети.

## Development

```bash
# Проверка типов
cargo check

# Линтер
cargo clippy -- -D warnings

# Тесты
cargo test

# Форматирование
cargo fmt --check
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
