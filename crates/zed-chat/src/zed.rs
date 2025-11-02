// Zed module - contains workspace and window handling code

pub mod app_menus;
// Removed editor-specific modules:
// - component_preview (development/testing feature)
// - edit_prediction_registry (AI code completion)
// - quick_action_bar (code execution UI)
#[cfg(target_os = "macos")]
pub(crate) mod mac_only_instance;
pub mod migrate;
pub mod open_listener;
#[cfg(target_os = "windows")]
pub(crate) mod windows_only_instance;

// Re-exports
pub use app_menus::*;
pub use open_listener::*;
