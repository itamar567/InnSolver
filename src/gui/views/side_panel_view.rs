use crate::game::entities::armors::Armor;
use crate::game::entities::enemies::Challenge;
use crate::game::types::gear::Slot;
use crate::gui::app::AppView;
use crate::gui::gear::GearSet;
use crate::gui::options::{AIOptions, GameOptions, GearOptions};
use crate::gui::widgets::number_input::unsigned_number_input;
use egui::{Ui, Vec2};
use std::cell::RefCell;
use std::rc::Rc;

pub struct SidePanelView {
    game_options: Rc<RefCell<GameOptions>>,
    ai_options: Rc<RefCell<AIOptions>>,
    gear_options: Rc<RefCell<GearOptions>>,
    view: AppView,

    pub change_view: Option<AppView>,

    new_build_name: String,
}

impl SidePanelView {
    pub fn new(
        game_options: Rc<RefCell<GameOptions>>,
        ai_options: Rc<RefCell<AIOptions>>,
        gear_options: Rc<RefCell<GearOptions>>,
    ) -> Self {
        Self {
            game_options,
            ai_options,
            gear_options,
            view: AppView::default(),
            change_view: None,
            new_build_name: String::new(),
        }
    }

    pub fn set_current_view(&mut self, view: AppView) {
        self.view = view;
    }

    fn draw_builds(&mut self, ui: &mut Ui, enable_edit: bool) {
        let mut gear_options = self.gear_options.borrow_mut();

        if !gear_options.gear_sets.is_empty() {
            egui::Grid::new("side_panel_gear_sets_grid").show(ui, |ui| {
                for (index, set) in gear_options.gear_sets.clone().iter().enumerate() {
                    ui.label(&set.name);
                    if enable_edit && ui.button("Edit").clicked() {
                        gear_options.current_set_index = Some(index);
                        self.change_view = Some(AppView::Gear);
                    }
                    if ui.button("ｘ").clicked() {
                        gear_options.gear_sets.remove(index);

                        if let Some(current_set_index) = &mut gear_options.current_set_index {
                            if index <= *current_set_index {
                                if *current_set_index == 0 {
                                    gear_options.current_set_index = None;
                                } else {
                                    *current_set_index -= 1;
                                }
                            }
                        }
                    }
                    ui.end_row();
                }
            });
        }

        ui.horizontal(|ui| {
            ui.add_sized(
                Vec2::new(ui.available_width() * 5.0 / 6.0, ui.available_height()),
                egui::TextEdit::singleline(&mut self.new_build_name),
            );
            if ui
                .add_enabled(!self.new_build_name.is_empty(), egui::Button::new("+"))
                .clicked()
            {
                gear_options
                    .gear_sets
                    .push(GearSet::new(self.new_build_name.clone()));
                self.new_build_name.clear();
                gear_options.current_set_index = Some(gear_options.gear_sets.len() - 1);
            }
        });
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        if matches!(self.view, AppView::Gear) {
            ui.collapsing("Manage Builds", |ui| {
                self.draw_builds(ui, false);
            });

            let mut gear_options = self.gear_options.borrow_mut();

            if gear_options.gear_sets.is_empty() {
                ui.separator();
                ui.label("Create a new build to start using the Gear section");
                return;
            }

            if gear_options.current_set_index.is_none() {
                // The current set is invalid, replace it with the first set
                gear_options.current_set_index = Some(0);
            }

            egui::Grid::new("side_panel_gear_grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Build");
                    egui::ComboBox::new("side_panel_build_combo_box", "")
                        .selected_text(&gear_options.get_current_set().unwrap().name)
                        .show_ui(ui, |ui| {
                            for (index, set) in gear_options.gear_sets.clone().iter().enumerate() {
                                ui.selectable_value(
                                    gear_options.current_set_index.as_mut().unwrap(),
                                    index,
                                    &set.name,
                                );
                            }
                        });

                    ui.end_row();

                    ui.label("Slot");
                    egui::ComboBox::new("side_panel_slot_combo_box", "")
                        .selected_text(gear_options.slot.to_string())
                        .show_ui(ui, |ui| {
                            for slot in Slot::vec() {
                                ui.selectable_value(&mut gear_options.slot, slot, slot.to_string());
                            }
                        });
                });

            ui.separator();

            gear_options.get_current_set().unwrap().draw(ui);

            return;
        }

        ui.collapsing("Game Options", |ui| {
            let mut game_options = self.game_options.borrow_mut();

            ui.horizontal(|ui| {
                ui.label("Armor");
                egui::ComboBox::new("side_panel_armor_combo_box", "")
                    .selected_text(game_options.armor.to_string())
                    .show_ui(ui, |ui| {
                        for option in Armor::vec() {
                            ui.selectable_value(
                                &mut game_options.armor,
                                option,
                                option.to_string(),
                            );
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Challenge");
                egui::ComboBox::new("side_panel_challenge_combo_box", "")
                    .selected_text(game_options.challenge.to_string())
                    .show_ui(ui, |ui| {
                        for option in Challenge::vec() {
                            ui.selectable_value(
                                &mut game_options.challenge,
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
                                game_options.stats.map.get_mut(stat).unwrap(),
                                200,
                                0,
                            );

                            ui.end_row();
                        }
                    });
            });

            drop(game_options);

            ui.collapsing("Builds", |ui| {
                self.draw_builds(ui, true);
            });
        });

        if matches!(self.view, AppView::AI) {
            let mut ai_options = self.ai_options.borrow_mut();

            ui.collapsing("AI Options", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Depth");
                    unsigned_number_input(ui, &mut ai_options.depth, 10, 2);
                });
            });
        }
    }
}
