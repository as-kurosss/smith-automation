//! # Example: rpa_with_ai — RPA tools with AI-powered error analysis
//!
//! Demonstrates the middle layer of Smith: combining smith-core RPA tools
//! with `smith-ai` for LLM-powered Q&A.
//!
//! The example:
//! 1. Executes a calculator tool
//! 2. On error (division by zero), sends the error context to an LLM
//! 3. Asks the LLM for a recovery suggestion
//!
//! ## Prerequisites
//! Set one of these environment variables:
//! - `OPENAI_API_KEY` for OpenAI
//! - `ANTHROPIC_API_KEY` for Anthropic
//!
//! ## Run
//! ```bash
//! $env:OPENAI_API_KEY = "sk-..."
//! cargo run --example rpa_with_ai
//! ```

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use smith_ai::{ProviderConfig, SmithAgent};
use smith_core::tool::{Tool, ToolError};
use smith_core::{ExecutionContext, ToolRegistry, Unvalidated};
use tokio_util::sync::CancellationToken;

// ---------------------------------------------------------------------------
// Calculator tool (same as rpa_basic)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize)]
struct CalcInput {
    a: f64,
    b: f64,
    operator: String,
}

#[derive(Debug, Serialize)]
struct CalcOutput {
    expression: String,
    result: f64,
}

struct CalculatorTool;

#[async_trait::async_trait]
impl Tool for CalculatorTool {
    type Input = CalcInput;
    type Output = CalcOutput;

    fn name(&self) -> &'static str {
        "calculator"
    }

    fn description(&self) -> &'static str {
        "Performs basic arithmetic: +, -, *, /"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "a": { "type": "number" },
                "b": { "type": "number" },
                "operator": { "type": "string", "enum": ["+", "-", "*", "/"] }
            },
            "required": ["a", "b", "operator"]
        })
    }

    async fn execute(
        &self,
        input: CalcInput,
        _ctx: &mut ExecutionContext,
        _token: CancellationToken,
    ) -> Result<CalcOutput, ToolError> {
        match input.operator.as_str() {
            "+" => Ok(CalcOutput {
                expression: format!("{} + {}", input.a, input.b),
                result: input.a + input.b,
            }),
            "-" => Ok(CalcOutput {
                expression: format!("{} - {}", input.a, input.b),
                result: input.a - input.b,
            }),
            "*" => Ok(CalcOutput {
                expression: format!("{} * {}", input.a, input.b),
                result: input.a * input.b,
            }),
            "/" if input.b != 0.0 => Ok(CalcOutput {
                expression: format!("{} / {}", input.a, input.b),
                result: input.a / input.b,
            }),
            "/" => Err(ToolError::invalid_input(
                "Division by zero",
                Some("b".into()),
                Some(json!({"b": input.b})),
            )),
            op => Err(ToolError::invalid_input(
                format!("Unknown operator: {op}"),
                Some("operator".into()),
                None,
            )),
        }
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Smith RPA + AI — Error Analysis with LLM ===\n");

    // -- LLM setup --
    let provider = if let Ok(key) = std::env::var("OPENAI_API_KEY") {
        Some(ProviderConfig::openai(key).with_model("gpt-4o-mini"))
    } else if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
        Some(ProviderConfig::anthropic(key).with_model("claude-3-haiku-20240307"))
    } else {
        None
    };

    let ai = provider.map(|p| SmithAgent::new(p).expect("Failed to create AI client"));

    // -- RPA tool setup --
    let mut registry = ToolRegistry::new();
    registry.register(CalculatorTool);
    let mut ctx = ExecutionContext::<Unvalidated>::new().validate();
    let token = CancellationToken::new();

    // -- Execute a valid operation --
    println!("--- Successful Execution ---");
    let calc = registry
        .execute(
            "calculator",
            json!({"a": 100.0, "b": 5.0, "operator": "/"}),
            &mut ctx,
            token.clone(),
        )
        .await?;
    println!("  Result: {calc}\n");

    // -- Trigger an error and analyse it with AI --
    println!("--- Error + AI Analysis ---");
    let err = registry
        .execute(
            "calculator",
            json!({"a": 42.0, "b": 0.0, "operator": "/"}),
            &mut ctx,
            token.clone(),
        )
        .await
        .unwrap_err();

    println!("  Tool error: {err}");

    if let Some(ref ai_agent) = ai {
        let prompt = format!(
            "A Smith RPA tool ('calculator') failed with this error:\n\n{err}\n\n\
             The user was trying to divide 42 by 0.\n\
             1. Explain why this error occurred.\n\
             2. Suggest what the user should do next.\n\n\
             Keep your answer concise (max 3 sentences)."
        );
        match ai_agent.prompt(&prompt).await {
            Ok(response) => println!("\n  🤖 AI Analysis:\n{}\n", response),
            Err(e) => println!("\n  ⚠️ AI call failed (not critical): {e}\n"),
        }
    } else {
        println!("\n  (Set OPENAI_API_KEY or ANTHROPIC_API_KEY for AI analysis)\n");
    }

    println!("✅ RPA + AI example completed!");
    Ok(())
}
