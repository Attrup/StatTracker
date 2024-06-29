use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use super::input::*;
use crate::setup::fonts;

// Set the minimum refresh rate of the app in Hz
const REFRESH_RATE: usize = 5;

pub struct GUI {
    timer: Arc<Mutex<String>>,
    size: Arc<Mutex<f32>>,
    _input_thread: thread::JoinHandle<()>,
}

impl GUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::load_monospace_font(&cc.egui_ctx);

        // Create shared variables for the timer and size
        let timer = Arc::new(Mutex::new("00:00".to_string()));
        let size = Arc::new(Mutex::new(40.0));

        GUI {
            timer: timer.clone(),
            size: size.clone(),
            _input_thread: thread::spawn(move || thread_reader(timer.clone(), size.clone())),
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
            // Display the timer with the specified size
            ui.label(
                egui::RichText::new(self.timer.lock().unwrap().as_str())
                    .size(*self.size.lock().unwrap())
                    .monospace(),
            )
        });

        // Force refresh of the app at the defined rate
        ctx.request_repaint_after(Duration::from_millis((1000 / REFRESH_RATE) as u64))
    }
}
