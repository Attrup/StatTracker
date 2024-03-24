#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

use eframe::egui::{Style, Vec2, ViewportBuilder, Visuals};
use stattracker::app::gui::GUI;

// Set initial window size
const WIDTH: f32 = 270.0;
const HEIGHT: f32 = 330.0;

/// Run the application
fn main() -> eframe::Result<(), eframe::Error> {
    // Set up the window for the application
    eframe::run_native(
        "Hitman StatTracker",
        eframe::NativeOptions {
            viewport: ViewportBuilder::default()
                .with_inner_size(Vec2::new(WIDTH, HEIGHT))
                .with_min_inner_size(Vec2::new(WIDTH, HEIGHT)),
            run_and_return: false,
            ..Default::default()
        },
        // Initialize the GUI
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
