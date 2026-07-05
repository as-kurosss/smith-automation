# Workflow example: Document processing from a folder

Business process: every morning invoices and contracts arrive in the `C:\inbox` folder. Each file needs to be opened, its document type determined, data extracted, saved to the database, and moved to the archive.

```rust
use smith_workflow::prelude::*;
use smith_rpa::windows;
use serde_json::json;

// ───────────────────────────────────────────────
// Workflow: "Incoming document processing"
// ───────────────────────────────────────────────
let workflow = Workflow::new("process_inbox")
    // ═══════════════════════════════════════════
    // Step 1: Deterministically — open folder
    // ═══════════════════════════════════════════
    .step(windows::process_start(r#"C:\Windows\explorer.exe"#, "C:\\inbox"))

    // ═══════════════════════════════════════════
    // Step 2: Deterministically — get file list
    // ═══════════════════════════════════════════
    .step(Step::rpa("fs.list_files").args(json!({
        "path": "C:\\inbox",
        "pattern": "*.pdf;*.xlsx"
    })))

    // ═══════════════════════════════════════════
    // Step 3: LLM decides — which files to process
    // ═══════════════════════════════════════════
    // Files can be: invoice, contract, memo, spam
    // LLM looks at names and sorts
    .step(Step::agent_think("Sort files by type: invoice, contract, other")
        .schema(json!({
            "type": "object",
            "properties": {
                "invoices": { "type": "array", "items": { "type": "string" } },
                "contracts": { "type": "array", "items": { "type": "string" } },
                "skipped": { "type": "array", "items": { "type": "string" } }
            }
        })))

    // ═══════════════════════════════════════════
    // Step 4: LLM decides — where to start
    // ═══════════════════════════════════════════
    .step(Step::agent_decide("Where to start processing?")
        .options(&["process_invoices_first", "process_contracts_first"]))

    // ── Next — process each invoice ──
    // (in reality this would be for_each, but for the example — one invoice)

    // ═══════════════════════════════════════════
    // Step 5: Deterministically — open file
    // ═══════════════════════════════════════════
    .step(windows::process_start("EXCEL.EXE", "C:\\inbox\\invoice_123.xlsx"))

    // ═══════════════════════════════════════════
    // Step 6: Deterministically — select and copy data
    // ═══════════════════════════════════════════
    .step(Step::rpa("windows.find").args(json!({"name": "Total"})))
    .step(windows::click())
    .step(Step::rpa("windows.copy").args(json!({})))
    .step(Step::rpa("clipboard.read").args(json!({})))

    // ═══════════════════════════════════════════
    // Step 7: LLM checks — is the amount valid?
    // ═══════════════════════════════════════════
    .step(Step::agent_think("Check that the amount in the document is a valid number and > 0"))

    // ═══════════════════════════════════════════
    // Step 8: LLM decides — what next
    // ═══════════════════════════════════════════
    .step(Step::agent_decide("Is invoice correct?")
        .options(&["save_to_db", "mark_for_review", "skip"]))

    // ── Conditional routing ──
    .on_choice("save_to_db", Workflow::new("save_invoice")
        // Deterministically: fill form in 1C
        .step(windows::process_start("1CV8.exe", ""))
        .step(windows::find("name=Invoice for payment"))
        .step(windows::set_text("Amount"))
        .step(windows::find("name=Save"))
        .step(windows::click())
        // LLM: verify it was saved
        .step(Step::agent_think("Verify that the document was saved"))
    )
    .on_choice("mark_for_review", Workflow::new("mark_for_review")
        // Deterministically: move to review folder
        .step(Step::rpa("fs.move_file").args(json!({
            "from": "{{file_path}}",
            "to": "C:\\inbox\\review\\"
        })))
        // LLM: write the reason
        .step(Step::agent_think("Describe why this invoice needs review"))
    )
    .on_choice("skip", Workflow::new("skip")
        .step(Step::rpa("fs.move_file").args(json!({
            "from": "{{file_path}}",
            "to": "C:\\inbox\\skipped\\"
        })))
    )

    // ═══════════════════════════════════════════
    // Step 9: LLM — processing report
    // ═══════════════════════════════════════════
    .step(Step::agent_think("Create a brief report: which files were processed, which were skipped, how long it took"))

    .build();
```

## How it works at runtime

Execution sequence for a typical run:

```
Step 0  [RPA]     explorer.exe C:\inbox                    → ✓
Step 1  [RPA]     fs.list_files                             → [invoice_123.xlsx, contract_456.pdf, readme.txt]
Step 2  [Think]   LLM sorts files by type                  → invoices: [invoice_123.xlsx], contracts: [contract_456.pdf], skipped: [readme.txt]
Step 3  [Decide]  LLM chooses where to start              → "process_invoices_first"
Step 4  [RPA]     EXCEL.EXE invoice_123.xlsx                → ✓
Step 5  [RPA]     windows.find "Total"                      → ✓
Step 6  [RPA]     windows.click                             → ✓
Step 7  [RPA]     clipboard.read                            → "12 450.00 ₽"
Step 8  [Think]   LLM checks amount                        → valid: true
Step 9  [Decide]  LLM: is invoice correct?                  → "save_to_db"
       └── on_choice → launch sub-workflow "save_invoice"
Step 10 [RPA]     1CV8.exe                                   → ✓
Step 11 [RPA]     windows.find "Invoice for payment"         → ✓
Step 12 [RPA]     windows.set_text                           → ✓
Step 13 [RPA]     windows.find "Save"                        → ✓
Step 14 [RPA]     windows.click                              → ✓
Step 15 [Think]   Verify save                                → ✓
Step 16 [Think]   LLM generates report                       → "Processed: 1 invoice, 1 contract. Skipped: 1."

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Workflow: process_inbox completed (28.4s)
Steps: 17 | RPA: 11 | AI: 6 | Success: ✓
```

## What matters here

**RPA steps (11 of 17)** — cheap, fast, deterministic. Open, find, click, read.

**AI steps (6 of 17)** — only where a decision is needed:
- Classify files → Think
- Choose order → Decide
- Verify correctness → Think
- Choose action → Decide
- Write reason → Think
- Generate report → Think

**No AI steps for simple actions.** The LLM is not called for `click()`, `find()` or `input_text()` — that would be a waste of money and time.

**Conditional routing** — after Decide, the route changes: invoice goes to the database, error goes to review, junk goes to skip. Without LLM branching, this would be an if-else forest.
