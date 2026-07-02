## 📐 Specification: windows.find | smith-automation

**🎯 Purpose:** Find a Windows UI element matching selector criteria and store it in `ExecutionContext` for later use by other tools (click, input_text, set_text).

**📥 Input:**
| parameter | type | constraint | example |
|-----------|------|-----------|---------|
| `name` | string | optional; element name | `"Submit"` |
| `automation_id` | string | optional; UIA identifier | `"btnSubmit"` |
| `control_type` | string | optional; see `parse_control_type()` | `"Button"` |
| `class_name` | string | optional; window class | `"Edit"` |
| `pid` | integer | optional; process ID filter | `1234` |
| `output_key` | string | **required**; context key for result | `"my_element"` |

At least one selector field should be set (otherwise `TrueCondition` matches everything — use with caution).

**📤 Output:**
| result | condition |
|--------|----------|
| `{ status: "found" }` | element found, stored under `output_key` in context |
| `Err(SmithError::ElementNotFound)` | no matching element |
| `Err(SmithError::PlatformError)` | UIA COM init or condition creation failed |

On `Err`: context is **not modified** (idempotent per Канон 10.2).

**⚠️ Boundaries:**
- `output_key` missing → `InvalidParams`
- No element matches → `ElementNotFound`
- `CancellationToken` cancelled before search → `Cancelled`
- Empty selector (no fields set) → matches root element (TrueCondition)
- Large UI tree → search is `TreeScope::Descendants` from desktop root; may be slow
- `SafeUIElement` creation and COM calls happen inside `spawn_blocking` (Канон 5.3)

**📌 Usage pattern:**
```rust
// Step 1: find element
let selector = ElementSelector::new().name("Submit").control_type("Button");
let element = selector.find_from_desktop()?;

// Step 2: store in context
ctx.set("btn", ContextValue::Custom(Arc::new(SafeUIElement::new(element))));

// Step 3: use in another tool by element_key
click.execute(json!({ "element_key": "btn" }), ctx, token)
```

📎 `crates/smith-windows/src/tools/find.rs` | `crates/smith-windows/src/selector.rs`
