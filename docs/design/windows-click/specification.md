## 📐 Specification: windows.click | smith-automation

**🎯 Purpose:** Perform a click on a UI element previously stored in `ExecutionContext`.

**📥 Input:**
| parameter | type | constraint | example |
|-----------|------|-----------|---------|
| `element_key` | string | **required**; key in context containing `SafeUIElement` | `"btn_submit"` |

Element must be stored first via `windows.find` or a selector-based tool.

**📤 Output:**
| result | condition |
|--------|----------|
| `null` | click succeeded |
| `Err(SmithError::InvalidParams)` | `element_key` missing |
| `Err(SmithError::ContextError)` | key not found in context |
| `Err(SmithError::PlatformError)` | COM click failed or spawn_blocking join failed |

On `Err`: context state is **not modified** (idempotent per Canon 10.2).

**⚠️ Boundaries:**
- `element_key` missing → `InvalidParams`
- Key not in context → `ContextError`
- Value under key is not `SafeUIElement` → `InvalidParams` (type mismatch)
- `CancellationToken` cancelled before click → `Cancelled`
- Element no longer valid (window closed) → `PlatformError` from UIA
- All UI Automation calls happen inside `spawn_blocking` (Canon 5.3)
- Currently only performs left click — no double-click or right-click support

**📌 Usage:**
```rust
// With element from context:
click.execute(json!({ "element_key": "btn" }), ctx, token)

// Element must be pre-stored:
// 1. windows.find → stores SafeUIElement under "btn"
// 2. windows.click → reads "btn" from context and clicks
```

📎 `crates/domain/smith-windows/src/tools/click.rs`
