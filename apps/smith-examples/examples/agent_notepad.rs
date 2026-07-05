//! # Example 2: Pure AI Agent
//!
//! LLM (via Rig) receives tools and decides on its own
//! in what order to call them for opening Notepad, typing text, and closing.
//!
//! ## Run
//! ```bash
//! $env:OPENAI_API_KEY = "sk-..."
//! cargo run --example agent_notepad
//! ```

#[cfg(windows)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::Arc;

    use rig::tool::ToolDyn;
    use smith_ai::{ProviderConfig, ToolAdapter};
    use smith_core::ExecutionContext;
    use smith_windows::tools::{FindTool, ProcessTool, SetTextTool};
    use tokio::sync::Mutex;
    use tokio_util::sync::CancellationToken;

    // -- API key --
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    // -- ExecutionContext shared between tools --
    let ctx = Arc::new(Mutex::new(ExecutionContext::new()));
    let token = CancellationToken::new();

    // -- Wrap Windows tools into Rig-compatible adapters --
    let tools: Vec<Box<dyn ToolDyn>> = vec![
        Box::new(ToolAdapter::new(
            FindTool::new(),
            ctx.clone(),
            token.clone(),
        )),
        Box::new(ToolAdapter::new(
            SetTextTool::new(),
            ctx.clone(),
            token.clone(),
        )),
        Box::new(ToolAdapter::new(
            ProcessTool::new(),
            ctx.clone(),
            token.clone(),
        )),
    ];

    // -- Build the agent --
    let provider = ProviderConfig::openai(api_key)
        .with_model("mimo-v2.5")
        .with_base_url("https://opencode.ai/zen/go/v1");

    let agent = smith_ai::SmithAgent::builder(provider)
        .system_prompt(
            "You are a Windows automation assistant. \
             You have access to tools:\n\
             - `windows.process` — start or stop an application\n\
             - `windows.find` — find a UI element on the screen\n\
             - `windows.set_text` — set text value of a UI element\n\n\
             When asked to automate Notepad:\n\
             1. Start Notepad with `windows.process`\n\
             2. Find the Edit field by class_name=\"Edit\" with `windows.find`\n\
             3. Type text with `windows.set_text` using element_key from step 2\n\
             4. Close Notepad with `windows.process` stop",
        )
        .with_tools(tools)
        .build()?;

    // -- Run (LLM plans and calls tools on its own) --
    let result = agent
        .prompt(
            "Open Notepad, type 'Hello from AI Agent!' in the text field, \
             then close Notepad.",
        )
        .await?;

    println!("✅ Agent response: {result}");

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("This example requires Windows (UI Automation).");
}
