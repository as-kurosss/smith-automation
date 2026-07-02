//! Windows UIA element capture logic
//!
//! Uses `uiautomation` crate to get the element under the cursor,
//! then walks the tree up to the desktop root collecting stable attributes.

use uiautomation::core::UIAutomation;
use uiautomation::core::{UIElement, UITreeWalker};
use uiautomation::types::ControlType;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

use crate::types::{BestSelector, CapturedElement, PathNode};

/// Returns the current cursor position in screen (logical) coordinates,
/// matching the coordinate system used by UIA bounding rectangles.
/// Uses `GetCursorPos` instead of rdev's `MouseMove` tracking because
/// rdev reports physical pixels while UIA uses DPI-scaled logical pixels.
pub fn cursor_position() -> (f64, f64) {
    unsafe {
        let mut pt = std::mem::zeroed();
        if GetCursorPos(&mut pt).is_ok() {
            (pt.x as f64, pt.y as f64)
        } else {
            (0.0, 0.0)
        }
    }
}

/// Captures the element at the given desktop coordinates.
///
/// Walks the UIA control-view tree from the root to find the deepest
/// element at the cursor position. Avoids `element_from_point` which
/// can return the desktop root depending on COM threading state.
pub fn capture_at_point(x: f64, y: f64) -> Result<(Vec<PathNode>, BestSelector), String> {
    let automation = UIAutomation::new().map_err(|e| format!("UIA init failed: {e}"))?;
    let ix = x as i32;
    let iy = y as i32;

    let root = automation
        .get_root_element()
        .map_err(|e| format!("get_root_element: {e}"))?;
    let walker = automation
        .get_control_view_walker()
        .map_err(|e| format!("get_control_view_walker: {e}"))?;

    let element = find_deepest_at_point(root, &walker, ix, iy)?;

    // Walk up the tree from target to desktop root (raw view for full path)
    let walker = automation
        .create_tree_walker()
        .map_err(|e| format!("create_tree_walker: {e}"))?;

    let mut path: Vec<PathNode> = Vec::new();
    let mut current = element;
    loop {
        let node = read_node(&current);
        path.push(node);
        match walker.get_parent(&current) {
            Ok(parent) => current = parent,
            Err(_) => break,
        }
    }
    path.reverse();

    let best = build_best_selector(&path);
    Ok((path, best))
}

/// Reads stable attributes from a UIA element into a `PathNode`.
fn read_node(element: &uiautomation::core::UIElement) -> PathNode {
    let ct = element
        .get_control_type()
        .unwrap_or(ControlType::Custom);
    let ct_name = format!("{ct:?}");

    PathNode {
        control_type: ct_name,
        class_name: element.get_classname().ok().filter(|s| !s.is_empty()),
        name: element.get_name().ok().filter(|s| !s.is_empty()),
        automation_id: element.get_automation_id().ok().filter(|s| !s.is_empty()),
    }
}

/// Captures the element that currently has keyboard focus.
#[allow(dead_code)]
pub fn capture_focused_element() -> Option<CapturedElement> {
    let automation = UIAutomation::new().ok()?;
    let element = automation.get_focused_element().ok()?;
    let walker = automation.create_tree_walker().ok()?;

    let mut path: Vec<PathNode> = Vec::new();
    let mut current = element;

    loop {
        let node = read_node(&current);
        path.push(node);
        match walker.get_parent(&current) {
            Ok(parent) => current = parent,
            Err(_) => break,
        }
    }

    path.reverse();
    let best = build_best_selector(&path);
    Some(CapturedElement {
        full_path: path,
        best_selector: best,
    })
}

/// Builds the flat best-effort selector from the last (target) path node.
pub(crate) fn build_best_selector(path: &[PathNode]) -> BestSelector {
    let target = path.last().expect("path is non-empty");
    BestSelector {
        control_type: target.control_type.clone(),
        name: target.name.clone(),
        class_name: target.class_name.clone(),
    automation_id: target.automation_id.clone(),
    }
}

/// Check whether a point lies within the element's bounding rectangle.
fn contains_point(element: &UIElement, x: i32, y: i32) -> bool {
    if let Ok(rect) = element.get_bounding_rectangle() {
        x >= rect.get_left()
            && x <= rect.get_right()
            && y >= rect.get_top()
            && y <= rect.get_bottom()
    } else {
        false
    }
}

/// Walk down the UIA control-view tree to find the deepest element
/// that contains the point. Uses bounding rectangles to navigate,
/// avoiding `element_from_point` which can return the desktop root
/// depending on COM threading state.
fn find_deepest_at_point(
    mut current: UIElement,
    walker: &UITreeWalker,
    x: i32,
    y: i32,
) -> Result<UIElement, String> {
    loop {
        let mut child = match walker.get_first_child(&current) {
            Ok(c) => c,
            Err(_) => return Ok(current),
        };

        let mut found = false;
        loop {
            if contains_point(&child, x, y) {
                current = child;
                found = true;
                break;
            }
            match walker.get_next_sibling(&child) {
                Ok(next) => child = next,
                Err(_) => break,
            }
        }

        if !found {
            return Ok(current);
        }
    }
}
