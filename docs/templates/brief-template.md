## 🤖 Instruction for Agent: [Module Name] | smith-automation

**📁 Sources (read in priority order):**
1. `docs/design/[module]/specification.md` — input/output, boundaries, criteria
2. `docs/design/[module]/contract.md` — requirements, guarantees, prohibitions, failures
3. `docs/design/[module]/test-plan.md` — test scenarios, mandatory checks
4. `docs/adr/XXX-[module].md` — architectural decisions
5. `AGENTS.md` — project rules, graphify-rs, process

**🔗 Cross-references:**
- `docs/design/[module]/brief.md` — dependency description (if any)
- `docs/adr/XXX-[module].md` — cross-cutting concerns, spawn_blocking (if applicable)
- `crates/smith-core/src/` — core traits: `Tool`, `ExecutionContext`, `SmithError`
- `crates/smith-windows/src/` — WinAPI specifics: `SafeUIElement`, `spawn_blocking`

**🎯 Task:**
[Describe task: form implementation plan, generate code, or another specific task]

**📋 Output format (strict):**
```
[File] → [Entities] → [cfg-flags] → [Tests] → [Validation]
```

**✅ Mandatory plan elements:**
- [Describe mandatory elements, e.g.]:
  - Validation BEFORE any backend/COM call (Канон 10.1)
  - Error type: `SmithError` (thiserror) with exact variant names from contract
  - Signature with explicit config struct `[Module]Config { timeout: Duration, cancellation: CancellationToken }`
  - COM / blocking WinAPI calls isolated via `tokio::task::spawn_blocking` (Канон 5.3)
  - `#[must_use]` on constructors and pure query methods
  - `#[derive(Debug, Clone)]` for data types; `unsafe impl Send + Sync` only with safety comment
  - Tests: `#[cfg(test)] mod tests` inside `src/[crate]/[module].rs` + `tests/integration/`
  - `Mock[Module]Backend` with `Arc<Mutex<MockState>>` for idempotency check on `Err`

**🚫 Prohibitions:**
- [Describe prohibitions, e.g.]:
  - Don't generate code at plan stage
- [Describe prohibitions, e.g.]:
  - Don't use `unwrap()` / `panic!()` / `expect()` even in examples
- [Describe prohibitions, e.g.]:
  - Don't create `src/[crate]/[module]/tests.rs` (combine in `mod tests` or move to `tests/`)
- [Describe prohibitions, e.g.]:
  - Don't change contract without explicit agreement
- [Describe prohibitions, e.g.]:
  - Don't block async runtime directly — always use `spawn_blocking` for sync calls

**🔄 Process:**
1. [Describe first step, e.g.: Read sources in priority order]
2. [Describe second step, e.g.: Form plan with graphify-rs query for architecture context]
3. [Describe third step, if applicable]

**📝 Metadata:**
- Author: [Agent role, e.g.: smith-core Architect / smith-windows Engineer]
- Date: [Creation date]
- Status: `[draft]` / `[approved]` / `[approved_with_corrections]` / `[deprecated]`
