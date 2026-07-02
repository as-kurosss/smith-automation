## 📐 Specification: windows.process | smith-automation

**🎯 Purpose:** Manage Windows processes — start an executable or stop by PID/name.

**📥 Input:**
| parameter | type | constraint | example |
|-----------|------|-----------|---------|
| `action` | `"start"` \| `"stop"` | required | `"start"` |
| `command` | string | **must be in allowlist**; case-insensitive | `"notepad.exe"` |
| `args` | `string[]` | optional | `["file.txt"]` |
| `working_dir` | string | optional | `"C:\\temp"` |
| `pid` | integer | for stop action | `1234` |
| `name` | string | for stop action | `"notepad.exe"` |

**📤 Output:**
| result | condition |
|--------|----------|
| `{ status: "started", pid: u32 }` | action=start succeeds |
| `{ status: "stop_initiated", method: "pid", pid }` | action=stop by PID |
| `{ status: "stop_initiated", method: "name", name }` | action=stop by name |
| `Err(SmithError::InvalidParams)` | unknown action / missing field / unallowed command |
| `Err(SmithError::PlatformError)` | taskkill or spawn failed |

On `Err`: process is **not started** (idempotent per Канон 10.2).

**⚠️ Boundaries:**
- Unknown `action` → `InvalidParams`
- `start` without `command` → `InvalidParams`
- `command` not in allowlist → `InvalidParams`
- `stop` without `pid` or `name` → `InvalidParams`
- `CancellationToken` cancelled before action → `Cancelled`
- `taskkill` may fail (process already dead) — logged via `tracing::warn!`, response still `stop_initiated`

**🔒 Security:** Only executables in `is_command_allowed()` whitelist can be started.
`cmd.exe` and `powershell.exe` are **excluded** — they allow arbitrary RCE via `/c` / `-Command`.
Daemon must run on `--host 127.0.0.1` (default).

📎 `crates/smith-windows/src/tools/process.rs`
