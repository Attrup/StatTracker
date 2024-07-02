#![windows_subsystem = "windows"] // Removes console when running on Windows

use eframe::egui::ViewportBuilder;
use stattracker::app::main::GUI;

// Define initial window size
const WIDTH: f32 = 270.0;
const HEIGHT: f32 = 330.0;

/// Run the application
fn main() -> eframe::Result<(), eframe::Error> {
    // Set up the window for the application
    eframe::run_native(
        "Hitman StatTracker",
        eframe::NativeOptions {
            viewport: ViewportBuilder::default()
                .with_inner_size([WIDTH, HEIGHT])
                .with_min_inner_size([WIDTH, HEIGHT]),
            run_and_return: false,
            ..Default::default()
        },
        // Initialize the GUI
        Box::new(|cc| Box::new(GUI::new(cc))),
    )
}
