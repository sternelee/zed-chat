// Zed Chat - Standalone Agent Chat Application
// Based on Zed editor, with editor/file/terminal components removed
// Keeping: Agent UI, Workspace infrastructure, Settings, Theme

mod reliability;
mod zed;
pub mod zed_base;

pub use zed_base::{handle_open_request, restorable_workspace_locations};

pub fn main() {
    zed_base::main();
}
