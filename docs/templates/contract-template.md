## 📜 Contract: [Function/Module] | smith-automation

**🔹 Requirements (BEFORE call):** [Conditions]
**🔸 Guarantees (AFTER):** If `Ok`: [what changes]; If `Err`: [what DOES NOT change — state unchanged per Canon 10.2]
**🚫 Prohibitions:** [What the module DOES NOT do; e.g. no direct async blocking, no `unwrap()`/`panic!()`]
**⚡ Failures:** [Reaction to timeout (CancellationToken), cancellation, invalid data]

---
## 🗓️ For `/plan`: key validation checkpoints
- [ ] Input validation happens BEFORE any backend call (Canon 10.1)
- [ ] `CancellationToken` is checked before and during long operations
- [ ] Errors handled via `SmithResult` / `SmithError`, not `panic!()`
- [ ] Events/logs are sent [when]
- [ ] COM / blocking calls isolated in `spawn_blocking` (Canon 5.3)
