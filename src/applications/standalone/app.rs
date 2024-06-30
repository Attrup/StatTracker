use crate::{
    setup::{colors::ColorMap, fonts},
    system_access::system::*,
    GameData,
};
use egui::*;
use std::time::Duration;
use sysinfo::System;

// Set the minimum refresh rate of the app in Hz
const REFRESH_RATE: usize = 30;

pub struct GUI {
    state: State,
    sys: System,
    color_map: ColorMap,
}

enum State {
    Game(Box<dyn GameData>),
    Settings,
    Waiting,
}

impl GUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::load_monospace_font(&cc.egui_ctx);
        Self {
            state: State::Settings,
            sys: System::new(),
            color_map: ColorMap::gr_cmap(),
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
                    display_game_data(ctx, game_data, &self.color_map);
                } else {
                    self.state = State::Waiting;
                    display_no_game(ctx);
                }
            }
            State::Settings => {
                display_settings(&mut self.color_map, ctx);
            }
        }

        // Force refresh of the app at the defined rate
        ctx.request_repaint_after(Duration::from_millis((1000 / REFRESH_RATE) as u64))
    }
}

/// Draw GUI for the application when a game is running
fn display_game_data(
    ctx: &egui::Context,
    data: (&str, u32, Option<([u32; 8], bool)>),
    color_map: &ColorMap,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // Mission Title / Game Status
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new(data.0).size(20.0));

            ui.separator();

            // Mission Time
            ui.add_space(4.0);

            ui.label(
                egui::RichText::new(format!(
                    "{:0>2}:{:0>2}.{:0>2}",
                    data.1 / 3600,
                    (data.1 / 60) % 60,
                    ((100 / 60) * (data.1 % 60)) as usize,
                ))
                .size(30.0)
                .monospace(),
            );

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
                            color_map.get_sa_true()
                        } else {
                            color_map.get_sa_false()
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
            // Heading
            ui.add_space(12.0);
            ui.label(egui::RichText::new("Hitman StatTracker").size(25.0));

            // Subheading
            ui.add_space(30.0);
            ui.label(
                egui::RichText::new("Launch Hitman 2 SA or \nHitman Contracts to show stats")
                    .size(16.0),
            );
        });
    });
}

/// Display the settings menu
fn display_settings(color_map: &mut ColorMap, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            // Heading
            ui.heading(egui::RichText::new("Settings").size(20.0));
            ui.separator();
            ui.add_space(4.0);
        });

        // Create grid for all the settings
        egui::Grid::new("Settings")
            .num_columns(2)
            .spacing([25.0, 5.0])
            .show(ui, |ui| {
                // Theme Toggle
                ui.add(egui::Label::new("Theme"));
                theme_toggle(ui, ctx);
                ui.end_row();

                // Color map selector
                ui.add(egui::Label::new("Rating Colors"));
                egui::ComboBox::from_label("")
                    .selected_text(format!("{}", color_map.get_label()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(color_map, ColorMap::gr_cmap(), "Green / Red");
                        ui.selectable_value(color_map, ColorMap::br_cmap(), "Blue / Red");
                        ui.selectable_value(color_map, ColorMap::bo_cmap(), "Blue / Orange");
                        ui.selectable_value(color_map, ColorMap::bb_cmap(), "Blue / Brown");
                        ui.selectable_value(color_map, ColorMap::mk_cmap(), "Mint / Khaki");
                    });
                ui.end_row();
            });

        // Draw color map preview
        ui.separator();
        ui.spacing();
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("SA Rating")
                    .size(15.0)
                    .color(color_map.get_sa_true()),
            );
            ui.add_space(30.0);
            ui.label(
                egui::RichText::new("Other Rating")
                    .size(15.0)
                    .color(color_map.get_sa_false()),
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

/// Creates two buttons that allow the user to switch between light and dark mode
fn theme_toggle(ui: &mut Ui, ctx: &egui::Context) {
    let colors = if !ui.visuals().dark_mode {
        (Color32::LIGHT_BLUE, Color32::TRANSPARENT)
    } else {
        (Color32::TRANSPARENT, Color32::from_rgb(0, 102, 204))
    };

    ui.horizontal(|ui| {
        if ui
            .add(Button::new(egui::RichText::new("Light")).fill(colors.0))
            .clicked()
        {
            ctx.set_visuals(Visuals::light())
        };

        if ui
            .add(Button::new(egui::RichText::new("Dark")).fill(colors.1))
            .clicked()
        {
            ctx.set_visuals(Visuals::dark())
        };
    });
}
