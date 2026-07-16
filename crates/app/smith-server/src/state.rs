//! **AppState** — shared application state for the Praxis API server.
//!
//! Manages the persistent agent registry and session store.

use serde::{Deserialize, Serialize};
use smith_agent::registry::{AgentRegistry, ProviderFactoryRegistry, SessionStore};
use smith_agent::sandbox::PendingApprovalStore;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

/// Configuration for an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Unique name for this server configuration.
    pub name: String,
    /// The MCP server command (e.g. "npx", "node", "python").
    pub command: String,
    /// Command-line arguments.
    #[serde(default)]
    pub args: Vec<String>,
}

/// A notification from background tasks.
#[derive(Debug, Clone, Serialize)]
pub struct Notification {
    pub kind: String,
    pub message: String,
    pub timestamp: String,
}

/// Persisted global server settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    /// Whether episodic memory recording is enabled for new chat sessions.
    /// Changing this does not affect running agents.
    #[serde(default = "default_true")]
    pub episodic_memory_enabled: bool,
    /// Default tool result cap in bytes (None = no cap).
    /// New agents inherit this as their tool_result_cap.
    #[serde(default)]
    pub default_tool_result_cap: Option<usize>,
    /// Whether env-gates (e.g. PRAXIS_ALLOW_UNSANDBOXED_RECALL) are active.
    #[serde(default = "default_true")]
    pub env_gate_enabled: bool,
}

fn default_true() -> bool {
    true
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            episodic_memory_enabled: true,
            default_tool_result_cap: None,
            env_gate_enabled: true,
        }
    }
}

impl ServerSettings {
    /// Load settings from a JSON file, or return defaults if it doesn't exist.
    pub fn load(path: &std::path::Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    /// Save settings to a JSON file.
    pub fn save(&self, path: &std::path::Path) {
        if let Ok(s) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, s);
        }
    }
}

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    /// Persistent agent & provider registry.
    pub registry: Arc<AgentRegistry>,
    /// Persistent session store.
    pub sessions: Arc<SessionStore>,
    /// Data directory path.
    pub data_dir: PathBuf,
    /// Web dist directory for static files.
    pub dist_dir: PathBuf,
    /// Request timeout in seconds for LLM calls.
    pub request_timeout_seconds: u64,
    /// Owner identifier (empty = single-user mode).
    pub owner_id: String,
    /// Pending notifications for the frontend.
    pub notifications: Arc<Mutex<Vec<Notification>>>,
    /// Provider factory registry — creates LlmClient from ProviderConfig.
    pub provider_registry: Arc<ProviderFactoryRegistry>,
    /// MCP server configurations (name → command/args).
    pub mcp_servers: Arc<Mutex<Vec<McpServerConfig>>>,
    /// Pending approval requests for interactive Ask-mode policy.
    pub approvals: PendingApprovalStore,
    /// Shared episodic memory for recording evicted turns across all agents.
    /// Opened at startup, `None` if the database could not be created.
    pub episodic_memory:
        Option<std::sync::Arc<std::sync::Mutex<smith_agent::memory::EpisodicMemory>>>,
    /// Persisted global server settings (episodic toggle, defaults, etc.).
    pub settings: Arc<RwLock<ServerSettings>>,
}

impl AppState {
    /// Create a new application state with registry + sessions in `data_dir`.
    ///
    /// # Errors
    /// Returns an I/O error if the data directory cannot be created,
    /// or the registry / session store files cannot be read.
    pub fn new(data_dir: PathBuf) -> std::io::Result<Self> {
        std::fs::create_dir_all(&data_dir)?;

        let registry_path = data_dir.join("registry.json");
        let registry = AgentRegistry::open(&registry_path)
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        let sessions = SessionStore::open(&data_dir)?;

        let dist_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("web")
            .join("dist");

        let provider_registry = Arc::new(smith_providers::register_default_factories());

        let request_timeout_seconds = std::env::var("PRAXIS_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        let owner_id = std::env::var("PRAXIS_OWNER").unwrap_or_default();

        let approvals = PendingApprovalStore::new();

        // Load server settings from disk
        let settings_path = data_dir.join("settings.json");
        let settings = ServerSettings::load(&settings_path);

        // Eagerly open episodic memory (non-fatal — chat handlers will work
        // without it, they just won't record evicted turns).
        let episodic_path = data_dir.join("episodic.db");
        let episodic_memory = if settings.episodic_memory_enabled {
            let m = smith_agent::memory::EpisodicMemory::open(&episodic_path)
                .ok()
                .map(|m| std::sync::Arc::new(std::sync::Mutex::new(m)));
            if m.is_none() {
                tracing::warn!("Failed to open episodic memory at {:?}", episodic_path);
            }
            m
        } else {
            None
        };

        let state = Self {
            registry: Arc::new(registry),
            sessions: Arc::new(sessions),
            data_dir,
            dist_dir,
            request_timeout_seconds,
            owner_id,
            provider_registry,
            mcp_servers: Arc::new(Mutex::new(Vec::new())),
            approvals,
            notifications: Arc::new(Mutex::new(Vec::new())),
            episodic_memory,
            settings: Arc::new(RwLock::new(settings)),
        };

        // Wire up approval notifications: every time a tool creates a pending
        // approval request, push a notification so the frontend can pick it up.
        let notifier = state.clone();
        state.approvals.set_on_pending(Box::new(move |req| {
            notifier.notify(
                "approval_created",
                format!(
                    "Tool '{}' requires approval — {}",
                    req.tool_name, req.reason
                ),
            );
        }));

        Ok(state)
    }

    /// Push a notification for the frontend.
    pub fn notify(&self, kind: impl Into<String>, message: impl Into<String>) {
        if let Ok(mut notes) = self.notifications.lock() {
            notes.push(Notification {
                kind: kind.into(),
                message: message.into(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            });
        }
    }

    /// Drain all pending notifications.
    pub fn drain_notifications(&self) -> Vec<Notification> {
        self.notifications
            .lock()
            .map_or_else(|_| Vec::new(), |mut notes| std::mem::take(&mut *notes))
    }

    /// Create a minimal state for integration testing (uses temp directory).
    #[cfg(test)]
    pub fn test() -> Self {
        let tmp = std::env::temp_dir().join(format!("praxis-api-test-{}", uuid::Uuid::new_v4()));
        Self::new(tmp).expect("failed to create test AppState")
    }
}
