// crates/smith-ai/src/agent.rs
//! Minimal HTTP-based LLM client for Q&A, Think, and Decide.
//!
//! No agent framework dependencies — just direct HTTP calls to
//! OpenAI and Anthropic chat completion APIs.

use async_trait::async_trait;
use serde_json::Value;
use smith_core::{AiHandler, ExecutionContext, SmithError, SmithResult};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::provider::ProviderConfig;

// ---------------------------------------------------------------------------
// AiClient trait — minimal LLM interface
// ---------------------------------------------------------------------------

/// Minimal async LLM completion trait.
///
/// Implementations make a bare HTTP POST to the provider's chat completions
/// endpoint and return the response text.  No agent loop, no tool calling.
#[async_trait]
pub trait AiClient: Send + Sync {
    /// Send a prompt and return the text response.
    async fn complete(&self, prompt: &str) -> Result<String, SmithError>;
}

// ---------------------------------------------------------------------------
// OpenAI implementation
// ---------------------------------------------------------------------------

/// OpenAI / OpenAI-compatible chat completion client.
pub struct OpenAiClient {
    http: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl OpenAiClient {
    /// Create a new OpenAI client from provider config.
    #[must_use]
    pub fn new(config: &ProviderConfig) -> Option<Self> {
        let (api_key, model, base_url) = match config {
            ProviderConfig::OpenAi {
                api_key,
                model,
                base_url,
            } => (
                api_key.clone(),
                model.clone(),
                base_url
                    .clone()
                    .unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            ),
            ProviderConfig::OpenAiCompatible {
                api_key,
                model,
                base_url,
            } => (api_key.clone(), model.clone(), base_url.clone()),
            _ => return None,
        };
        Some(Self {
            http: reqwest::Client::new(),
            api_key,
            model,
            base_url,
        })
    }
}

#[async_trait]
impl AiClient for OpenAiClient {
    async fn complete(&self, prompt: &str) -> Result<String, SmithError> {
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let body = serde_json::json!({
            "model": self.model,
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7,
            "max_tokens": 4096,
        });

        let resp = self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| SmithError::Other(anyhow::anyhow!("OpenAI request failed: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            return Err(SmithError::Other(anyhow::anyhow!(
                "OpenAI error {status}: {text}"
            )));
        }

        let data: Value = resp
            .json()
            .await
            .map_err(|e| SmithError::Other(anyhow::anyhow!("OpenAI parse failed: {e}")))?;

        let text = data["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| {
                SmithError::Other(anyhow::anyhow!("OpenAI response missing content: {data}"))
            })?;

        Ok(text.to_string())
    }
}

// ---------------------------------------------------------------------------
// Anthropic implementation
// ---------------------------------------------------------------------------

/// Anthropic Claude chat completion client.
pub struct AnthropicClient {
    http: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl AnthropicClient {
    /// Create a new Anthropic client from provider config.
    #[must_use]
    pub fn new(config: &ProviderConfig) -> Option<Self> {
        match config {
            ProviderConfig::Anthropic {
                api_key,
                model,
                base_url,
            } => Some(Self {
                http: reqwest::Client::new(),
                api_key: api_key.clone(),
                model: model.clone(),
                base_url: base_url
                    .clone()
                    .unwrap_or_else(|| "https://api.anthropic.com".to_string()),
            }),
            _ => None,
        }
    }
}

#[async_trait]
impl AiClient for AnthropicClient {
    async fn complete(&self, prompt: &str) -> Result<String, SmithError> {
        let url = format!("{}/v1/messages", self.base_url.trim_end_matches('/'));
        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": 4096,
            "messages": [{"role": "user", "content": prompt}],
        });

        let resp = self
            .http
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| SmithError::Other(anyhow::anyhow!("Anthropic request failed: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            return Err(SmithError::Other(anyhow::anyhow!(
                "Anthropic error {status}: {text}"
            )));
        }

        let data: Value = resp
            .json()
            .await
            .map_err(|e| SmithError::Other(anyhow::anyhow!("Anthropic parse failed: {e}")))?;

        let text = data["content"][0]["text"].as_str().ok_or_else(|| {
            SmithError::Other(anyhow::anyhow!("Anthropic response missing text: {data}"))
        })?;

        Ok(text.to_string())
    }
}

// ---------------------------------------------------------------------------
// Factory
// ---------------------------------------------------------------------------

/// Create an [`AiClient`] from provider configuration.
///
/// # Errors
/// Returns `SmithError::Other` if the provider config does not match any
/// known client type or if required fields are missing.
pub fn create_client(config: &ProviderConfig) -> Result<Box<dyn AiClient>, SmithError> {
    match config {
        ProviderConfig::OpenAi { .. } | ProviderConfig::OpenAiCompatible { .. } => {
            OpenAiClient::new(config)
                .map(|c| Box::new(c) as Box<dyn AiClient>)
                .ok_or_else(|| {
                    SmithError::Other(anyhow::anyhow!(
                        "Failed to create OpenAI client from config"
                    ))
                })
        }
        ProviderConfig::Anthropic { .. } => AnthropicClient::new(config)
            .map(|c| Box::new(c) as Box<dyn AiClient>)
            .ok_or_else(|| {
                SmithError::Other(anyhow::anyhow!(
                    "Failed to create Anthropic client from config"
                ))
            }),
    }
}

// ---------------------------------------------------------------------------
// SmithAgent — minimal agent that implements AiHandler
// ---------------------------------------------------------------------------

/// Minimal LLM agent for Q&A, Think, and Decide operations.
///
/// Unlike the Rig-based predecessor this agent makes a single HTTP call
/// per invocation — no agent loop, no tool calling.
pub struct SmithAgent {
    client: Box<dyn AiClient>,
}

impl SmithAgent {
    /// Create a new agent from provider configuration.
    ///
    /// # Errors
    /// Returns an error if the provider configuration is invalid or
    /// the client cannot be instantiated.
    pub fn new(config: ProviderConfig) -> Result<Self, SmithError> {
        Ok(Self {
            client: create_client(&config)?,
        })
    }

    /// Execute a simple prompt and return the text response.
    ///
    /// # Errors
    /// Returns an error if the LLM request fails.
    pub async fn prompt(&self, prompt: &str) -> Result<String, SmithError> {
        self.client.complete(prompt).await
    }
}

// ---------------------------------------------------------------------------
// AiHandler implementation (for smith-graph compatibility)
// ---------------------------------------------------------------------------

#[async_trait]
impl AiHandler for SmithAgent {
    async fn agent_run(
        &self,
        prompt: &str,
        tools: &[String],
        _max_steps: usize,
        ctx: &mut ExecutionContext,
        token: &CancellationToken,
    ) -> SmithResult<Value> {
        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        if !tools.is_empty() {
            warn!(
                "Agent step requested specific tools {:?}, but smith-ai has no tool support; \
                 use smith-agent for tool-calling agents",
                tools
            );
        }

        info!("Agent step: {prompt}");
        let result = self.prompt(prompt).await?;

        ctx.set(
            "last_agent_result",
            smith_core::ContextValue::String(result.clone()),
        );

        let trimmed = result.trim_start();
        if (trimmed.starts_with('{') || trimmed.starts_with('['))
            && let Ok(val) = serde_json::from_str::<Value>(&result)
        {
            return Ok(val);
        }

        Ok(Value::String(result))
    }

    async fn think(
        &self,
        prompt: &str,
        schema: &Value,
        ctx: &mut ExecutionContext,
        token: &CancellationToken,
    ) -> SmithResult<Value> {
        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        let full_prompt = if schema.is_object() || schema.is_array() {
            format!(
                "{prompt}\n\nRespond with valid JSON matching this schema:\n```json\n{}\n```",
                serde_json::to_string_pretty(schema).unwrap_or_default()
            )
        } else {
            prompt.to_string()
        };

        info!("Think step: {prompt}");
        let result = self.prompt(&full_prompt).await?;

        ctx.set(
            "last_think_result",
            smith_core::ContextValue::String(result.clone()),
        );

        let trimmed = result.trim_start();
        if (trimmed.starts_with('{') || trimmed.starts_with('['))
            && let Ok(val) = serde_json::from_str::<Value>(&result)
        {
            return Ok(val);
        }

        Ok(Value::String(result))
    }

    async fn decide(
        &self,
        prompt: &str,
        options: &[String],
        ctx: &mut ExecutionContext,
        token: &CancellationToken,
    ) -> SmithResult<String> {
        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        if options.is_empty() {
            return Err(SmithError::InvalidParams(
                "Decide step must have at least one option".into(),
            ));
        }

        let options_str = options.join("\", \"");
        let full_prompt = format!(
            "{prompt}\n\nChoose one option from: [\"{options_str}\"]\nRespond only with the option name, no explanations."
        );

        info!("Decide step: {prompt}");
        let result = self.prompt(&full_prompt).await?;

        let trimmed = result.trim().trim_matches('"').to_string();
        if options.iter().any(|o| o == &trimmed) {
            ctx.set(
                "last_decision",
                smith_core::ContextValue::String(trimmed.clone()),
            );
            Ok(trimmed)
        } else {
            warn!(
                "LLM returned invalid option '{}', expected one of {:?}",
                trimmed, options
            );
            Err(SmithError::InvalidParams(format!(
                "LLM returned invalid option '{trimmed}', expected one of {options:?}"
            )))
        }
    }
}

// ---------------------------------------------------------------------------
// Tests (mocked, no HTTP calls)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use smith_core::{Ready, Unvalidated};

    /// Mock client that returns preset responses.
    struct MockClient {
        response: String,
    }

    #[async_trait]
    impl AiClient for MockClient {
        async fn complete(&self, _prompt: &str) -> Result<String, SmithError> {
            Ok(self.response.clone())
        }
    }

    fn make_agent(response: &str) -> SmithAgent {
        SmithAgent {
            client: Box::new(MockClient {
                response: response.to_string(),
            }),
        }
    }

    fn ctx_ready() -> ExecutionContext<Ready> {
        ExecutionContext::<Unvalidated>::new().validate()
    }

    #[tokio::test]
    async fn test_agent_run_returns_plain_text() {
        let agent = make_agent("just some text");
        let mut ctx = ctx_ready();
        let token = CancellationToken::new();

        let result = agent
            .agent_run("test", &[], 10, &mut ctx, &token)
            .await
            .unwrap();

        assert_eq!(result, Value::String("just some text".into()));
        assert!(ctx.get("last_agent_result").is_some());
    }

    #[tokio::test]
    async fn test_agent_run_parses_json() {
        let agent = make_agent(r#"{"key": "value"}"#);
        let mut ctx = ctx_ready();
        let token = CancellationToken::new();

        let result = agent
            .agent_run("test", &[], 10, &mut ctx, &token)
            .await
            .unwrap();

        assert_eq!(result, serde_json::json!({ "key": "value" }));
    }

    #[tokio::test]
    async fn test_think_returns_plain_text() {
        let agent = make_agent("analysis result");
        let mut ctx = ctx_ready();
        let token = CancellationToken::new();

        let result = agent
            .think("analyze", &Value::Null, &mut ctx, &token)
            .await
            .unwrap();

        assert_eq!(result, Value::String("analysis result".into()));
        assert!(ctx.get("last_think_result").is_some());
    }

    #[tokio::test]
    async fn test_think_parses_json() {
        let agent = make_agent(r#"{"decision": "ok"}"#);
        let mut ctx = ctx_ready();
        let token = CancellationToken::new();

        let result = agent
            .think("decide", &Value::Null, &mut ctx, &token)
            .await
            .unwrap();

        assert_eq!(result, serde_json::json!({ "decision": "ok" }));
    }

    #[tokio::test]
    async fn test_decide_cancelled() {
        let agent = make_agent("");
        let mut ctx = ctx_ready();
        let cancelled = CancellationToken::new();
        cancelled.cancel();

        let result = agent
            .decide("choose", &["a".into(), "b".into()], &mut ctx, &cancelled)
            .await;

        assert!(matches!(result, Err(SmithError::Cancelled)));
    }

    #[tokio::test]
    async fn test_decide_empty_options() {
        let agent = make_agent("");
        let mut ctx = ctx_ready();
        let token = CancellationToken::new();

        let result = agent.decide("choose", &[], &mut ctx, &token).await;

        assert!(matches!(result, Err(SmithError::InvalidParams(_))));
    }

    #[tokio::test]
    async fn test_decide_valid_choice() {
        let agent = make_agent("option_b");
        let mut ctx = ctx_ready();
        let token = CancellationToken::new();

        let result = agent
            .decide(
                "pick one",
                &["option_a".into(), "option_b".into()],
                &mut ctx,
                &token,
            )
            .await
            .unwrap();

        assert_eq!(result, "option_b");
        assert!(ctx.get("last_decision").is_some());
    }

    #[tokio::test]
    async fn test_decide_invalid_choice() {
        let agent = make_agent("option_c");
        let mut ctx = ctx_ready();
        let token = CancellationToken::new();

        let result = agent
            .decide(
                "pick one",
                &["option_a".into(), "option_b".into()],
                &mut ctx,
                &token,
            )
            .await;

        assert!(matches!(result, Err(SmithError::InvalidParams(_))));
    }

    #[tokio::test]
    async fn test_decide_trims_quotes() {
        let agent = make_agent(r#""option_a""#);
        let mut ctx = ctx_ready();
        let token = CancellationToken::new();

        let result = agent
            .decide(
                "pick one",
                &["option_a".into(), "option_b".into()],
                &mut ctx,
                &token,
            )
            .await
            .unwrap();

        assert_eq!(result, "option_a");
    }
}
