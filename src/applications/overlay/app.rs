use std::time::Duration;

use super::input::*;
use crate::setup::{colors::ColorMap, fonts};

// Set the minimum refresh rate of the app in Hz
const REFRESH_RATE: usize = 30;

pub struct GUI {
    colormap: ColorMap,
    sa_status: bool,
    timer: String,
    size: f32,
}

impl GUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::load_monospace_font(&cc.egui_ctx);

        GUI {
            colormap: ColorMap::from_label("GR"),
            sa_status: true,
            timer: String::new(),
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

        // Read next command from Stdin
        match get_next_command() {
            Ok(Command::Time(minutes, seconds, sa)) => {
                self.timer = format!("{}:{}", minutes, seconds);
                self.sa_status = sa;
            }
            Ok(Command::Size(timer_size)) => {
                self.size = timer_size;
            }
            Ok(Command::Color(color)) => {
                self.colormap = ColorMap::from_label(&color);
            }
            _ => {}
        }

        // Create the central panel container with a transparent background
        egui::CentralPanel::default().frame(frame).show(&ctx, |ui| {
            // Display the mission time in the center of the window
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(&self.timer)
                        .size(self.size)
                        .monospace()
                        .color(if self.sa_status {
                            self.colormap.get_sa_true()
                        } else {
                            self.colormap.get_sa_false()
                        }),
                )
            });
        });

        // Force refresh of the app at the defined rate
        ctx.request_repaint_after(Duration::from_millis((1000 / REFRESH_RATE) as u64))
    }
}
