## 📐 Specification: [Module/Function Name] | smith-automation

**🎯 Purpose:** [1 sentence: why it's needed, what problem it solves]

**📥 Input:**
- `parameter` (type) | constraint | example

**📤 Output:**
- `result` (type) | side effect | example
- On error: `SmithError` variant | what DOES NOT change (Canon 10.2)

**⚠️ Boundaries:**
- What if input is empty / `0` / `NaN` / max?
- What if called in an invalid state?
- What if `CancellationToken` is already cancelled?

**✅ Success criteria:**
- [ ] All scenarios from "Boundaries" handled without panics
- [ ] State doesn't break on error (idempotency per Canon 10.2)
- [ ] Log/metric records result or failure cause
- [ ] `#[must_use]` on constructors and query methods
- [ ] `unsafe impl Send + Sync` only with /// Safety doc

---
## 🗓️ Implementation plan (for `/plan`)
- [ ] Create/update file: `crates/[crate]/src/[module].rs`
- [ ] Implement types, `#[derive(Debug, Clone)]` for data structs
- [ ] Implement functions with contracts; COM/blocking via `spawn_blocking`
- [ ] Add `#[cfg(test)] mod tests` with positive, boundary, negative cases
- [ ] Update docs: `docs/adr/XXX.md` or `AGENTS.md`
- [ ] Checks: `cargo test`, `cargo clippy -- -D warnings`
- [ ] Verify with `graphify-rs build --no-llm --output ./smith-graphify` if architecture changed
