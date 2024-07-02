use super::colors::ColorMap;

pub fn draw_overlay(
    ctx: &egui::Context,
    cmap: &mut ColorMap,
    overlay_size: &u8,
    timer: &u32,
    sa_status: &bool,
) {
    // Create colored background frame depending on the current SA status
    let frame = egui::containers::Frame {
        fill: cmap.get_rating_color(*sa_status),
        ..Default::default()
    };

    ctx.show_viewport_immediate(
        // Build new overlay viewport
        egui::ViewportId::from_hash_of("overlay"),
        egui::ViewportBuilder::default()
            .with_title("StatTracker Overlay")
            .with_inner_size([(overlay_size * 22) as f32, (overlay_size * 9) as f32])
            .with_always_on_top()
            .with_transparent(true)
            .with_resizable(false)
            .with_mouse_passthrough(true)
            .with_decorations(false)
            .with_position([0.0, 0.0]),
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
