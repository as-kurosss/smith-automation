# Community 12: read_node()

**Members:** 13

## Nodes

- **capture** (`apps_selector_capture_src_capture_rs`, File, degree: 12)
- **build_best_selector()** (`apps_selector_capture_src_capture_rs_build_best_selector`, Function, degree: 3)
- **capture_at_point()** (`apps_selector_capture_src_capture_rs_capture_at_point`, Function, degree: 4)
- **capture_focused_element()** (`apps_selector_capture_src_capture_rs_capture_focused_element`, Function, degree: 3)
- **contains_point()** (`apps_selector_capture_src_capture_rs_contains_point`, Function, degree: 2)
- **cursor_position()** (`apps_selector_capture_src_capture_rs_cursor_position`, Function, degree: 1)
- **find_deepest_at_point()** (`apps_selector_capture_src_capture_rs_find_deepest_at_point`, Function, degree: 3)
- **crate::types::{BestSelector, CapturedElement, PathNode}** (`apps_selector_capture_src_capture_rs_import_crate_types_bestselector_capturedelement_pathnode`, Module, degree: 1)
- **uiautomation::core::UIAutomation** (`apps_selector_capture_src_capture_rs_import_uiautomation_core_uiautomation`, Module, degree: 1)
- **uiautomation::core::{UIElement, UITreeWalker}** (`apps_selector_capture_src_capture_rs_import_uiautomation_core_uielement_uitreewalker`, Module, degree: 1)
- **uiautomation::types::ControlType** (`apps_selector_capture_src_capture_rs_import_uiautomation_types_controltype`, Module, degree: 1)
- **windows::Win32::UI::WindowsAndMessaging::GetCursorPos** (`apps_selector_capture_src_capture_rs_import_windows_win32_ui_windowsandmessaging_getcursorpos`, Module, degree: 1)
- **read_node()** (`apps_selector_capture_src_capture_rs_read_node`, Function, degree: 3)

## Relationships

- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_import_uiautomation_core_uiautomation (imports)
- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_import_uiautomation_core_uielement_uitreewalker (imports)
- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_import_uiautomation_types_controltype (imports)
- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_import_windows_win32_ui_windowsandmessaging_getcursorpos (imports)
- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_import_crate_types_bestselector_capturedelement_pathnode (imports)
- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_cursor_position (defines)
- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_capture_at_point (defines)
- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_read_node (defines)
- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_capture_focused_element (defines)
- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_build_best_selector (defines)
- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_contains_point (defines)
- apps_selector_capture_src_capture_rs → apps_selector_capture_src_capture_rs_find_deepest_at_point (defines)
- apps_selector_capture_src_capture_rs_capture_at_point → apps_selector_capture_src_capture_rs_find_deepest_at_point (calls)
- apps_selector_capture_src_capture_rs_capture_at_point → apps_selector_capture_src_capture_rs_read_node (calls)
- apps_selector_capture_src_capture_rs_capture_at_point → apps_selector_capture_src_capture_rs_build_best_selector (calls)
- apps_selector_capture_src_capture_rs_capture_focused_element → apps_selector_capture_src_capture_rs_read_node (calls)
- apps_selector_capture_src_capture_rs_capture_focused_element → apps_selector_capture_src_capture_rs_build_best_selector (calls)
- apps_selector_capture_src_capture_rs_find_deepest_at_point → apps_selector_capture_src_capture_rs_contains_point (calls)

