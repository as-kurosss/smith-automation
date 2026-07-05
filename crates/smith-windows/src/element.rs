// crates/smith-windows/src/element.rs
use std::sync::Arc;
use uiautomation::UIElement;

/// Thread-safe wrapper over `UIElement`.
///
/// # Safety
///
/// `UIElement` internally contains `NonNull<c_void>` (a COM interface), which
/// does not implement `Send`. However, UI Automation objects are
/// free-threaded and can be safely transferred between threads.
/// All mutations (clicks, input) are performed via `spawn_blocking`,
/// where COM calls happen on a dedicated thread.
#[derive(Debug)]
pub struct SafeUIElement(Arc<UIElement>);

impl SafeUIElement {
    /// Creates a new thread-safe wrapper.
    #[must_use]
    pub fn new(element: UIElement) -> Self {
        // SAFETY: UIElement is a free-threaded COM object despite lacking
        // Send/Sync markers. Thread safety is enforced by restricting all
        // mutating operations to spawn_blocking contexts.
        #[allow(clippy::arc_with_non_send_sync)]
        Self(Arc::new(element))
    }

    /// Returns a reference to the inner element.
    #[must_use]
    pub fn inner(&self) -> &UIElement {
        &self.0
    }
}

// SAFETY: UI Automation elements are free-threaded COM objects.
// They can be safely sent between threads. All mutating operations
// are performed inside spawn_blocking to avoid blocking the async runtime.
unsafe impl Send for SafeUIElement {}
unsafe impl Sync for SafeUIElement {}

impl Clone for SafeUIElement {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
