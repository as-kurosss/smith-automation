//! # Example 4: FlowGraph — execution graph with error handling
//!
//! Demonstrates building a graph via `FlowGraphBuilder` with RPA nodes,
//! AI error analysis ([`Node::Think`]) and routing ([`Node::Router`]).
//!
//! ## Graph
//!
//! ```text
//! [start_notepad] ─success→ [find_edit] ─success→ [type_text] ─success→ [close]
//!                     │                                       │
//!                     └──failure→ [analyze] ─success→ [router] ┘
//!                                                      │
//!                                          ┌───────────┴──────────┐
//!                                          ▼                      ▼
//!                                     "retry" → [find_edit]   "abort" → [close]
//! ```
//!
//! ## Run
//! ```bash
//! $env:OPENAI_API_KEY = "sk-..."
//! cargo run --example flowgraph_notepad
//! ```

#[cfg(windows)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use smith_ai::SmithAgent;
    use smith_core::{AiHandler, ExecutionContext, ToolRegistry};
    use smith_graph::{
        EdgeKind, FlowGraph, GraphExecutor, Node, RetryPolicy,
    };
    use smith_windows::tools::{FindTool, ProcessTool, SetTextTool, WaitTool};
    use tokio_util::sync::CancellationToken;

    // -- API key --
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    // -- Register RPA tools --
    let mut registry = ToolRegistry::new();
    registry.register(FindTool::new());
    registry.register(SetTextTool::new());
    registry.register(ProcessTool::new());
    registry.register(WaitTool::new());

    // -- Create AI agent --
    let provider = smith_ai::ProviderConfig::openai(api_key)
        .with_model("mimo-v2.5")
        .with_base_url("https://opencode.ai/zen/go/v1");
    let ai_agent = SmithAgent::builder(provider)
        .system_prompt("You are an automation assistant specialized in Windows UI Automation.")
        .build()?;

    // -- Build FlowGraph --
    let mut b = FlowGraph::builder("notepad_flowgraph");

    // 1. RPA: start Notepad
    let start = b.add_node(Node::Rpa {
        tool: "windows.process",
        args: serde_json::json!({"action": "start", "command": "notepad.exe"}),
        retry: RetryPolicy::default(),
    });

    // 2. RPA: find Edit input field
    let find = b.add_node(Node::Rpa {
        tool: "windows.find",
        args: serde_json::json!({
            "class_name": "Edit",
            "control_type": "Edit",
            "output_key": "notepad_edit",
        }),
        retry: RetryPolicy { max_retries: 10, delay_ms: 500 },
    });

    // 3. Think: AI analyzes the error (if find failed)
    let analyze = b.add_node(Node::Think {
        prompt: "The Windows UI Automation 'find' operation failed. \
                 Analyze the possible causes (window not ready, wrong class, etc.) \
                 and prepare a recommendation. Return JSON with 'possible_causes' and \
                 'recommendation' fields.".to_string(),
        output_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "possible_causes": { "type": "array", "items": { "type": "string" } },
                "recommendation": { "type": "string", "enum": ["retry", "abort"] }
            }
        }),
    });

    // 4. Router: decides — retry or stop
    let router = b.add_node(Node::Router {
        prompt: "Based on the error analysis, should we retry finding the Edit element or abort the workflow?".to_string(),
        options: vec![
            ("retry".into(), "Retry finding the Edit element".into()),
            ("abort".into(), "Abort the workflow".into()),
        ],
    });

    // 5. RPA: type text
    let type_text = b.add_node(Node::Rpa {
        tool: "windows.set_text",
        args: serde_json::json!({
            "element_key": "notepad_edit",
            "text": "Hello from FlowGraph! Nodes + edges = ❤️",
        }),
        retry: RetryPolicy::default(),
    });

    // 6. RPA: pause 3 seconds (to see the result)
    let wait = b.add_node(Node::Rpa {
        tool: "windows.wait",
        args: serde_json::json!({"duration_ms": 3000}),
        retry: RetryPolicy::default(),
    });

    // 7. RPA: close Notepad
    let close = b.add_node(Node::Rpa {
        tool: "windows.process",
        args: serde_json::json!({"action": "stop", "name": "notepad.exe"}),
        retry: RetryPolicy::default(),
    });

    // -- Connect edges --
    // Main flow (success)
    b.connect(start, EdgeKind::Success, find);
    b.connect(find, EdgeKind::Success, type_text);
    b.connect(type_text, EdgeKind::Success, wait);
    b.connect(wait, EdgeKind::Success, close);

    // Find error handling
    b.connect(find, EdgeKind::Failure, analyze);
    b.connect(analyze, EdgeKind::Success, router);

    // Router → choice
    b.on_choice(router, "retry", find);
    b.on_choice(router, "abort", close);

    // Build graph
    let graph = b.build().map_err(|e| format!("Graph validation failed: {e}"))?;

    // -- Execute --
    let executor = GraphExecutor::new(&registry, Some(&ai_agent as &dyn AiHandler));
    let mut ctx = ExecutionContext::new();
    let token = CancellationToken::new();

    let result = executor.execute(&graph, &mut ctx, token).await?;

    println!("✅ FlowGraph execution completed:");
    println!("   name:     {}", graph.name);
    println!("   output:   {}", result);

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("This example requires Windows (UI Automation).");
}
