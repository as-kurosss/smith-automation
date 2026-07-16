// crates/smith-ai/src/lib.rs
//! Minimal LLM client for Q&A, Think, and Decide operations.
//!
//! No agent framework dependencies — just direct HTTP calls to
//! OpenAI and Anthropic chat completion APIs.
//!
//! Contains:
//! - `agent` — AiClient trait, OpenAiClient, AnthropicClient, SmithAgent
//! - `provider` — provider configuration (OpenAI, Anthropic)

pub mod agent;
pub mod provider;

pub use agent::{AiClient, AnthropicClient, OpenAiClient, SmithAgent, create_client};
pub use provider::ProviderConfig;
