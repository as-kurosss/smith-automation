// crates/smith-ai/src/provider.rs
//! LLM provider configuration.

/// LLM provider configuration.
#[derive(Clone)]
pub enum ProviderConfig {
    /// OpenAI (GPT-4o, o4-mini, GPT-4.1, etc.).
    OpenAi {
        /// API key.
        api_key: String,
        /// Model (default "gpt-4o", typically overridden for your proxy).
        model: String,
        /// Base URL (for self-hosted / proxy).
        base_url: Option<String>,
    },
    /// Anthropic (Claude).
    Anthropic {
        /// API key.
        api_key: String,
        /// Model (default "claude-sonnet-4-20250514").
        model: String,
        /// Base URL (for self-hosted / proxy).
        base_url: Option<String>,
    },
}

impl ProviderConfig {
    /// Creates an OpenAI config.
    #[must_use]
    pub fn openai(api_key: impl Into<String>) -> Self {
        Self::OpenAi {
            api_key: api_key.into(),
            model: "gpt-4o".to_string(),
            base_url: None,
        }
    }

    /// Creates an Anthropic config.
    #[must_use]
    pub fn anthropic(api_key: impl Into<String>) -> Self {
        Self::Anthropic {
            api_key: api_key.into(),
            model: "claude-sonnet-4-20250514".to_string(),
            base_url: None,
        }
    }

    /// Sets the model.
    #[must_use]
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        match &mut self {
            Self::OpenAi { model: m, .. } | Self::Anthropic { model: m, .. } => {
                *m = model.into();
            }
        }
        self
    }

    /// Sets the base URL.
    #[must_use]
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        match &mut self {
            Self::OpenAi { base_url: b, .. } | Self::Anthropic { base_url: b, .. } => {
                *b = Some(url.into());
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_default_model() {
        let config = ProviderConfig::openai("sk-test");
        match config {
            ProviderConfig::OpenAi {
                api_key,
                model,
                base_url,
            } => {
                assert_eq!(api_key, "sk-test");
                assert_eq!(model, "gpt-4o");
                assert!(base_url.is_none());
            }
            _ => panic!("Expected OpenAi variant"),
        }
    }

    #[test]
    fn test_anthropic_default_model() {
        let config = ProviderConfig::anthropic("sk-ant-test");
        match config {
            ProviderConfig::Anthropic {
                api_key,
                model,
                base_url,
            } => {
                assert_eq!(api_key, "sk-ant-test");
                assert_eq!(model, "claude-sonnet-4-20250514");
                assert!(base_url.is_none());
            }
            _ => panic!("Expected Anthropic variant"),
        }
    }

    #[test]
    fn test_with_model_override() {
        let config = ProviderConfig::openai("key").with_model("o4-mini");
        match config {
            ProviderConfig::OpenAi { model, .. } => assert_eq!(model, "o4-mini"),
            _ => panic!("Expected OpenAi variant"),
        }
    }

    #[test]
    fn test_with_base_url() {
        let config = ProviderConfig::openai("key").with_base_url("https://my-proxy.example.com/v1");
        match config {
            ProviderConfig::OpenAi { base_url, .. } => {
                assert_eq!(
                    base_url,
                    Some("https://my-proxy.example.com/v1".to_string())
                );
            }
            _ => panic!("Expected OpenAi variant"),
        }
    }
}
