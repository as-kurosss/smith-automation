// crates/smith-windows/src/tools/process.rs
use std::collections::HashSet;

use async_trait::async_trait;
use serde_json::{Value, json};
use smith_core::{ExecutionContext, SmithError, SmithResult, Tool, ToolConfig, ToolResult};
use tokio_util::sync::CancellationToken;

/// Whitelist of allowed executable names for process start.
///
/// This prevents arbitrary command execution via the HTTP API.
/// Only well-known Windows utilities and the specified automation targets are permitted.
/// Keys can be bare names (e.g. "notepad.exe") or full paths.
fn is_command_allowed(cmd: &str) -> bool {
    let allowed: HashSet<&str> = HashSet::from_iter([
        // Standard Windows apps
        "notepad.exe",
        "calc.exe",
        "mspaint.exe",
        "cmd.exe",
        "powershell.exe",
        "explorer.exe",
        "write.exe",
        "wordpad.exe",
        // Common paths for these executables
        "NOTEPAD.EXE",
        "CALC.EXE",
        "MSPAINT.EXE",
        "CMD.EXE",
        "POWERSHELL.EXE",
        "EXPLORER.EXE",
        "WRITE.EXE",
        "WORDPAD.EXE",
    ]);

    // Extract the file name from the path
    let name = cmd
        .rsplit_once(|c| c == '/' || c == '\\')
        .map(|(_, file)| file)
        .unwrap_or(cmd);

    allowed.contains(name)
}

/// Инструмент для управления процессами Windows.
///
/// Поддерживает действия:
/// - `start` — запуск нового процесса (не ждёт завершения)
/// - `stop` — остановка процесса по PID или имени (не ждёт завершения taskkill)
pub struct ProcessTool;

impl ProcessTool {
    /// Создаёт новый экземпляр `ProcessTool`.
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
        "Manages Windows processes: start or stop (fire-and-forget)"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["start", "stop"],
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
        let action = config
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SmithError::InvalidParams("Missing 'action'".into()))?;

        if token.is_cancelled() {
            return Err(SmithError::Cancelled);
        }

        match action {
            "start" => self::action_start(&config),
            "stop" => self::action_stop(&config),
            other => Err(SmithError::InvalidParams(format!(
                "Unknown action: {other}"
            ))),
        }
    }
}

fn action_start(config: &Value) -> SmithResult<ToolResult> {
    // Валидация: command обязателен
    let cmd_str = config
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SmithError::InvalidParams("Missing 'command' for start action".into()))?;

    // Command injection protection (Canon 10.1 Input Validation)
    if !is_command_allowed(cmd_str) {
        return Err(SmithError::InvalidParams(format!(
            "Command '{}' is not in the allowed list",
            cmd_str
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

    let child = cmd
        .spawn()
        .map_err(|e| SmithError::PlatformWithCause {
            message: "Failed to start process".into(),
            source: Box::new(e),
        })?;

    let pid = child.id();

    Ok(json!({
        "status": "started",
        "pid": pid
    }))
}

fn action_stop(config: &Value) -> SmithResult<ToolResult> {
    use std::process::Stdio;

    if let Some(pid) = config.get("pid").and_then(|v| v.as_u64()) {
        // Остановка по PID через taskkill — только запускаем, не ждём завершения
        std::process::Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| SmithError::PlatformWithCause {
                message: "taskkill spawn failed".into(),
                source: Box::new(e),
            })?;

        Ok(json!({ "status": "stop_initiated", "method": "pid", "pid": pid }))
    } else if let Some(name) = config.get("name").and_then(|v| v.as_str()) {
        // Остановка по имени через taskkill — только запускаем, не ждём завершения
        std::process::Command::new("taskkill")
            .args(["/F", "/IM", name])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| SmithError::PlatformWithCause {
                message: "taskkill spawn failed".into(),
                source: Box::new(e),
            })?;

        Ok(json!({ "status": "stop_initiated", "method": "name", "name": name }))
    } else {
        Err(SmithError::InvalidParams(
            "Must provide 'pid' or 'name' for stop action".into(),
        ))
    }
}
