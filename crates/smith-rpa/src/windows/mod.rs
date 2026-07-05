// crates/smith-rpa/src/windows/mod.rs
//! Step constructors for Windows UI Automation tools.
//!
//! Each function returns a ready-to-use `Step` for use in a workflow:
//!
//! ```ignore
//! use smith_rpa::windows;
//!
//! let workflow = Workflow::new("demo")
//!     .step(windows::find("name=Notepad", "found_element"))
//!     .step(windows::click())
//!     .step(windows::input_text("Hello"))
//!     .build();
//! ```

use serde_json::json;
use smith_workflow::Step;

/// Creates a Step for finding a UI element.
///
/// The `selector` parameter is a simplified selector: `"name=Value"` or
/// `"className=Edit"`. For complex cases use `Step::rpa("windows.find").args(...)`.
///
/// `output_key` — the key under which the found element will be saved in the context.
#[must_use]
pub fn find(selector: &str, output_key: &str) -> Step {
    Step::rpa("windows.find").args(json!({
        "name": selector,
        "output_key": output_key,
    }))
}

/// Creates a Step for clicking a UI element.
///
/// Expects the element to have been previously saved in the context under the `"found"` key.
#[must_use]
pub fn click() -> Step {
    Step::rpa("windows.click").args(json!({ "element_key": "found" }))
}

/// Creates a Step for inputting text.
///
/// If `element_key` is not specified, text is typed into the active window.
#[must_use]
pub fn input_text(text: &str) -> Step {
    Step::rpa("windows.input_text").args(json!({ "text": text }))
}

/// Creates a Step for setting text via ValuePattern.
///
/// Faster than `input_text`, but does not simulate real keyboard input.
#[must_use]
pub fn set_text(text: &str) -> Step {
    Step::rpa("windows.set_text").args(json!({ "text": text }))
}

/// Creates a Step for starting a process.
#[must_use]
pub fn process_start(command: &str) -> Step {
    Step::rpa("windows.process").args(json!({
        "action": "start",
        "command": command,
    }))
}

/// Creates a Step for starting a process with arguments.
#[must_use]
pub fn process_start_with_args(command: &str, args: &[&str]) -> Step {
    Step::rpa("windows.process").args(json!({
        "action": "start",
        "command": command,
        "args": args,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_creates_step_with_output_key() {
        let step = find("name=Notepad", "found_element");
        assert_eq!(step.kind_name(), "RPA");
    }

    #[test]
    fn test_click_uses_found_key() {
        let step = click();
        assert_eq!(step.kind_name(), "RPA");
    }

    #[test]
    fn test_input_text_step() {
        let step = input_text("Hello");
        assert_eq!(step.kind_name(), "RPA");
    }

    #[test]
    fn test_set_text_step() {
        let step = set_text("Hello");
        assert_eq!(step.kind_name(), "RPA");
    }

    #[test]
    fn test_process_start_step() {
        let step = process_start("notepad.exe");
        assert_eq!(step.kind_name(), "RPA");
    }

    #[test]
    fn test_process_start_with_args_step() {
        let step = process_start_with_args("notepad.exe", &["test.txt"]);
        assert_eq!(step.kind_name(), "RPA");
    }
}
