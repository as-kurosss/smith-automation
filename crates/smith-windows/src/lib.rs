// crates/smith-windows/src/lib.rs
pub mod tools;

#[cfg(windows)]
pub mod element;

#[cfg(windows)]
pub use element::SafeUIElement;
#[cfg(windows)]
pub use tools::ClickTool;
