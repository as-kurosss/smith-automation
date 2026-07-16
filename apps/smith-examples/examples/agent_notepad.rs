//! # Example 2: Q&A with AI (no agent loop)
//!
//! Minimal LLM call — smith-ai is for simple Q&A, not agentic tool use.
//! For tool-calling agents, use `smith-agent` (formerly praxis).
//!
//! ## Run
//! ```bash
//! $env:OPENAI_API_KEY = "sk-..."
//! cargo run --example agent_notepad
//! ```

#[cfg(windows)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    use smith_ai::ProviderConfig;
    use smith_ai::SmithAgent;

    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let provider = ProviderConfig::openai(api_key).with_model("gpt-4o-mini");

    let agent = SmithAgent::new(provider).expect("Failed to create AI client");

    let result = agent
        .prompt("What are the steps to automate Notepad on Windows using UI Automation?")
        .await?;

    println!("AI says:\n{result}");
    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("This example requires Windows (UI Automation).");
}
