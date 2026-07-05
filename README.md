# smith-automation

**Smith** — A Rust library for programmatic UI automation on Windows.

```rust
use smith_core::{ExecutionContext, ToolRegistry};
use smith_windows::ClickTool;

let mut ctx = ExecutionContext::new();
let mut registry = ToolRegistry::new();
registry.register(ClickTool::new());

let result = registry
    .execute("windows.click", config, &mut ctx, token)
    .await?;
```

## Goals

- Type-safe, async-first API for UI automation in Rust
- Windows UI Automation (UIA) — primary engine
- Cancellation, timeouts, scoped variables
- Plugin architecture via the `Tool` trait

## Workspace

```text
crates/
├── smith-core/         # Core: Tool, ExecutionContext, SmithError
├── smith-windows/      # Windows UI Automation tools
├── smith-workflow/     # Workflow engine (linear steps)
├── smith-rpa/          # Type-safe Step constructors
├── smith-ai/           # Rig-based LLM agent
└── smith-graph/        # FlowGraph execution engine
apps/
├── smith-examples/     # Example applications
└── selector-capture/   # UI selector capture utility
```

| Crate | Description |
|-------|-------------|
| **smith-core** | `Tool` trait, `ExecutionContext` with scoped variables, `SmithError` |
| **smith-windows** | UI Automation tools (`ClickTool`, `FindTool`, `InputTextTool`, `ProcessTool`, `SetTextTool`, `WaitTool`) |
| **smith-workflow** | Workflow engine with linear RPA/AI step execution |
| **smith-rpa** | Type-safe Step constructors by domain (windows) |
| **smith-ai** | Rig-based LLM agent wrapper (`SmithAgent`) |
| **smith-graph** | FlowGraph — graph execution engine with error handling and routing |
| **smith-examples** | Example apps: pure RPA, AI agent, FlowGraph, combined workflow |
| **selector-capture** | UI element selector capture utility |

## Build

```bash
cargo build                    # everything
cargo build -p smith-core      # core only
cargo build -p smith-windows   # Windows tools
```

## Development

```bash
cargo check
cargo clippy -- -D warnings
cargo test
cargo fmt --check
```

## License

MIT. See [LICENSE](LICENSE).
