use crate::game::entities::armors::Armor;
use crate::game::entities::enemies::Challenge;
use crate::gui::app::AppView;
use crate::gui::options::{AIOptions, GameOptions};
use crate::gui::widgets::number_input::unsigned_number_input;
use egui::Ui;

pub struct SidePanelView {
    game_options: GameOptions,
    ai_options: AIOptions,
    view: AppView,
}

impl SidePanelView {
    pub fn new() -> Self {
        Self {
            game_options: GameOptions::default(),
            ai_options: AIOptions::default(),
            view: AppView::default(),
        }
    }

    pub fn get_options(&self) -> (GameOptions, AIOptions) {
        (self.game_options.clone(), self.ai_options.clone())
    }

    pub fn set_current_view(&mut self, view: AppView) {
        self.view = view;
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        ui.collapsing("Game Options", |ui| {
            ui.horizontal(|ui| {
                ui.label("Armor");
                egui::ComboBox::new("side_panel_armor_combo_box", "")
                    .selected_text(self.game_options.armor.to_string())
                    .show_ui(ui, |ui| {
                        for option in Armor::vec() {
                            ui.selectable_value(
                                &mut self.game_options.armor,
                                option,
                                option.to_string(),
                            );
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Challenge");
                egui::ComboBox::new("side_panel_challenge_combo_box", "")
                    .selected_text(self.game_options.challenge.to_string())
                    .show_ui(ui, |ui| {
                        for option in Challenge::vec() {
                            ui.selectable_value(
                                &mut self.game_options.challenge,
                                option,
                                option.to_string(),
                            );
                        }
                    });
            });

            ui.collapsing("Stats", |ui| {
                egui::Grid::new("side_panel_stats_grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        for stat in ["STR", "DEX", "INT", "CHA", "LUK", "END", "WIS"] {
                            ui.label(stat);

                            unsigned_number_input(
                                ui,
                                self.game_options.stats.map.get_mut(stat).unwrap(),
                                200,
                                0,
                            );

                            ui.end_row();
                        }
                    });
            });
        });

        if matches!(self.view, AppView::AIAppView) {
            ui.collapsing("AI Options", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Depth");
                    unsigned_number_input(ui, &mut self.ai_options.depth, 10, 2);
                });
            });
        }
    }
}
