// crates/smith-ai/src/lib.rs
//! Rig-based LLM agent.
//!
//! Contains:
//! - `adapter` — converts smith_core::Tool → rig::tool::Tool
//! - `agent` — SmithAgent, wrapper over Rig Agent
//! - `provider` — provider configuration (OpenAI, Anthropic)

pub mod adapter;
pub mod agent;
pub mod provider;

pub use adapter::ToolAdapter;
pub use agent::SmithAgent;
pub use provider::ProviderConfig;
