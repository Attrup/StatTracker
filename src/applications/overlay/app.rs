use crate::setup::fonts;

pub struct GUI {}

impl GUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::load_monospace_font(&cc.egui_ctx);
        GUI {}
    }
}

impl eframe::App for GUI {
    /// Override the clear color to make the window itself transparent
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Transparent frame to make the 'CentralPanel' background transparent
        let frame = egui::containers::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };

        // Create the central panel container with a transparent background
        egui::CentralPanel::default().frame(frame).show(&ctx, |ui| {
            ui.label(egui::RichText::new("03:18").size(40.0).monospace())
        });
    }
}
