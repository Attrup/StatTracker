use super::overlay::draw_overlay;
use super::{colors::ColorMap, fonts, system_access::get_game};
use crate::{Backend, GameData, MissionStats, Window};

use egui::*;
use std::time::Duration;
use sysinfo::System;

// Set the minimum refresh rate of the app in Hz
// Note: Refresh rate will increase if the cursor is moved around while the window is in focus
const RUNNING_REFRESH_RATE: usize = 30;
const WAITING_REFRESH_RATE: usize = 1;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
/// Application struct that holds the GUI state and required data
/// The user settings are made persistent using Serde to keep the settings
/// consistent between application sessions.
pub struct App {
    // Application data (Not persistent)
    #[serde(skip)]
    state: State,

    #[serde(skip)]
    game: Option<Box<dyn Backend>>,

    #[serde(skip)]
    game_window: Option<Window>,

    #[serde(skip)]
    sys: System,

    // User settings (Persistent)
    cmap: ColorMap,
    show_overlay: bool,
    overlay_size: u8,
    theme: Visuals,
}

/// Enum to track the different states of the application
/// Each state has a corresponding GUI layout that is displayed
enum State {
    Running,
    Settings,
    Waiting,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: State::Waiting,
            game: None,
            game_window: None,
            sys: System::new(),
            cmap: ColorMap::default(),
            show_overlay: false,
            overlay_size: 5,
            theme: Visuals::dark(),
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::load_monospace_font(&cc.egui_ctx);

        // Load the user settings from the previous session if they exist
        if let Some(storage) = cc.storage {
            let app_data: App = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            // Theme is not saved by egui itself so we set it manually at launch
            cc.egui_ctx.set_visuals(app_data.theme.clone());

            return app_data;
        }

        // If no settings are found, use the default settings
        Default::default()
    }
}

/// Implementation of the eframe::App trait to allow for easy creation of the App
/// and its GUI using the eframe crate.
impl eframe::App for App {
    /// Save the user settings to the storage on application close
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Draw the GUI for the application
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // If game is running, update game stored game data, otherwise check if game
        // is running and update state accordingly
        match self.state {
            State::Waiting => {
                display_no_game(ctx, &mut self.state);
                if let Some(game) = get_game(&mut self.sys) {
                    self.game = Some(game);
                    self.state = State::Running;
                }

                // Force redraw of the GUI
                ctx.request_repaint_after(Duration::from_millis(
                    (1000 / WAITING_REFRESH_RATE) as u64,
                ))
            }
            State::Running => {
                match self.game.as_mut().unwrap().update() {
                    Some(game_data) => {
                        display_game_data(ctx, &game_data, &mut self.state, &self.cmap);

                        // Draw the overlay if enabled
                        if self.show_overlay {
                            draw_overlay(
                                ctx,
                                &self.cmap,
                                &self.game_window,
                                &self.overlay_size,
                                &game_data.mission_time,
                                &game_data.rating.map_or(true, |r| r.sa_rating),
                            );
                        }
                    }
                    None => {
                        self.state = State::Waiting;
                        self.game_window = None;
                        display_no_game(ctx, &mut self.state);
                    }
                }

                // If game is running but window is not set, set the window
                if self.game_window.is_none() {
                    self.game_window = self.game.as_ref().unwrap().game_window();
                }

                // Force redraw of the GUI
                ctx.request_repaint_after(Duration::from_millis(
                    (1000 / RUNNING_REFRESH_RATE) as u64,
                ))
            }

            State::Settings => {
                display_settings(
                    ctx,
                    &mut self.cmap,
                    &mut self.show_overlay,
                    &mut self.overlay_size,
                    &mut self.state,
                    &mut self.theme,
                );

                // Draw the overlay if enabled
                if self.show_overlay {
                    draw_overlay(
                        ctx,
                        &self.cmap,
                        &self.game_window,
                        &self.overlay_size,
                        &0,
                        &true,
                    );
                }
            }
        }
    }
}

/// Draw GUI for the application when a game is running
fn display_game_data(ctx: &egui::Context, data: &GameData, app_state: &mut State, cmap: &ColorMap) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // Mission Title / Game Status
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new(&data.mission_name).size(20.0));
            ui.separator();

            // Mission Time
            if ui
                .add(
                    Button::new(
                        egui::RichText::new(format!(
                            "{:0>2}:{:0>2}",
                            data.mission_time / 3600,
                            (data.mission_time / 60) % 60,
                        ))
                        .size(40.0)
                        .monospace(),
                    )
                    .frame(false),
                )
                .clicked()
            {
                *app_state = State::Settings;
            }

            // Mission Stats
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                egui::Grid::new("Stats")
                    .num_columns(2)
                    .spacing([20.0, 2.0])
                    .show(ui, |ui| {
                        if let Some(rating) = data.rating {
                            format_stats(ui, rating.stats);
                        } else {
                            format_stats(ui, MissionStats::default());
                        }
                    });
            });

            // Mission Rating
            ui.add_space(4.0);
            if let Some(stats) = data.rating {
                ui.label(
                    egui::RichText::new("SILENT ASSASSIN")
                        .size(25.0)
                        .monospace()
                        .color(cmap.get_rating_color(stats.sa_rating)),
                );
            }
        });
    });
}

/// Draw GUI for the application while waiting for a compatible game to launch
fn display_no_game(ctx: &egui::Context, app_state: &mut State) {
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

            // Open Settings Button
            ui.add_space(167.0);
            if ui
                .button(egui::RichText::new("Settings").size(15.0))
                .clicked()
            {
                *app_state = State::Settings;
            }
        });
    });
}

/// Display the settings menu
fn display_settings(
    ctx: &egui::Context,
    cmap: &mut ColorMap,
    show_overlay: &mut bool,
    overlay_size: &mut u8,
    app_state: &mut State,
    theme: &mut Visuals,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            // Heading
            ui.heading(egui::RichText::new("Settings").size(20.0));
            ui.separator();
        });

        // Create grid for all the settings
        egui::Grid::new("Settings")
            .num_columns(2)
            .spacing([25.0, 5.0])
            .show(ui, |ui| {
                // Theme Toggle
                ui.add(egui::Label::new("Theme"));
                theme_toggle(ui, ctx, theme);
                ui.end_row();

                // Color map selector
                ui.add(egui::Label::new("Rating Colors"));
                cmap_selector(ui, cmap);
                ui.end_row();

                // Use game overlay
                ui.add(egui::Label::new("Game Overlay"));
                ui.checkbox(show_overlay, "Enable");
                ui.end_row();

                // Text size of the overlay
                ui.add(egui::Label::new("Overlay Size"));

                ui.add(egui::Slider::new(overlay_size, 1..=10));
            });

        ui.vertical_centered(|ui| {
            // Color map preview
            ui.add_space(20.0);
            let mut rating_text = egui::text::LayoutJob::default();

            rating_text.append(
                "SA Rating",
                0.0,
                TextFormat {
                    font_id: FontId::proportional(18.0),
                    color: cmap.get_sa_true(),
                    ..Default::default()
                },
            );

            rating_text.append(
                "Other Rating",
                25.0,
                TextFormat {
                    font_id: FontId::proportional(18.0),
                    color: cmap.get_sa_false(),
                    ..Default::default()
                },
            );

            ui.label(rating_text);

            // About section
            ui.add_space(35.0);
            ui.hyperlink_to("GitHub", "https://github.com/Attrup/StatTracker");
            ui.label(format!("Hitman StatTracker v{}", env!("CARGO_PKG_VERSION")));
            ui.label("By Jonas Attrup");

            // Return to previous state button
            ui.add_space(25.0);
            if ui
                .button(egui::RichText::new("Exit Settings").size(15.0))
                .clicked()
            {
                *app_state = State::Waiting;
            }
        });
    });
}

/// Display the current game stats in the defined order
fn format_stats(ui: &mut Ui, stats: MissionStats) {
    format_stat(ui, stats.shots_fired, "Shots Fired");
    format_stat(ui, stats.close_encounters, "Close Encounters");
    format_stat(ui, stats.headshots, "Headshots");
    format_stat(ui, stats.alerts, "Alerts");
    format_stat(ui, stats.enemies_killed, "Enemies Killed");
    format_stat(ui, stats.enemies_harmed, "Enemies Harmed");
    format_stat(ui, stats.innocents_killed, "Innocents Killed");
    format_stat(ui, stats.innocents_harmed, "Innocents Harmed");
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
fn theme_toggle(ui: &mut Ui, ctx: &egui::Context, theme: &mut Visuals) {
    let colors = if !ui.visuals().dark_mode {
        (Color32::LIGHT_BLUE, Color32::TRANSPARENT)
    } else {
        (Color32::TRANSPARENT, Color32::from_rgb(0, 102, 204))
    };

    ui.horizontal(|ui| {
        // Set the theme to light mode
        if ui
            .add(Button::new(egui::RichText::new("Light")).fill(colors.0))
            .clicked()
        {
            ctx.set_visuals(Visuals::light());
            *theme = Visuals::light();
        };

        // Set the theme to dark mode
        if ui
            .add(Button::new(egui::RichText::new("Dark")).fill(colors.1))
            .clicked()
        {
            ctx.set_visuals(Visuals::dark());
            *theme = Visuals::dark();
        };
    });
}

/// Create color map dropdown menu
fn cmap_selector(ui: &mut Ui, cmap: &mut ColorMap) {
    egui::ComboBox::from_label("")
        .selected_text(format!("{}", cmap.get_label()))
        .show_ui(ui, |ui| {
            for map in ColorMap::all_cmaps() {
                ui.selectable_value(cmap, map.clone(), map.get_label());
            }
        });
}
