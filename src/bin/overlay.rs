#![windows_subsystem = "windows"] // Removes console when running on Windows

use eframe::egui::{Style, ViewportBuilder, Visuals};
use stattracker::applications::overlay::app::GUI;

fn main() -> eframe::Result<()> {
    // Define app settings
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 100.0])
            .with_resizable(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_decorations(false)
            .with_mouse_passthrough(true),
        run_and_return: false,
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Hitman StatTracker (Overlay)",
        native_options,
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals::light(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            Box::new(GUI::new(cc))
        }),
    )
}
