#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

use eframe::egui::{Style, Visuals};
use stattracker::app::gui::GUI;

// Set initial window size
//const WIDTH: f32 = 270.0;
// const HEIGHT: f32 = 330.0;

/// Run the application
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Hitman StatTracker",
        eframe::NativeOptions {
            //min_window_size: Some(egui::Vec2::new(WIDTH, HEIGHT)),
            //initial_window_size: Some(egui::Vec2::new(WIDTH, HEIGHT)),
            run_and_return: false,
            ..eframe::NativeOptions::default()
        },
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
