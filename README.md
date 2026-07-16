# Smith

**Smith** — A Rust framework for building AI agents with Windows UI automation capabilities.

Smith provides a layered architecture for creating autonomous agents — from simple tool-calling loops to multi-agent orchestration with workflow graphs, memory, and undo support — all with first-class Windows UI Automation (UIA) integration.

## Goals

- Type-safe, async-first API for UI automation in Rust
- Windows UI Automation (UIA) — primary engine
- Cancellation, timeouts, scoped variables
- Plugin architecture via the `Tool` trait

## Quick Start

### 1. Add dependencies

```toml
[dependencies]
smith-agent = { git = "https://github.com/as-kurosss/smith.git" }
smith-providers = { git = "https://github.com/as-kurosss/smith.git" }
tokio = { version = "1", features = ["full"] }
```

### 2. Create an agent

A minimal agent that answers questions via an LLM:

```rust
use smith_agent::agent::{Agent, AgentConfig};
use smith_agent::loops::{Context, CycleType, Loop, LoopId, StopCondition};
use smith_providers::OpenAiClient;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an LLM client from environment variables:
    //   OPENAI_API_KEY, OPENAI_API_URL, OPENAI_MODEL
    let client = OpenAiClient::from_env("gpt-4o")?;

    // Configure the agent
    let config = AgentConfig {
        model: "gpt-4o".into(),
        model_id: None,
        system_prompt: "You are a concise assistant.".into(),
        temperature: Some(0.5),
        max_tokens: Some(1024),
        scroll_strategy: None,
        protect_active_turn: false,
        tool_result_cap: None,
    };

    let agent = Agent::new(client, config);

    // Build the execution context
    let ctx = Context::new(
        LoopId::new(),
        CycleType::Turn,
        StopCondition::new(Some(25), Some(Duration::from_secs(120))),
        "What is the capital of France?".to_string(),
    );

    let mut state = Vec::new();
    let result = agent.execute(ctx, &mut state).await;

    match result.output {
        Some(answer) => println!("Answer: {answer}"),
        None => eprintln!("Agent failed: {:?}", result.status),
    }

    Ok(())
}
```

Run it:

```bash
cargo run --example simple_agent
```

### 3. Add tools

Bridge domain tools (e.g. Windows UI Automation) into the agent via `SmithToolAdapter`:

```rust
use smith_agent::tools::ToolSet;
use smith_agent::tools::SmithToolAdapter;
use smith_windows::ClickTool;

let mut toolset = ToolSet::new();
toolset.register(SmithToolAdapter::from(ClickTool::new()));
agent.set_tools(toolset);
```

See `rpa_with_agent` for a complete working example:

```bash
cargo run --example rpa_with_agent
```

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
| **smith-ai** | Minimal HTTP-based LLM client for Q&A, Think, and Decide operations |
| **smith-workflow** | FlowGraph — graph execution engine with error handling and routing |
| **smith-providers** | LLM provider adapters (Anthropic, OpenAI, Gemini) |
| **smith-mcp** | MCP server and protocol implementation |
| **smith-agent** | Agent lifecycle, tool orchestration, and session management (`SmithToolAdapter`) |
| **smith-observe** | OpenTelemetry tracing, logging, and metrics |
| **smith-server** | HTTP API server for remote agent control |
| **smith-cli** | CLI entrypoint, configuration, and argument parsing |
| **smith-examples** | Example apps covering all layers |
| **selector-capture** | UI element selector capture utility |

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
