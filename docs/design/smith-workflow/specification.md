## 📐 Specification: smith-workflow | smith

**🎯 Purpose:** Combine deterministic RPA tools (domains `windows`, `browser`, etc.) with an LLM agent (`ai`) into a single workflow engine (`agent`), where each step explicitly indicates who is responsible for it — deterministic code or LLM.

---

**📦 Crate structure:**

```
smith/
├── crates/
│   ├── core/smith-core/        # Tool trait, ToolRegistry, ExecutionContext (as-is)
│   ├── domain/
│   │   ├── smith-rpa/          # RPA tool library by domain
│   │   │   ├── src/
│   │   │   │   ├── lib.rs
│   │   │   │   ├── windows/    # Click, Find, InputText, SetText, Process
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── click.rs
│   │   │   │   │   ├── find.rs
│   │   │   │   │   ├── input_text.rs
│   │   │   │   │   ├── set_text.rs
│   │   │   │   │   └── process.rs
│   │   │   │   ├── browser/    # (placeholder for future)
│   │   │   │   │   └── mod.rs
│   │   │   │   └── excel/      # (placeholder for future)
│   │   │   │       └── mod.rs
│   │   │   └── Cargo.toml
│   │   ├── smith-ai/           # Rig-based LLM agent
│   │   │   ├── src/
│   │   │   │   ├── lib.rs
│   │   │   │   ├── adapter.rs  # smith-core → rig::tool::Tool
│   │   │   │   ├── agent.rs    # SmithAgent — wrapper over Rig Agent
│   │   │   │   └── provider.rs
│   │   │   └── Cargo.toml
│   │   └── (future: smith-browser, smith-excel, …)
│   ├── integration/
│   │   ├── smith-workflow/     # Workflow engine with steps
│   │   │   ├── src/
│   │   │   │   ├── lib.rs
│   │   │   │   ├── workflow.rs # Workflow, Step
│   │   │   │   ├── step.rs     # StepKind
│   │   │   │   ├── context.rs  # WorkflowContext
│   │   │   │   ├── executor.rs # WorkflowExecutor
│   │   │   │   └── error.rs    # WorkflowError
│   │   │   └── Cargo.toml
│   │   ├── smith-providers/    # LLM provider adapters
│   │   └── smith-mcp/          # MCP protocol integration
│   ├── agent/smith-agent/      # Agent lifecycle & orchestration
│   └── app/
│       ├── smith-cli/          # CLI for running workflow
│       ├── smith-server/       # HTTP API server
│       └── smith-observe/      # Observability
├── apps/
│   ├── selector-capture/       # as-is
│   └── smith-examples/        # Example applications
└── Cargo.toml                  # workspace manifest
```

**📥 Input of each crate:**

### smith-rpa

```
// Entry point — Session for each domain
use smith_rpa::windows::WindowsSession;
use smith_rpa::domain::{DomainRegistry, DomainTool};

let session = WindowsSession::new()?;

// 1. Standalone call (for deterministic scripts)
session.find("name=Save")?;
session.click()?;

// 2. Registration in ToolRegistry (for passing to workflow / AI)
let registry = session.tool_registry();
// registry contains: windows.find, windows.click, windows.input_text, ...

// 3. Passing to AI agent
let tools: Vec<Box<dyn DomainTool>> = session.tools();
```

### smith-ai

```
// Entry point — SmithAgent, wrapper over Rig Agent
use smith_ai::SmithAgent;
use smith_ai::provider::OpenAi;

let provider = OpenAi::new(std::env::var("OPENAI_API_KEY")?);

let agent = SmithAgent::builder(provider)
    .with_tools(session.tools())    // DomainTool → rig::tool::Tool
    .system_prompt("You are a Windows automation assistant")
    .build();

// Free mode (without workflow):
let result = agent.prompt("Open Notepad and write Hello").await?;

// Workflow mode:
let result = agent.run_workflow(workflow).await?;
```

### smith-workflow

```
use smith_workflow::{Workflow, Step};
use smith_workflow::agent::Agent;

// Workflow — sequence of steps
let workflow = Workflow::new("save_document")
    // Step 1: Deterministically open Notepad
    .step(Step::rpa("windows.process").args(json!({
        "action": "start",
        "path": "notepad.exe"
    })))

    // Step 2: Wait for window, find input field
    .step(Step::rpa("windows.find").args(json!({
        "class_name": "Edit"
    })))

    // Step 3: Type text
    .step(Step::rpa("windows.input_text").args(json!({
        "text": "Hello, World!"
    })))

    // Step 4: Agent decides whether to save
    .step(Step::agent_decide("Should we save the file?")
        .context("The user wants to save the document")
        .options(&["save", "cancel"]))

    // Step 5: If save — deterministic save
    .step(Step::rpa("windows.send_keys").args(json!({
        "keys": "^s"
    })))
    .build();

// Agent — workflow executor
let agent = Agent::new()
    .with_registry(session.tool_registry())
    .with_ai(ai_agent)
    .run(workflow, ExecutionContext::new())
    .await?;
```

**📤 Output:**

```
AgentResult {
    success: true,
    workflow_name: "save_document",
    steps_completed: 5,
    output: { "status": "saved", "path": "..." },
    execution_time_ms: 12500,
}
```

On error: `WorkflowError` with the step index, cause, and current context state. The context is not lost — you can retry from the same step.

---

**⚠️ Boundaries:**

- **Step::rpa("nonexistent")** — error at `build()` time, not at runtime. Workflow validates tool names during assembly.
- **Step::agent_decide with empty `options`** — panics at build time (developer logic error).
- **CancellationToken cancelled during RPA step** — the tool checks the token itself (already implemented in smith-core). The step is marked as `Cancelled`, state is not lost.
- **LLM did not return a response** — `Step::agent_decide` / `Step::agent_think` returns `WorkflowError::AgentError` with the raw model response for diagnostics.
- **RPA step failed (element not found)** — `WorkflowError::StepError { step_idx, source }`. A `retry_policy` can be configured for the step.
- **What if a tool from another domain (browser) is called without a session?** — `runtimeRegistry` checks if the tool is registered, otherwise `ToolNotFound`.
- **Nested workflows:** `Step::workflow(sub_workflow)` — running a sub-workflow as a single step for composition.

**📦 StepKind — full specification:**

```rust
pub enum StepKind {
    /// Deterministic RPA step. No LLM.
    /// name — tool name (e.g. "windows.click")
    /// args — JSON with parameters
    Rpa {
        name: &'static str,
        args: Value,
        retry: RetryPolicy,
    },

    /// Agent receives a prompt and decides for itself
    /// which RPA tools to call and in what order.
    Agent {
        prompt: String,
        tools: Vec<&'static str>,    // which tools are available to the agent
        max_steps: usize,            // limit on tool invocations
    },

    /// Agent generates data/decision without calling tools.
    /// Result is saved in WorkflowContext.
    Think {
        prompt: String,
        output_schema: Value,        // JSON Schema of expected response
    },

    /// Agent selects one option from a list.
    /// Result — the selected option. Further workflow execution
    /// depends on the choice (conditional routing).
    Decide {
        prompt: String,
        options: Vec<&'static str>,
    },

    /// Nested workflow
    Workflow(Workflow),
}
```

**🔀 Conditional routing after Decide:**

```rust
// Decide returns the selected option.
// Workflow can branch:

let workflow = Workflow::new("process_document")
    .step(Step::rpa("windows.find").args(json!({"name": "Document"})))
    .step(Step::agent_decide("Is this an invoice or a contract?")
        .options(&["invoice", "contract"]))
    .on_choice("invoice", Workflow::new("handle_invoice")
        .step(Step::rpa("excel.read").args(json!({"range": "A1:F20"})))
        .step(Step::agent_think("Extract the amount and date")))
    .on_choice("contract", Workflow::new("handle_contract")
        .step(Step::rpa("excel.read").args(json!({"range": "A1:H50"})))
        .step(Step::agent_think("Extract the parties and terms")))
    .build();
```

**🤖 What becomes `domain::windows::click()`:**

In code, this is not a string call, but a type-safe Builder for Step:

```rust
// This is what the API will look like for the developer:

use smith_workflow::prelude::*;
use smith_rpa::windows;

// Option A: Workflow from Steps
fn build_workflow() -> Workflow {
    Workflow::new("demo")
        .step(windows::find("name=Notepad"))
        .step(windows::click())
        .step(windows::input_text("Hello"))
        // or `.step(agent_think("Check the result"))`
        .build()
}

// Option B: Workflow from Steps with explicit names
fn build_workflow_verbose() -> Workflow {
    Workflow::new("demo")
        .step(Step::rpa("windows.find").args(json!({"name": "Notepad"})))
        .step(Step::rpa("windows.click").args(json!({"element_key": "found"})))
        .step(Step::agent("Check if Notepad opened"))
        .build()
}
```

**Option A (type-safe) — this is your `domain::windows::click()` in Rust.**

```rust
// smith-rpa::windows — public Step constructor functions
// Each function knows its parameters, returns a ready Step

pub fn find(selector: &str) -> Step {
    Step::rpa("windows.find").args(json!({"name": selector}))
}

pub fn click() -> Step {
    Step::rpa("windows.click")
}

pub fn input_text(text: &str) -> Step {
    Step::rpa("windows.input_text").args(json!({"text": text}))
}
```

---

**🔌 n8n as external orchestrator (future):**

n8n (Apache 2.0) can be used as an external trigger for smith-workflow:

```
n8n:
  [Folder Watch] → POST localhost:8742/run/process_inbox
  [Schedule 09:00] → POST localhost:8742/run/daily_report
  [Webhook]       → POST localhost:8742/run/{workflow}
  → [Telegram/Slack/Email] notification of result
```

n8n only triggers the workflow and does not manage RPA steps. **Not being developed now**, just kept in mind.

---

**✅ Success criteria:**

- [ ] `smith-core` remains unchanged — Tool trait, ExecutionContext, ToolRegistry as-is
- [ ] `smith-rpa::windows` re-exports all existing tools from smith-windows under the new API
- [ ] `smith-rpa::windows::click()` returns a ready `Step` (does not execute anything)
- [ ] `smith-ai::SmithAgent::builder(provider).with_tools(tools)` builds a Rig agent
- [ ] `smith-workflow::Workflow` validates tool names during `build()`
- [ ] `smith-workflow::Agent::run` executes workflow: RPA steps via ToolRegistry, Agent steps via SmithAgent
- [ ] `Step::agent_decide` returns the selected option; workflow supports `on_choice` branching
- [ ] `CancellationToken` is propagated to all steps
- [ ] On RPA step error, retry is possible; on Agent step error, fallback is possible
- [ ] `cargo test`, `cargo clippy -- -D warnings` pass

---

## 🗓️ Implementation plan

- [x] Create `crates/domain/smith-rpa/` — move smith-windows as `windows` module, add `domain.rs` (DomainTool trait + DomainRegistry)
- [x] Create `crates/domain/smith-ai/` — adapter `smith-core::Tool` → `rig::tool::Tool`, `SmithAgent` wrapper
- [x] Create `crates/integration/smith-workflow/` — `Workflow`, `Step`, `StepKind`, `WorkflowExecutor`
- [ ] Implement `Agent` — combines ToolRegistry + SmithAgent, executes workflow
- [ ] Add `Step::workflow(sub_workflow)` for composition
- [ ] Add `on_choice` conditional routing for `Step::agent_decide`
- [ ] Update `README.md` and `ARCHITECTURE.md`
- [ ] Checks: `cargo test`, `cargo clippy -- -D warnings`
