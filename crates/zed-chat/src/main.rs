use anyhow::Result;
use assets::Assets;
use gpui::{
    colors::{Colors, GlobalColors}, px, App, Application, Bounds, SharedString, WindowBounds,
    WindowOptions, div, prelude::*,
};
use log::LevelFilter;
use simplelog::SimpleLogger;
use std::sync::Arc;

const WELCOME_MESSAGE: &str = "Zed Chat - Standalone GPUI Application\n\n\
    This is a standalone GPUI application extracted from Zed's agent chat functionality.\n\n\
    Features:\n\
    - Independent GPUI application\n\
    - Agent chat UI components\n\
    - Separate from main Zed editor\n\n\
    The full agent UI integration requires additional initialization that would be \
    completed in a production version.";

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default())
        .expect("could not initialize logger");

    let app = Application::new().with_assets(Assets);
    app.run(|cx| {
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
    let user_agent = format!(
        "ZedChat/{} ({}; {})",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    let http = reqwest_client::ReqwestClient::user_agent(&user_agent)?;
    cx.set_http_client(Arc::new(http));

    // Initialize settings and theme
    settings::init(cx);
    theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);

    // Create the main window with a simple chat UI placeholder
    let size = gpui::size(px(1200.), px(800.));
    let bounds = Bounds::centered(None, size, cx);

    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("Zed Chat - Standalone Agent Chat Application".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            window_min_size: Some(gpui::size(px(640.), px(480.))),
            ..Default::default()
        },
        |window, cx| {
            theme::setup_ui_font(window, cx);
            cx.new(|cx| ChatWindow::new(cx))
        },
    )?;

    cx.activate(true);
    Ok(())
}

// Simple chat window placeholder
struct ChatWindow {
    message: SharedString,
}

impl ChatWindow {
    fn new(_cx: &mut gpui::Context<Self>) -> Self {
        Self {
            message: WELCOME_MESSAGE.into(),
        }
    }
}

impl gpui::Render for ChatWindow {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(gpui::rgb(0x1e1e1e))
            .text_color(gpui::rgb(0xcccccc))
            .p_4()
            .gap_2()
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child("Zed Chat")
            )
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .bg(gpui::rgb(0x252525))
                    .border_1()
                    .border_color(gpui::rgb(0x3e3e3e))
                    .rounded_lg()
                    .child(self.message.clone())
            )
    }
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
    cx.text_system()
        .add_fonts(embedded_fonts)
        .map_err(|e| anyhow::anyhow!("Failed to load fonts: {}", e))
}
