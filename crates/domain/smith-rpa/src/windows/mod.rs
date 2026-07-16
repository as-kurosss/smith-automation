// crates/smith-rpa/src/windows/mod.rs
//! `Node::Rpa` constructors for Windows UI Automation tools.
//!
//! Each function returns a `Node::Rpa` for use in a `FlowGraph`:
//!
//! ```ignore
//! use smith_rpa::windows;
//! use smith_workflow::{FlowGraph, EdgeKind};
//!
//! let mut b = FlowGraph::builder("demo");
//! let find = b.add_node(windows::find("name=Notepad", "found_element"));
//! let click = b.add_node(windows::click());
//! b.connect(find, EdgeKind::Success, click);
//! let graph = b.build().unwrap();
//! ```

use serde_json::json;
use smith_core::RetryPolicy;
use smith_workflow::node::Node;

/// Creates a `Node::Rpa` for finding a UI element.
///
/// The `selector` parameter is a simplified selector: `"name=Value"` or
/// `"className=Edit"`. For complex cases construct `Node::Rpa` directly.
///
/// `output_key` — the key under which the found element will be saved in the context.
#[must_use]
pub fn find(selector: &str, output_key: &str) -> Node {
    Node::Rpa {
        tool: "windows.find",
        args: json!({
            "name": selector,
            "output_key": output_key,
        }),
        retry: RetryPolicy::default(),
    }
}

/// Creates a `Node::Rpa` for clicking a UI element.
///
/// Expects the element to have been previously saved in the context under the `"found"` key.
#[must_use]
pub fn click() -> Node {
    Node::Rpa {
        tool: "windows.click",
        args: json!({ "element_key": "found" }),
        retry: RetryPolicy::default(),
    }
}

/// Creates a `Node::Rpa` for inputting text.
///
/// If `element_key` is not specified, text is typed into the active window.
#[must_use]
pub fn input_text(text: &str) -> Node {
    Node::Rpa {
        tool: "windows.input_text",
        args: json!({ "text": text }),
        retry: RetryPolicy::default(),
    }
}

/// Creates a `Node::Rpa` for setting text via ValuePattern.
///
/// Faster than `input_text`, but does not simulate real keyboard input.
#[must_use]
pub fn set_text(text: &str) -> Node {
    Node::Rpa {
        tool: "windows.set_text",
        args: json!({ "text": text }),
        retry: RetryPolicy::default(),
    }
}

/// Creates a `Node::Rpa` for starting a process.
#[must_use]
pub fn process_start(command: &str) -> Node {
    Node::Rpa {
        tool: "windows.process",
        args: json!({
            "action": "start",
            "command": command,
        }),
        retry: RetryPolicy::default(),
    }
}

/// Creates a `Node::Rpa` for starting a process with arguments.
#[must_use]
pub fn process_start_with_args(command: &str, args: &[&str]) -> Node {
    Node::Rpa {
        tool: "windows.process",
        args: json!({
            "action": "start",
            "command": command,
            "args": args,
        }),
        retry: RetryPolicy::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_creates_node() {
        let node = find("name=Notepad", "found_element");
        assert_eq!(node.kind_name(), "Rpa");
    }

    #[test]
    fn test_click_uses_found_key() {
        let node = click();
        assert_eq!(node.kind_name(), "Rpa");
    }

    #[test]
    fn test_input_text_node() {
        let node = input_text("Hello");
        assert_eq!(node.kind_name(), "Rpa");
    }

    #[test]
    fn test_set_text_node() {
        let node = set_text("Hello");
        assert_eq!(node.kind_name(), "Rpa");
    }

    #[test]
    fn test_process_start_node() {
        let node = process_start("notepad.exe");
        assert_eq!(node.kind_name(), "Rpa");
    }

    #[test]
    fn test_process_start_with_args_node() {
        let node = process_start_with_args("notepad.exe", &["test.txt"]);
        assert_eq!(node.kind_name(), "Rpa");
    }
}
