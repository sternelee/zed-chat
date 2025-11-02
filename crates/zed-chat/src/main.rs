// Zed Chat - Standalone Agent Chat Application
// Based on Zed editor, with editor/file/terminal components removed
// Keeping: Agent UI, Workspace infrastructure, Settings, Theme

mod reliability;
mod zed;
mod zed_base;

pub fn main() {
    zed_base::main();
}
