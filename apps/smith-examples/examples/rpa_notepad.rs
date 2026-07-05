//! # Example 1: Pure RPA
//!
//! All steps are deterministic — no AI involved.
//! Workflow: open Notepad → find Edit field → type text → close.
//!
//! ## Run
//! ```bash
//! cargo run --example rpa_notepad
//! ```

#[cfg(windows)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use smith_core::{ExecutionContext, ToolRegistry};
    use smith_windows::tools::{FindTool, ProcessTool, SetTextTool};
    use smith_workflow::WorkflowExecutor;
    use smith_workflow::prelude::*;
    use tokio_util::sync::CancellationToken;

    // -- Register Windows tools --
    let mut registry = ToolRegistry::new();
    registry.register(FindTool::new());
    registry.register(SetTextTool::new());
    registry.register(ProcessTool::new());

    // -- Build workflow from RPA steps --
    let workflow = Workflow::new("rpa_notepad")
        // 1. Start notepad.exe
        .step(Step::rpa("windows.process").args(json!({
            "action": "start",
            "command": "notepad.exe",
        })))
        // 2. Find Edit field — with retry since the window may not open instantly
        .step(
            Step::rpa("windows.find")
                .args(json!({
                    "class_name": "Edit",
                    "control_type": "Edit",
                    "output_key": "notepad_edit",
                }))
                .retry(RetryPolicy {
                    max_retries: 10,
                    delay_ms: 500,
                }),
        )
        // 3. Set text via ValuePattern (faster than input_text)
        .step(Step::rpa("windows.set_text").args(json!({
            "element_key": "notepad_edit",
            "text": "Hello from smith RPA!",
        })))
        // 4. Pause 3 seconds — to see the result
        .step(Step::rpa("windows.process").args(json!({
            "action": "sleep",
            "duration_ms": 3000,
        })))
        // 5. Close Notepad (force kill — for demo)
        .step(Step::rpa("windows.process").args(json!({
            "action": "stop",
            "name": "notepad.exe",
        })))
        .build();

    // -- Execute workflow --
    let executor = WorkflowExecutor::new(&registry, None::<&dyn smith_core::AiHandler>);
    let mut ctx = ExecutionContext::new();
    let token = CancellationToken::new();

    let result = executor.execute(workflow?, &mut ctx, token).await?;

    println!("✅ RPA workflow completed:");
    println!("   name:     {}", result.workflow_name);
    println!("   steps:    {}", result.steps_completed);
    println!("   time_ms:  {}", result.execution_time_ms);
    println!("   output:   {}", result.output);

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("This example requires Windows (UI Automation).");
}
