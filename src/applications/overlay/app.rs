use super::input::*;
use crate::setup::fonts;

pub struct GUI {
    timer: String,
    size: f32,
}

impl GUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::load_monospace_font(&cc.egui_ctx);
        GUI {
            timer: "00:00".to_string(),
            size: 40.0,
        }
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
            // Read and execute the next command from stdin
            match get_next_command() {
                Ok(Command::Time(minutes, seconds)) => {
                    self.timer = format!("{}:{}", minutes, seconds);
                }
                Ok(Command::Size(size)) => {
                    self.size = size;
                }
                _ => {}
            }

            ui.label(
                egui::RichText::new(self.timer.as_str())
                    .size(self.size)
                    .monospace(),
            )
        });
    }
}
