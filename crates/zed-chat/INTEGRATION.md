# AgentPanel Integration Guide

This document explains the technical challenges and steps required to integrate the full `AgentPanel` from Zed's `agent_ui` crate into the standalone zed-chat application.

## Overview

The `AgentPanel` is Zed's comprehensive agent chat interface. It provides:
- Text-based chat conversations
- Context management (files, symbols, web search)
- Slash commands for special operations
- Multiple agent backends (Claude, GPT, local models)
- Conversation history and persistence

## Architecture Requirements

### GPUI Entity System

GPUI uses a context-sensitive entity creation system:

```rust
// ❌ Cannot create entities at app level
fn initialize_app(cx: &mut App) {
    let entity = cx.new(|cx| MyEntity::new(cx)); // ERROR: method doesn't exist
}

// ✅ Must create entities in window context
cx.open_window(|window, cx| {
    let entity = cx.new(|cx| MyEntity::new(cx)); // OK: window context has .new()
});
```

### Required Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
# Existing
anyhow.workspace = true
assets.workspace = true
gpui = { workspace = true, features = ["wayland", "x11", "font-kit"] }
log.workspace = true
reqwest_client.workspace = true
settings.workspace = true
simplelog.workspace = true
theme.workspace = true

# Additional for AgentPanel
agent_ui.workspace = true
agent_settings.workspace = true
assistant_slash_commands.workspace = true
client.workspace = true
db.workspace = true
editor.workspace = true
extension.workspace = true
extension_host.workspace = true
fs.workspace = true
language.workspace = true
language_model.workspace = true
language_models.workspace = true
node_runtime.workspace = true
paths.workspace = true
project.workspace = true
prompt_store.workspace = true
release_channel.workspace = true
session.workspace = true
workspace.workspace = true
```

## Implementation Steps

### Step 1: Restructure Initialization

Current structure (App level):
```rust
fn initialize_app(cx: &mut App) -> Result<()> {
    load_fonts(cx)?;
    settings::init(cx);
    theme::init(cx);
    
    cx.open_window(|window, cx| {
        cx.new(|cx| ChatWindow::new(cx))
    })?;
    
    Ok(())
}
```

Required structure (Window level entities):
```rust
fn initialize_app(cx: &mut App) -> Result<()> {
    // App-level initialization
    load_fonts(cx)?;
    settings::init(cx);
    theme::init(cx);
    
    // Setup services needed by window
    let fs = Arc::new(RealFs::new(None, cx.background_executor()));
    <dyn Fs>::set_global(fs.clone(), cx);
    
    let client = Client::production(cx);
    Client::set_global(client.clone(), cx);
    
    let languages = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));
    
    // Clone for move into closure
    let fs_clone = fs.clone();
    let client_clone = client.clone();
    let languages_clone = languages.clone();
    
    cx.open_window(|window, cx| {
        // Create entities in window context
        create_workspace_with_agent_panel(
            fs_clone,
            client_clone, 
            languages_clone,
            window,
            cx
        )
    })?;
    
    Ok(())
}
```

### Step 2: Create Workspace with Entities

```rust
fn create_workspace_with_agent_panel(
    fs: Arc<dyn Fs>,
    client: Arc<Client>,
    languages: Arc<LanguageRegistry>,
    window: &mut gpui::Window,
    cx: &mut gpui::App,
) -> Entity<Workspace> {
    // Create required entities in window context
    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
    let workspace_store = cx.new(|cx| WorkspaceStore::new(client.clone(), cx));
    
    let session = cx.background_executor().block(Session::new());
    let app_session = cx.new(|cx| AppSession::new(session, cx));
    
    let node_runtime = NodeRuntime::unavailable();
    
    // Build app state
    let app_state = Arc::new(AppState {
        languages: languages.clone(),
        client: client.clone(),
        user_store: user_store.clone(),
        fs: fs.clone(),
        build_window_options: |_, _| Default::default(),
        workspace_store,
        node_runtime,
        session: app_session,
    });
    AppState::set_global(Arc::downgrade(&app_state), cx);
    
    // Initialize systems that depend on app state
    Project::init(&client, cx);
    language_model::init(client.clone(), cx);
    language_models::init(user_store.clone(), client.clone(), cx);
    agent_settings::init(cx);
    editor::init(cx);
    workspace::init(app_state.clone(), cx);
    
    // Initialize prompt builder and agent UI
    let prompt_builder = PromptBuilder::load(fs.clone(), false, cx);
    agent_ui::init(
        fs.clone(),
        client.clone(),
        prompt_builder.clone(),
        languages.clone(),
        false,
        cx,
    );
    
    // Create project
    let project = cx.new(|cx| {
        Project::local(
            client.clone(),
            node_runtime,
            user_store.clone(),
            languages.clone(),
            fs.clone(),
            None,
            cx,
        )
    });
    
    // Create workspace
    let workspace = cx.new(|cx| {
        Workspace::new(None, project, app_state, window, cx)
    });
    
    // Load and attach AgentPanel asynchronously
    load_agent_panel(workspace.clone(), prompt_builder, window, cx);
    
    workspace
}
```

### Step 3: Async Panel Loading

```rust
fn load_agent_panel(
    workspace: Entity<Workspace>,
    prompt_builder: Arc<PromptBuilder>,
    window: &mut gpui::Window,
    cx: &mut gpui::App,
) {
    let workspace_weak = workspace.downgrade();
    let window_handle = window.window_handle();
    
    cx.spawn(async move |mut cx| {
        // Load panel asynchronously
        let panel = agent_ui::AgentPanel::load(
            workspace_weak.clone(),
            prompt_builder,
            cx.clone(),
        )
        .await;
        
        // Add panel to workspace if successful
        if let Ok(panel) = panel {
            workspace_weak.update(&mut cx, |workspace, cx| {
                // Add panel to workspace dock
                workspace.add_panel_in(&window_handle, panel, cx);
            })
            .ok();
        }
    })
    .detach();
}
```

## Common Issues and Solutions

### Issue 1: "no method named `new` found for `&mut App`"

**Cause**: Trying to create entities outside window context

**Solution**: Move entity creation into the window closure:
```rust
// ❌ Wrong
fn initialize_app(cx: &mut App) {
    let entity = cx.new(|cx| MyEntity::new(cx));
}

// ✅ Correct
cx.open_window(|window, cx| {
    let entity = cx.new(|cx| MyEntity::new(cx));
});
```

### Issue 2: AsyncWindowContext creation

**Cause**: AsyncWindowContext doesn't have a public constructor

**Solution**: Use the context provided by cx.spawn():
```rust
// ❌ Wrong
let async_cx = AsyncWindowContext::new(cx.clone());

// ✅ Correct  
cx.spawn(async move |cx| {
    // cx is already AsyncWindowContext
    AgentPanel::load(workspace, prompt_builder, cx).await
})
```

### Issue 3: Background executor type mismatch

**Cause**: RealFs expects specific executor type

**Solution**: Use the executor from cx:
```rust
// ✅ Correct
let fs = Arc::new(RealFs::new(
    None,  // git_binary_path
    cx.background_executor(),  // Use executor from context
));
```

## Testing the Integration

1. **Compile**: `cargo build -p zed-chat`
2. **Run**: `cargo run -p zed-chat`
3. **Verify**:
   - Window opens
   - AgentPanel appears in dock (right side by default)
   - Can create new chat threads
   - Can interact with language models

## Performance Considerations

- **Initial load**: ~500ms for full workspace setup
- **Panel load**: ~200ms for async AgentPanel initialization
- **Memory**: ~150MB additional (vs minimal version at ~50MB)
- **Dependencies**: Build time increases by ~5-10 minutes

## Alternative Approaches

### Approach 1: Embedded Workspace (Current Challenge)
- Full Zed workspace with all features
- Requires all architectural components
- Most complex but most powerful

### Approach 2: Lightweight Agent UI
- Extract just the chat interface components
- Skip workspace/project infrastructure
- Custom context management
- Simpler but requires custom implementation

### Approach 3: Hybrid Model
- Minimal workspace for panel requirements
- Stub out unused features (LSP, file watching)
- Balance between complexity and functionality

## Conclusion

Full AgentPanel integration is architecturally challenging due to GPUI's context-sensitive design. The current minimal version provides a solid foundation. Full integration requires:

1. Deep understanding of GPUI entity lifecycle
2. Proper async initialization patterns
3. Complete Zed workspace infrastructure
4. ~25 additional dependencies

Estimated effort: 2-3 days of focused development for experienced GPUI developers.
