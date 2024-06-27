use super::misc::setup_fonts;
use crate::{system_access::system::*, GameData};
use egui::*;
use std::time::Duration;
use sysinfo::System;

// Set the minimum refresh rate of the app in Hz
const REFRESH_RATE: usize = 30;

pub struct GUI {
    state: State,
    sys: System,
}

enum State {
    Game(Box<dyn GameData>),
    Waiting,
}

impl GUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        Self {
            state: State::Waiting,
            sys: System::new(),
        }
    }
}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // If game is running, update game stored game data, otherwise check if game
        // is running and update state accordingly
        match self.state {
            State::Waiting => {
                display_no_game(ctx);
                if let Some(game) = get_game(&mut self.sys) {
                    self.state = State::Game(game);
                }
            }
            State::Game(ref mut game) => {
                if let Some(game_data) = game.update() {
                    display_game_data(ctx, game_data);
                } else {
                    self.state = State::Waiting;
                    display_no_game(ctx);
                }
            }
        }

        // Force refresh of the app at the defined rate
        ctx.request_repaint_after(Duration::from_millis((1000 / REFRESH_RATE) as u64))
    }
}

/// Draw GUI for the application when a game is running
fn display_game_data(ctx: &egui::Context, data: (&str, u32, Option<([u32; 8], bool)>)) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // Mission Title / Game Status
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new(data.0).size(20.0));

            ui.separator();

            // Mission Time
            ui.add_space(4.0);

            if ui
                .add(
                    Button::new(
                        egui::RichText::new(format!(
                            "{:0>2}:{:0>2}.{:0>2}",
                            data.1 / 3600,
                            (data.1 / 60) % 60,
                            ((100 / 60) * (data.1 % 60)) as usize,
                        ))
                        .size(30.0)
                        .monospace(),
                    )
                    .frame(false),
                )
                .clicked()
            {
                match ui.visuals().dark_mode {
                    true => ctx.set_visuals(Visuals::light()),
                    false => ctx.set_visuals(Visuals::dark()),
                }
            };

            ui.add_space(4.0);

            // Mission Stats
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                egui::Grid::new("Stats")
                    .num_columns(2)
                    .spacing([20.0, 2.0])
                    .show(ui, |ui| {
                        if let Some(stats) = data.2 {
                            format_stats(ui, stats.0);
                        } else {
                            format_stats(ui, [0; 8]);
                        }
                    });
            });

            // Mission Rating
            ui.add_space(6.0);

            if let Some(stats) = data.2 {
                ui.label(
                    egui::RichText::new("SILENT ASSASSIN")
                        .size(25.0)
                        .monospace()
                        .color(if stats.1 {
                            egui::Color32::from_rgb(0, 160, 0)
                        } else {
                            egui::Color32::RED
                        }),
                );
            }
        });
    });
}

/// Draw GUI for the application while waiting for a compatible game to launch
fn display_no_game(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(12.0);
            if ui
                .add(Button::new(egui::RichText::new("Hitman StatTracker").size(25.0)).frame(false))
                .clicked()
            {
                match ui.visuals().dark_mode {
                    true => ctx.set_visuals(Visuals::light()),
                    false => ctx.set_visuals(Visuals::dark()),
                }
            };

            ui.add_space(30.0);
            ui.label(
                egui::RichText::new("Launch Hitman 2 SA or \nHitman Contracts to show stats")
                    .size(16.0),
            );
        });
    });
}

/// Display the current game stats in the defined order
fn format_stats(ui: &mut Ui, stats: [u32; 8]) {
    format_stat(ui, stats[0], "Shots Fired");
    format_stat(ui, stats[1], "Close Encounters");
    format_stat(ui, stats[2], "Headshots");
    format_stat(ui, stats[3], "Alerts");
    format_stat(ui, stats[4], "Enemies Killed");
    format_stat(ui, stats[5], "Enemies Harmed");
    format_stat(ui, stats[6], "Innocents Killed");
    format_stat(ui, stats[7], "Innocents Harmed");
}

/// Format the UI of a single stat
fn format_stat(ui: &mut Ui, value: u32, name: &str) {
    ui.label(
        egui::RichText::new(format!("{: >5}", value))
            .size(18.0)
            .monospace(),
    );
    ui.label(egui::RichText::new(name).size(18.0));
    ui.end_row();
}
