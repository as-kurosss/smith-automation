## 🧪 Test Plan: [Name] | smith-automation

**✅ Positive:** [input] → [expected output]
**🔄 Boundary:** `0` / empty / max / cancelled token → [expected]
**❌ Negative:** Invalid input → `SmithError::InvalidParams`; timeout → `SmithError::Timeout`

**🔍 Mandatory checks:**
- [ ] On `Err`, state unchanged (idempotency per Канон 10.2)
- [ ] No duplicate events/logs
- [ ] No `unwrap()`, `panic!`, or blocking in async (Канон 4.5)
- [ ] `CancellationToken` checked before and during operation
- [ ] `#[must_use]` on all non-mutating public methods

---
## 🗓️ For `/plan`: tests as steps
- [ ] Add `#[cfg(test)] mod tests` in source file (unit tests)
- [ ] Implement tests: positive, boundary, negative, cancellation
- [ ] Run `cargo test -- --nocapture`
- [ ] Run `cargo clippy -- -D warnings`
