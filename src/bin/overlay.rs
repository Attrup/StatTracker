#![windows_subsystem = "windows"] // Removes console when running on Windows

use eframe::egui::ViewportBuilder;
use egui::Pos2;
use stattracker::applications::overlay::app::GUI;
use stattracker::system_access::system::get_game_window;
use std::env;

// Define window size
const WIDTH: f32 = 200.0;
const HEIGHT: f32 = 80.0;

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
        None => Pos2 { x: 500.0, y: 0.0 },
    };

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([WIDTH, HEIGHT])
            .with_min_inner_size([WIDTH, HEIGHT])
            .with_resizable(false)
            .with_transparent(true)
            .with_always_on_top()
            // .with_mouse_passthrough(true) Currently broken in winit, waiting on fix
            .with_decorations(true)
            .with_position(window_pos),
        run_and_return: false,
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Hitman StatTracker (Overlay)",
        native_options,
        Box::new(|cc| Box::new(GUI::new(cc))),
    )
}
