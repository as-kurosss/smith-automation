// crates/smith-windows/src/tools/mod.rs
#[cfg(windows)]
pub mod click;

#[cfg(windows)]
pub use click::ClickTool;
