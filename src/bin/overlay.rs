#![windows_subsystem = "windows"] // Removes console when running on Windows

use eframe::egui::{Style, ViewportBuilder, Visuals};
use egui::Pos2;
use stattracker::applications::overlay::app::GUI;
use stattracker::system_access::system::get_game_window;
use std::env;

// Define window size
const WIDTH: f32 = 150.0;
const HEIGHT: f32 = 60.0;

fn main() -> eframe::Result<()> {
    // Retrieve PID of target process from command line arguments
    let args: Vec<String> = env::args().collect();
    let window_pos = match get_game_window(args) {
        // Center the overlay window on top of the game window
        Some((left, top, right, _)) => Pos2 {
            x: ((right - left) / 2.0) - (WIDTH / 2.0),
            y: top,
        },
        // Default to top left corner of the screen
        None => Pos2 { x: 0.0, y: 0.0 },
    };

    // Define app settings
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([WIDTH, HEIGHT])
            .with_resizable(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_position(window_pos)
            .with_decorations(true)
            .with_mouse_passthrough(false),
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
