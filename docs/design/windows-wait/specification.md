## 📐 Specification: windows.wait | smith-automation

**🎯 Purpose:** Pause execution for a specified duration. Useful for waiting between steps (UI animations, window opening, data processing) where a fixed delay is needed.

**📥 Input:**

| parameter | type | constraint | example |
|-----------|------|-----------|---------|
| `duration_ms` | integer | **required**; >= 0 | `3000` |
| `delay_before_ms` | integer | optional; >= 0 | `500` |
| `delay_after_ms` | integer | optional; >= 0 | `500` |

**📤 Output:**

| result | condition |
|--------|----------|
| `{ status: "waited", duration_ms: 3000 }` | delay completed successfully |
| `Err(SmithError::InvalidParams)` | `duration_ms` missing or invalid |
| `Err(SmithError::Cancelled)` | `CancellationToken` cancelled during sleep |

On `Err`: context state is **not modified** (idempotent per Canon 10.2).

**⚠️ Boundaries:**
- `duration_ms` missing → `InvalidParams`
- `duration_ms = 0` → returns immediately (no-op)
- `CancellationToken` cancelled during sleep → `Cancelled` (via `tokio::select!`)
- Context is never modified (read-only tool)

**📌 Usage pattern:**
```rust
// In a flow graph
let wait = b.add_node(Node::Rpa {
    tool: "windows.wait",
    args: serde_json::json!({"duration_ms": 3000}),
    retry: RetryPolicy::default(),
});

// With additional delays around the tool itself
let wait = b.add_node(Node::Rpa {
    tool: "windows.wait",
    args: serde_json::json!({
        "duration_ms": 3000,
        "delay_before_ms": 500,
        "delay_after_ms": 500,
    }),
    retry: RetryPolicy::default(),
});
```

📎 `crates/domain/smith-windows/src/tools/wait.rs`
