// crates/smith-ai/src/agent.rs
//! SmithAgent — wrapper over Rig Agent.

use async_trait::async_trait;
use futures::future::BoxFuture;
use rig::agent::Agent;
use rig::agent::PromptHook;
use rig::client::CompletionClient;
use rig::completion::CompletionModel;
use rig::completion::Prompt;
use rig::providers::anthropic;
use rig::providers::openai;
use rig::tool::ToolDyn;
use serde_json::Value;
use smith_core::{AiHandler, ExecutionContext, SmithError, SmithResult};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::provider::ProviderConfig;

// ---- Type-erased agent ----

trait AgentLike: Send + Sync {
    fn prompt<'a>(&'a self, prompt: &'a str) -> BoxFuture<'a, Result<String, SmithError>>;
}

impl<M, P> AgentLike for Agent<M, P>
where
    M: CompletionModel + Send + Sync + 'static,
    P: PromptHook<M> + Send + Sync + 'static,
{
    fn prompt<'a>(&'a self, prompt: &'a str) -> BoxFuture<'a, Result<String, SmithError>> {
        Box::pin(async move {
            Prompt::prompt(self, prompt)
                .await
                .map_err(|e| SmithError::Other(anyhow::anyhow!("Rig agent error: {e}")))
        })
    }
}

// ---- Public API ----

/// SmithAgent — wrapper over Rig Agent.
pub struct SmithAgent {
    inner: Box<dyn AgentLike>,
}

impl SmithAgent {
    /// Creates a builder.
    pub fn builder(provider: ProviderConfig) -> SmithAgentBuilder {
        SmithAgentBuilder {
            provider,
            tools: vec![],
            system_prompt: None,
        }
    }

    /// Executes a prompt in free mode (without workflow).
    pub async fn prompt(&self, prompt: &str) -> Result<String, SmithError> {
        self.inner.prompt(prompt).await
    }
}

/// Builder for SmithAgent.
pub struct SmithAgentBuilder {
    provider: ProviderConfig,
    tools: Vec<Box<dyn ToolDyn>>,
    system_prompt: Option<String>,
}

impl SmithAgentBuilder {
    /// Adds tools.
    pub fn with_tools(mut self, tools: Vec<Box<dyn ToolDyn>>) -> Self {
        self.tools.extend(tools);
        self
    }

    /// Sets the system prompt.
    pub fn system_prompt(mut self, prompt: &str) -> Self {
        self.system_prompt = Some(prompt.to_string());
        self
    }

    /// Builds the SmithAgent.
    ///
    /// # Errors
    ///
    /// Returns `SmithError` if the Rig Agent could not be created.
    pub fn build(self) -> Result<SmithAgent, SmithError> {
        let preamble = self.system_prompt.unwrap_or_default();

        let SmithAgentBuilder {
            provider,
            tools,
            system_prompt: _,
        } = self;

        let inner: Box<dyn AgentLike> = match &provider {
            ProviderConfig::OpenAi {
                api_key,
                model,
                base_url,
            } => {
                let mut builder = openai::Client::builder().api_key(api_key.clone());
                if let Some(url) = base_url {
                    builder = builder.base_url(url);
                }
                let client = builder.build().map_err(|e| {
                    SmithError::Other(anyhow::anyhow!("Failed to create OpenAI client: {e}"))
                })?;

                Box::new(
                    client
                        .completions_api()
                        .agent(model)
                        .preamble(&preamble)
                        .tools(tools)
                        .default_max_turns(10)
                        .build(),
                )
            }
            ProviderConfig::Anthropic {
                api_key,
                model,
                base_url,
            } => {
                let mut builder = anthropic::Client::builder().api_key(api_key.clone());
                if let Some(url) = base_url {
                    builder = builder.base_url(url);
                }
                let client = builder.build().map_err(|e| {
                    SmithError::Other(anyhow::anyhow!("Failed to create Anthropic client: {e}"))
                })?;

                Box::new(
                    client
                        .agent(model)
                        .preamble(&preamble)
                        .tools(tools)
                        .default_max_turns(10)
                        .build(),
                )
            }
        };

        Ok(SmithAgent { inner })
    }
}

// ---- AiHandler ----

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
            return Err(smith_core::SmithError::Cancelled);
        }

        if !tools.is_empty() {
            warn!(
                "Agent step requested specific tools {:?}, but SmithAgent uses all built-in tools; consider configuring tools at build time",
                tools
            );
        }

        info!("Agent step: {prompt}");
        let result = self.prompt(prompt).await.map_err(|e| {
            smith_core::SmithError::Other(anyhow::anyhow!("Agent prompt failed: {e}"))
        })?;

        ctx.set(
            "last_agent_result",
            smith_core::ContextValue::String(result.clone()),
        );

        // Quick heuristic: only try JSON parse if response looks like JSON
        let trimmed = result.trim_start();
        let looks_like_json = trimmed.starts_with('{') || trimmed.starts_with('[');
        #[allow(clippy::collapsible_if)]
        if looks_like_json {
            if let Ok(val) = serde_json::from_str::<Value>(&result) {
                return Ok(val);
            }
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
            return Err(smith_core::SmithError::Cancelled);
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
        let result = self.prompt(&full_prompt).await.map_err(|e| {
            smith_core::SmithError::Other(anyhow::anyhow!("Think prompt failed: {e}"))
        })?;

        ctx.set(
            "last_think_result",
            smith_core::ContextValue::String(result.clone()),
        );

        // Quick heuristic: only try JSON parse if response looks like JSON
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
            return Err(smith_core::SmithError::Cancelled);
        }

        if options.is_empty() {
            return Err(smith_core::SmithError::InvalidParams(
                "Decide step must have at least one option".into(),
            ));
        }

        let options_str = options.join("\", \"");
        let full_prompt = format!(
            "{prompt}\n\nChoose one option from: [\"{options_str}\"]\nRespond only with the option name, no explanations."
        );

        info!("Decide step: {prompt}");
        let result = self.prompt(&full_prompt).await.map_err(|e| {
            smith_core::SmithError::Other(anyhow::anyhow!("Decide prompt failed: {e}"))
        })?;

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
            Err(smith_core::SmithError::InvalidParams(format!(
                "LLM returned invalid option '{trimmed}', expected one of {options:?}"
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock AgentLike that returns preset responses.
    struct MockAgent {
        response: String,
    }

    impl AgentLike for MockAgent {
        fn prompt<'a>(&'a self, _prompt: &'a str) -> BoxFuture<'a, Result<String, SmithError>> {
            let response = self.response.clone();
            Box::pin(async move { Ok(response) })
        }
    }

    fn make_agent(response: &str) -> SmithAgent {
        SmithAgent {
            inner: Box::new(MockAgent {
                response: response.to_string(),
            }),
        }
    }

    #[tokio::test]
    async fn test_agent_run_returns_plain_text() {
        let agent = make_agent("just some text");
        let mut ctx = ExecutionContext::new();
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
        let mut ctx = ExecutionContext::new();
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
        let mut ctx = ExecutionContext::new();
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
        let mut ctx = ExecutionContext::new();
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
        let mut ctx = ExecutionContext::new();
        let cancelled = CancellationToken::new();
        cancelled.cancel();

        let result = agent
            .decide("choose", &["a".into(), "b".into()], &mut ctx, &cancelled)
            .await;

        assert!(matches!(result, Err(smith_core::SmithError::Cancelled)));
    }

    #[tokio::test]
    async fn test_decide_empty_options() {
        let agent = make_agent("");
        let mut ctx = ExecutionContext::new();
        let token = CancellationToken::new();

        let result = agent.decide("choose", &[], &mut ctx, &token).await;

        assert!(matches!(result, Err(smith_core::SmithError::InvalidParams(_))));
    }

    #[tokio::test]
    async fn test_decide_valid_choice() {
        let agent = make_agent("option_b");
        let mut ctx = ExecutionContext::new();
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
        let mut ctx = ExecutionContext::new();
        let token = CancellationToken::new();

        let result = agent
            .decide(
                "pick one",
                &["option_a".into(), "option_b".into()],
                &mut ctx,
                &token,
            )
            .await;

        assert!(matches!(result, Err(smith_core::SmithError::InvalidParams(_))));
    }

    #[tokio::test]
    async fn test_decide_trims_quotes() {
        let agent = make_agent(r#""option_a""#);
        let mut ctx = ExecutionContext::new();
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
