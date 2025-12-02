# Zed Chat - Standalone GPUI Application

A standalone GPUI application extracted from Zed's agent chat functionality.

## Overview

This is an independent GPUI application that demonstrates the agent chat UI components from the Zed editor as a separate, standalone application. It serves as a foundation for building a dedicated chat interface using GPUI.

## Features

- **Standalone Application**: Runs independently from the main Zed editor
- **GPUI-based UI**: Built using the GPUI framework for cross-platform support
- **Agent Chat Foundation**: Contains the infrastructure for agent chat functionality
- **Minimal Dependencies**: Streamlined dependency set for faster builds

## Building

### Prerequisites

On Linux, you need to install system dependencies first:

```bash
# Ubuntu/Debian
sudo apt-get install -y libasound2-dev libfontconfig-dev libwayland-dev \
    libx11-xcb-dev libxkbcommon-x11-dev libvulkan1

# Or run the full dependency installation script from the root directory
./script/linux
```

### Compilation

```bash
# From the repository root
cargo build -p zed-chat --release
```

The compiled binary will be available at `target/release/zed-chat`.

## Running

```bash
# From the repository root
cargo run -p zed-chat --release
```

Or run the compiled binary directly:

```bash
./target/release/zed-chat
```

## Architecture

The application is structured as follows:

- **Main Entry**: `src/main.rs` - Application initialization and window creation
- **Assets**: Shared with Zed for fonts and themes
- **Dependencies**: Minimal set focused on GPUI and chat functionality

## Current Status

This version provides a working standalone GPUI application foundation:

1. ✅ Standalone GPUI application structure
2. ✅ Window creation and basic UI
3. ✅ Font and theme loading
4. ✅ HTTP client initialization
5. ✅ Settings system integration
6. 📋 Full AgentPanel integration (architectural challenge - see below)

## AgentPanel Integration Challenge

Integrating the full `AgentPanel` from `agent_ui` requires significant architectural changes:

### Required Components

The AgentPanel requires a full Workspace environment with:
- **UserStore**: User authentication and state
- **WorkspaceStore**: Workspace management
- **Project**: Language server, file system integration
- **AppSession**: Session management
- **LanguageRegistry**: Language support
- **NodeRuntime**: JavaScript/TypeScript tooling

### GPUI Context Challenge

GPUI entities must be created in specific contexts:
- **App context**: Global initialization (settings, themes, HTTP client)
- **Window context**: Entity creation (UserStore, Project, Workspace)
- Entities cannot be created in App context and passed to windows

### Integration Path

To integrate the full AgentPanel:

1. **Restructure initialization**: Move entity creation into window closure
2. **Add dependencies**: workspace, project, client, session, db (~25 additional crates)
3. **Implement entity lifecycle**: Proper async initialization and state management
4. **Setup Workspace**: Create and configure Workspace with all required services
5. **Load AgentPanel**: Async panel loading and attachment to Workspace

Example initialization structure:
```rust
cx.open_window(|window, cx| {
    // Create entities in window context
    let user_store = cx.new(|cx| UserStore::new(client, cx));
    let project = cx.new(|cx| Project::local(...));
    let workspace = cx.new(|cx| Workspace::new(...));
    
    // Load AgentPanel asynchronously
    cx.spawn(async move |cx| {
        let panel = AgentPanel::load(workspace, prompt_builder, cx).await?;
        workspace.update(cx, |ws, cx| ws.add_panel(panel, cx));
    });
})
```

### Current Implementation Decision

The current version provides:
- ✅ **Working foundation**: Demonstrates standalone GPUI app structure
- ✅ **Minimal complexity**: Easy to understand and modify
- ✅ **Build ready**: Compiles and runs successfully
- 📝 **Integration roadmap**: Clear path for full AgentPanel integration

This approach allows:
1. **Immediate use**: Application works and can be extended
2. **Learning tool**: Shows GPUI application basics
3. **Future expansion**: Foundation ready for full integration when architectural requirements are met

## Next Steps

### Short-term (Current Foundation)
- Customize the ChatWindow UI
- Add basic chat message display
- Implement simple user interactions

### Long-term (Full Integration)
1. Implement window-context entity creation
2. Add all required dependencies
3. Set up complete Workspace environment
4. Integrate full AgentPanel with async loading
5. Add language model and context management

## Development

To modify or extend this application:

1. The main window UI is defined in the `ChatWindow` struct
2. Initialization logic is in the `initialize_app` function
3. Dependencies are managed in `Cargo.toml`

## License

This project inherits the GPL-3.0-or-later license from the Zed project.
