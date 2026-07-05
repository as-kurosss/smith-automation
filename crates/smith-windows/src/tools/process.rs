// crates/smith-windows/src/tools/process.rs
use std::collections::HashSet;
use std::time::Duration;

use async_trait::async_trait;
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult};
use tokio_util::sync::CancellationToken;

/// Whitelist of allowed executable names for process start.
///
/// This prevents arbitrary command execution via the HTTP API.
/// Only well-known Windows utilities are permitted.
/// `cmd.exe` and `powershell.exe` are intentionally excluded because they allow
/// arbitrary command execution via arguments (`/c`, `-Command`).
///
/// # Security
///
/// The daemon must NOT be exposed to untrusted networks (`--host 127.0.0.1`).
/// Comparison is case-insensitive (Windows filesystem convention).
fn is_command_allowed(cmd: &str) -> bool {
    let allowed: HashSet<&str> = HashSet::from_iter([
        "notepad.exe",
        "calc.exe",
        "mspaint.exe",
        "explorer.exe",
        "write.exe",
        "wordpad.exe",
    ]);

    // Extract the file name from the path
    let name = cmd.rsplit_once(['/', '\\']).map_or(cmd, |(_, file)| file);

    allowed.iter().any(|&a| a.eq_ignore_ascii_case(name))
}

/// Tool for managing Windows processes.
///
/// Supports actions:
/// - `start` — launches a new process (does not wait for completion)
/// - `stop` — stops a process by PID or name (does not wait for taskkill to finish)
/// - `sleep` — pauses for `duration_ms` milliseconds
pub struct ProcessTool;

impl ProcessTool {
    /// Creates a new `ProcessTool` instance.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProcessTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for ProcessTool {
    fn name(&self) -> &'static str {
        "windows.process"
    }

    fn description(&self) -> &'static str {
        "Manages Windows processes: start, stop, or sleep"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["start", "stop", "sleep"],
                    "description": "Action to perform"
                },
                "command": {
                    "type": "string",
                    "description": "Executable path (required for start)"
                },
                "args": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Command-line arguments (for start)"
                },
                "working_dir": {
                    "type": "string",
                    "description": "Working directory (for start)"
                },
                "pid": {
                    "type": "integer",
                    "description": "Process ID to stop"
                },
                "name": {
                    "type": "string",
                    "description": "Process image name to stop (e.g. notepad.exe)"
                },
                "duration_ms": {
                    "type": "integer",
                    "minimum": 0,
                    "description": "Sleep duration in milliseconds (required for sleep)"
                },
                "delay_before_ms": {
                    "type": "integer",
                    "minimum": 0,
                    "description": "Delay before execution in milliseconds"
                },
                "delay_after_ms": {
                    "type": "integer",
                    "minimum": 0,
                    "description": "Delay after execution in milliseconds"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(
        &self,
        config: ToolConfig,
        _ctx: &mut ExecutionContext,
        token: CancellationToken,
    ) -> SmithResult<ToolResult> {
        // 0. Optional delay before execution
        crate::tools::apply_delay_before(&config).await;

        let action = config
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SmithError::InvalidParams("Missing 'action'".into()))?;

        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        let result = match action {
            "start" => self::action_start(&config),
            "stop" => {
                let config = config.clone();
                tokio::task::spawn_blocking(move || self::action_stop(&config))
                    .await
                    .map_err(|e| SmithError::PlatformError {
                        message: "Blocking task join failed".into(),
                        source: Box::new(e),
                    })?
            }
            "sleep" => self::action_sleep(config.clone()).await,
            other => Err(SmithError::InvalidParams(format!(
                "Unknown action: {other}"
            ))),
        };

        // Optional delay after execution (only on success)
        if result.is_ok() {
            crate::tools::apply_delay_after(&config).await;
        }

        result
    }
}

fn action_start(config: &Value) -> SmithResult<ToolResult> {
    // Validation: command is required
    let cmd_str = config
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SmithError::InvalidParams("Missing 'command' for start action".into()))?;

    // Command injection protection (Canon 10.1 Input Validation)
    if !is_command_allowed(cmd_str) {
        return Err(SmithError::InvalidParams(format!(
            "Command '{cmd_str}' is not in the allowed list",
        )));
    }

    let mut cmd = std::process::Command::new(cmd_str);

    if let Some(args) = config.get("args").and_then(|v| v.as_array()) {
        for arg in args {
            if let Some(s) = arg.as_str() {
                cmd.arg(s);
            }
        }
    }

    if let Some(dir) = config.get("working_dir").and_then(|v| v.as_str()) {
        cmd.current_dir(dir);
    }

    let child = cmd.spawn().map_err(|e| SmithError::PlatformError {
        message: "Failed to start process".into(),
        source: Box::new(e),
    })?;

    let pid = child.id();

    Ok(json!({
        "status": "started",
        "pid": pid
    }))
}

async fn action_sleep(config: Value) -> SmithResult<ToolResult> {
    let duration_ms = config
        .get("duration_ms")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| {
            SmithError::InvalidParams("Missing 'duration_ms' for sleep action".into())
        })?;

    tokio::time::sleep(Duration::from_millis(duration_ms)).await;

    Ok(json!({
        "status": "slept",
        "duration_ms": duration_ms
    }))
}

fn action_stop(config: &Value) -> SmithResult<ToolResult> {
    use std::process::Stdio;

    if let Some(pid) = config.get("pid").and_then(serde_json::Value::as_u64) {
        let output = std::process::Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .stdin(Stdio::null())
            .output()
            .map_err(|e| SmithError::PlatformError {
                message: "taskkill spawn failed".into(),
                source: Box::new(e),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(SmithError::PlatformError {
                message: format!("taskkill for pid {pid} failed: {stderr}"),
                source: Box::new(std::io::Error::other(stderr.as_ref())),
            });
        }

        Ok(json!({ "status": "stopped", "method": "pid", "pid": pid }))
    } else if let Some(name) = config.get("name").and_then(|v| v.as_str()) {
        let output = std::process::Command::new("taskkill")
            .args(["/F", "/IM", name])
            .stdin(Stdio::null())
            .output()
            .map_err(|e| SmithError::PlatformError {
                message: "taskkill spawn failed".into(),
                source: Box::new(e),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(SmithError::PlatformError {
                message: format!("taskkill for {name} failed: {stderr}"),
                source: Box::new(std::io::Error::other(stderr.as_ref())),
            });
        }

        Ok(json!({ "status": "stopped", "method": "name", "name": name }))
    } else {
        Err(SmithError::InvalidParams(
            "Must provide 'pid' or 'name' for stop action".into(),
        ))
    }
}
