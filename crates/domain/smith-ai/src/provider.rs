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
        /// Base URL (without `/chat/completions`, Rig adds it).
        base_url: Option<String>,
    },
    /// Anthropic (Claude).
    Anthropic {
        /// API key.
        api_key: String,
        /// Model (default "claude-sonnet-4-20250514").
        model: String,
        /// Base URL (without `/messages`, Rig adds it).
        base_url: Option<String>,
    },
    /// OpenAI-compatible (OpenRouter, Ollama, vLLM, OpenCode, etc.).
    ///
    /// Uses Rig's OpenAI client with a custom `base_url`.
    /// Rig adds `/chat/completions` to `base_url` automatically.
    OpenAiCompatible {
        /// API key.
        api_key: String,
        /// Model name.
        model: String,
        /// Base URL without `/chat/completions` (e.g. `https://opencode.ai/zen/go/v1`).
        base_url: String,
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

    /// Creates an OpenAI-compatible config.
    ///
    /// `base_url` should NOT include `/chat/completions` — Rig adds it.
    #[must_use]
    pub fn openai_compatible(
        api_key: impl Into<String>,
        model: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Self {
        Self::OpenAiCompatible {
            api_key: api_key.into(),
            model: model.into(),
            base_url: base_url.into(),
        }
    }

    /// Sets the model.
    #[must_use]
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        match &mut self {
            Self::OpenAi { model: m, .. }
            | Self::Anthropic { model: m, .. }
            | Self::OpenAiCompatible { model: m, .. } => {
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
            Self::OpenAiCompatible { base_url: b, .. } => {
                *b = url.into();
            }
        }
        self
    }

    /// Sets the full URL (including `/chat/completions`).
    ///
    /// Convenience: automatically strips `/chat/completions` suffix
    /// and sets `base_url`. Works for all variants.
    #[must_use]
    pub fn with_full_url(mut self, url: impl Into<String>) -> Self {
        let url = url.into();
        let base = url
            .strip_suffix("/chat/completions")
            .or_else(|| url.strip_suffix("/messages"))
            .unwrap_or(&url)
            .to_string();
        match &mut self {
            Self::OpenAi { base_url: b, .. } | Self::Anthropic { base_url: b, .. } => {
                *b = Some(base);
            }
            Self::OpenAiCompatible { base_url: b, .. } => {
                *b = base;
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
