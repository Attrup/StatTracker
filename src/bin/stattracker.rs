#![windows_subsystem = "windows"] // Removes console when running on Windows

use eframe::egui::{IconData, ViewportBuilder};
use stattracker::app::main::App;

// Define Window size
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
                .with_min_inner_size([WIDTH, HEIGHT])
                .with_resizable(false)
                .with_icon(load_icon()),
            run_and_return: false,
            ..Default::default()
        },
        // Launch the GUI
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

/// load the icon for the application
pub fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../../assets/icon.ico");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}