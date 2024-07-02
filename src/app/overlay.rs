use crate::Window;

use super::colors::ColorMap;

pub fn draw_overlay(
    ctx: &egui::Context,
    cmap: &ColorMap,
    game_window: &Option<Window>,
    overlay_size: &u8,
    timer: &u32,
    sa_status: &bool,
) {
    // Create colored background frame depending on the current SA status
    let frame = egui::containers::Frame {
        fill: cmap.get_rating_color(*sa_status),
        ..Default::default()
    };

    // Calculate the overlay position
    let width = overlay_size * 22;
    let height = overlay_size * 9;

    let overlay_position = match game_window {
        Some(window) => {
            [
                (window.left + window.right - width as i32) as f32 * 0.5, // Center the overlay
                (window.top + 2) as f32, // The 2 account for the window frame
            ]
        }
        None => [0.0, 0.0], // Default to top left corner of the screen
    };

    ctx.show_viewport_immediate(
        // Build new overlay viewport
        egui::ViewportId::from_hash_of("overlay"),
        egui::ViewportBuilder::default()
            .with_title("StatTracker Overlay")
            .with_inner_size([width as f32, height as f32])
            .with_always_on_top()
            .with_transparent(true)
            .with_resizable(false)
            .with_mouse_passthrough(true)
            .with_decorations(false)
            .with_position(overlay_position),
        |ctx, _| {
            // Set the contents of the overlay viewport
            egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "{:0>2}:{:0>2}",
                            timer / 3600,
                            (timer / 60) % 60
                        ))
                        .size((overlay_size * 8) as f32)
                        .monospace()
                        .color(egui::Color32::WHITE),
                    )
                });
            });
        },
    );
}
