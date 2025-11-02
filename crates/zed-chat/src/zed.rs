// Zed module - contains workspace and window handling code

pub mod app_menus;
pub mod component_preview;
pub mod edit_prediction_registry;
#[cfg(target_os = "macos")]
pub(crate) mod mac_only_instance;
pub mod migrate;
pub mod open_listener;
pub mod quick_action_bar;
#[cfg(target_os = "windows")]
pub(crate) mod windows_only_instance;

// Re-exports
pub use app_menus::*;
pub use edit_prediction_registry::*;
pub use open_listener::*;
