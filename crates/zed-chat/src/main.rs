use anyhow::Result;
use assets::Assets;
use gpui::{App, Application, Bounds, Entity, WindowBounds, WindowOptions, px, size, colors::{Colors, GlobalColors}};
use log::LevelFilter;
use simplelog::SimpleLogger;
use std::sync::Arc;
use workspace::Workspace;
use agent_ui::AgentPanel;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default())
        .expect("could not initialize logger");

    Application::new().with_assets(Assets).run(|cx| {
        if let Err(err) = initialize_app(cx) {
            eprintln!("Failed to initialize application: {}", err);
            std::process::exit(1);
        }
    });
}

fn initialize_app(cx: &mut App) -> Result<()> {
    // Load embedded fonts
    load_embedded_fonts(cx)?;

    // Set global colors
    cx.set_global(GlobalColors(Arc::new(Colors::default())));

    // Initialize HTTP client
    let http_client = reqwest_client::ReqwestClient::user_agent("zed_chat")?;
    cx.set_http_client(Arc::new(http_client));

    // Initialize core services
    settings::init(cx);
    theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);
    language::init(cx);
    client::init_settings(cx);
    language_model::init(cx);
    editor::init(cx);
    project::Project::init_settings(cx);
    workspace::init_settings(cx);
    
    // Initialize agent-specific components
    agent_ui::init(cx);

    // Set up release channel
    release_channel::init(
        release_channel::AppVersion::init(env!("CARGO_PKG_VERSION")),
        cx,
    );

    // Create the main window
    let size = size(px(1200.), px(800.));
    let bounds = Bounds::centered(None, size, cx);
    
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("Zed Chat".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            window_min_size: Some(size(px(640.), px(480.))),
            ..Default::default()
        },
        |window, cx| {
            theme::setup_ui_font(window, cx);
            create_workspace(window, cx)
        },
    )?;

    cx.activate(true);
    Ok(())
}

fn create_workspace(
    window: &mut gpui::Window,
    cx: &mut gpui::App,
) -> Entity<Workspace> {
    // Create necessary services
    let fs = Arc::new(fs::RealFs::new(
        smol::block_on(async {
            dirs::ProjectDirs::from("dev", "zed", "ZedChat")
                .expect("Could not determine Zed Chat data directory")
        }),
        None,
    ));

    let client = client::Client::production(cx);
    let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
    
    let node_runtime = Arc::new(node_runtime::NodeRuntime::unavailable());
    
    let language_registry = Arc::new(language::LanguageRegistry::new(
        cx.background_spawn_executor().clone(),
    ));

    let project = cx.new(|cx| {
        project::Project::local(
            client.clone(),
            node_runtime.clone(),
            user_store.clone(),
            language_registry.clone(),
            fs.clone(),
            cx,
        )
    });

    let app_state = Arc::new(workspace::AppState {
        languages: language_registry.clone(),
        client: client.clone(),
        user_store: user_store.clone(),
        fs: fs.clone(),
        build_window_options: |_, _| Default::default(),
        node_runtime: node_runtime.clone(),
    });

    cx.new(|cx| {
        let workspace = Workspace::new(None, project, app_state, window, cx);
        workspace
    })
}

fn load_embedded_fonts(cx: &App) -> Result<()> {
    let font_paths = cx.asset_source().list("fonts")?;
    let mut embedded_fonts = Vec::new();
    for font_path in font_paths {
        if font_path.ends_with(".ttf") {
            if let Some(font_bytes) = cx.asset_source().load(&font_path)? {
                embedded_fonts.push(font_bytes);
            }
        }
    }
    cx.text_system().add_fonts(embedded_fonts)
}
