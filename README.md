# Smith

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

## Architectural Layers

Smith is organised in layers. Each layer builds on the one below it, letting users
start at the level that fits their needs.

```text
┌─────────────────────────────────────────────────┐
│  Apps: CLI, Server, Examples                    │
│  (entrypoints, configuration, HTTP API)         │
├─────────────────────────────────────────────────┤
│  Agent: Agent lifecycle, orchestration, tools   │
│  (session management, SmithToolAdapter)         │
├─────────────────────────────────────────────────┤
│  Workflow: FlowGraph, routing, error handling   │
│  Providers: LLM adapters (Anthropic, OpenAI)    │
│  MCP: Model Context Protocol integration        │
├─────────────────────────────────────────────────┤
│  AI: Rig-based LLM agent, RPA templates         │
│  Windows: UI Automation tools (Click, Find,     │
│           InputText, Process, SetText, Wait)    │
├─────────────────────────────────────────────────┤
│  Core: Tool trait, ExecutionContext, SmithError │
└─────────────────────────────────────────────────┘
```

| Layer | Crates | Purpose |
|-------|--------|---------|
| **Core** | `smith-core` | `Tool` trait, `ExecutionContext` with scoped variables, `SmithError` — the foundation every tool depends on |
| **Domain** | `smith-windows`, `smith-rpa`, `smith-ai` | Concrete automation tools (UIA), type-safe RPA node builders, LLM integration via Rig |
| **Integration** | `smith-workflow`, `smith-providers`, `smith-mcp` | Graph execution engine, LLM provider adapters (Anthropic, OpenAI), MCP protocol server |
| **Agent** | `smith-agent` | Agent lifecycle, session management, tool orchestration (`SmithToolAdapter` bridges domain tools into agent `ToolSet`) |
| **App** | `smith-cli`, `smith-server`, `smith-observe` | CLI entrypoint, HTTP API server, OpenTelemetry tracing/metrics |

## Workspace

```text
crates/
├── core/
│   └── smith-core/         # Tool trait, ExecutionContext, SmithError
├── domain/
│   ├── smith-windows/      # Windows UI Automation tools
│   ├── smith-rpa/          # Type-safe Node::Rpa constructors
│   └── smith-ai/           # Rig-based LLM agent
├── integration/
│   ├── smith-workflow/     # FlowGraph execution engine
│   ├── smith-providers/    # LLM provider adapters
│   └── smith-mcp/          # MCP protocol integration
├── agent/
│   └── smith-agent/        # Agent lifecycle and orchestration
└── app/
    ├── smith-observe/      # Observability (tracing, metrics)
    ├── smith-server/       # HTTP server for remote control
    └── smith-cli/          # CLI entrypoint
apps/
├── smith-examples/         # Example applications
└── selector-capture/       # UI selector capture utility
```

| Crate | Description |
|-------|-------------|
| **smith-core** | `Tool` trait, `ExecutionContext` with scoped variables, `SmithError` |
| **smith-windows** | UI Automation tools (`ClickTool`, `FindTool`, `InputTextTool`, `ProcessTool`, `SetTextTool`, `WaitTool`) |
| **smith-rpa** | Type-safe `Node::Rpa` constructors by domain (windows) |
| **smith-ai** | Rig-based LLM agent wrapper (`SmithAgent`) |
| **smith-workflow** | FlowGraph — graph execution engine with error handling and routing |
| **smith-providers** | LLM provider adapters (Anthropic, OpenAI, etc.) |
| **smith-mcp** | MCP server and protocol implementation |
| **smith-agent** | Agent lifecycle, tool orchestration, and session management (`SmithToolAdapter`) |
| **smith-observe** | OpenTelemetry tracing, logging, and metrics |
| **smith-server** | HTTP API server for remote agent control |
| **smith-cli** | CLI entrypoint, configuration, and argument parsing |
| **smith-examples** | Example apps covering all layers |
| **selector-capture** | UI element selector capture utility |

## Quick Start

### Level 1 — Pure RPA (no AI)

Run a tool directly via `ToolRegistry`:

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

### Level 2 — RPA + AI

Use `smith-ai` to let an LLM decide which tool to call:

```rust
use smith_ai::SmithAgent;
use smith_core::ToolRegistry;

let mut registry = ToolRegistry::new();
registry.register(Tool::new());
let agent = SmithAgent::new(provider, registry);
let response = agent.prompt("Click the login button").await?;
```

### Level 3 — Agent orchestration

Bridge domain tools into the agent `ToolSet` via `SmithToolAdapter`:

```rust
use smith_agent::tools::SmithToolAdapter;
use smith_windows::ClickTool;

let adapter = SmithToolAdapter::from(ClickTool::new());
toolset.register(adapter);
```

### Level 4 — FlowGraph

Define a multi-step workflow with conditional routing:

```rust
use smith_workflow::FlowGraph;

let mut graph = FlowGraph::new();
graph.add_node("classify", Node::ai_prompt(classify_prompt));
graph.add_node("click", Node::tool("windows.click"));
graph.add_node("extract", Node::tool("windows.get_text"));
graph.add_edge("classify", "click", "click");
graph.add_edge("classify", "extract", "extract");
```

## Examples

Run any example with:

```bash
cargo run --example <name>
```

| Example | Layer | Description |
|---------|-------|-------------|
| `rpa_basic` | Core | Pure RPA tool execution (Echo + Calculator tools via `ToolRegistry`) |
| `rpa_with_ai` | Domain + AI | RPA with AI-assisted tool selection and error analysis |
| `rpa_with_agent` | Agent | `SmithToolAdapter` wrapping smith-core tools for agent `ToolSet` |
| `agent_graph` | Agent + Workflow | FlowGraph with Think/Router nodes and conditional routing |
| `smith_bridge` | All layers | End-to-end flow combining core tools, AI, adapter, and workflow |
| `simple_agent` | Agent | Minimal agent setup with tool calling |
| `agent_notepad` | Agent + Windows | Agent that controls Notepad via UIA tools |
| `flowgraph_notepad` | Workflow + Windows | FlowGraph-based Notepad automation |
| `multi_agent` | Agent | Multi-agent coordination with message passing |
| `approval_workflow` | Agent + Workflow | Human-in-the-loop approval before tool execution |
| `persistent_graph` | Workflow | FlowGraph with persisted state across runs |
| `streaming` | Agent | Streaming LLM responses from the agent |

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
