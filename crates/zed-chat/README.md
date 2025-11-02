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

This is the initial version that provides:

1. ✅ Standalone GPUI application structure
2. ✅ Window creation and basic UI
3. ✅ Font and theme loading
4. ✅ HTTP client initialization
5. ⏳ Full agent chat panel integration (to be completed)

## Next Steps

To complete the full agent chat functionality, the following items need to be integrated:

1. Complete workspace and project initialization
2. Integrate the full `AgentPanel` from `agent_ui`
3. Add language model configuration
4. Implement context management
5. Add slash command support

## Development

To modify or extend this application:

1. The main window UI is defined in the `ChatWindow` struct
2. Initialization logic is in the `initialize_app` function
3. Dependencies are managed in `Cargo.toml`

## License

This project inherits the GPL-3.0-or-later license from the Zed project.
