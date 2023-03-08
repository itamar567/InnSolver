use crate::game::game_manager::GameManager;
use crate::gui::options::GameOptions;
use crate::gui::utils;
use egui::{CollapsingHeader, Color32, Ui};
use std::cell::RefCell;
use std::rc::Rc;

struct GameHistoryManager {
    game: GameManager,
    game_history: Vec<GameManager>,
}

impl GameHistoryManager {
    pub fn new(game: GameManager) -> Self {
        Self {
            game,
            game_history: Vec::new(),
        }
    }

    pub fn current(&self) -> &GameManager {
        &self.game
    }

    pub fn current_mut(&mut self) -> &mut GameManager {
        &mut self.game
    }

    pub fn do_turn(&mut self) {
        self.game_history.push(self.game.clone());
        self.game.do_turn();
    }

    pub fn rollback(&mut self) {
        self.game = self.game_history.pop().unwrap();
    }
}

pub struct InteractiveView {
    game: Option<GameHistoryManager>,
    game_options: Rc<RefCell<GameOptions>>,
}

impl InteractiveView {
    pub fn new(game_options: Rc<RefCell<GameOptions>>) -> Self {
        Self {
            game: None,
            game_options,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        if let Some(game_history_manager) = &self.game {
            let game = game_history_manager.current();

            ui.heading(format!("Turn {}", game.turn));
            let skills = game
                .player
                .get_base_type()
                .as_player()
                .get_available_skills();
            self.draw_skillset(ui, skills);
            ui.separator();
            self.draw_game(ui);
        } else {
            self.game = Some(GameHistoryManager::new(
                self.game_options.borrow().create_game(),
            ));
        }
    }

    fn draw_skillset(&mut self, ui: &mut Ui, skills: Vec<usize>) {
        ui.horizontal(|ui| {
            for skill in skills {
                let game = self.game.as_mut().unwrap();
                let mut _player_ref = game.current_mut().player.get_base_type_mut();
                let player = _player_ref.as_player();

                if ui.button(player.skills[skill].name.clone()).clicked() {
                    player.set_current_skill(skill);
                    self.game.as_mut().unwrap().do_turn();
                }
            }

            let game = self.game.as_mut().unwrap();
            if !game.game_history.is_empty() && ui.button("Back").clicked() {
                game.rollback();
            }
        });
    }

    fn draw_game(&self, ui: &mut Ui) {
        let game = &self.game.as_ref().unwrap().game;
        let mut entities = vec![game.player.get_base_entity()];
        for enemy in &game.enemies {
            entities.push(enemy.get_base_entity());
        }

        let col_width = ui.available_width() / (entities.len() + 1) as f32;
        egui::Grid::new("game_grid")
            .min_col_width(col_width)
            .max_col_width(col_width)
            .show(ui, move |ui| {
                // Names
                for entity in &entities {
                    ui.heading(&entity.name);
                }
                ui.end_row();

                // HP
                for entity in &entities {
                    ui.add(
                        egui::ProgressBar::new(entity.hp as f32 / entity.max_hp as f32)
                            .text(format!(
                                "{} / {} ({}%)",
                                entity.hp,
                                entity.max_hp,
                                (100.0 * entity.hp as f32 / entity.max_hp as f32).ceil() as i32
                            ))
                            .fill(Color32::RED),
                    );
                }
                ui.end_row();

                // MP
                for entity in &entities {
                    ui.add(
                        egui::ProgressBar::new(entity.mp as f32 / entity.max_mp as f32)
                            .text(format!(
                                "{} / {} ({}%)",
                                entity.mp,
                                entity.max_mp,
                                100 * entity.mp / entity.max_mp
                            ))
                            .fill(Color32::BLUE),
                    );
                }
                ui.end_row();

                // Bonuses & resists
                for (index, entity) in entities.iter().enumerate() {
                    egui::Grid::new(format!("entity_details_bonuses_and_resists_grid_{}", index))
                        .num_columns(2)
                        .show(ui, |ui| {
                            ui.push_id(format!("entity_details_bonuses_label_{}", index), |ui| {
                                ui.collapsing("Bonuses", |ui| {
                                    let mut bonuses = entity.bonuses.clone();
                                    bonuses.merge(&entity.gear_bonuses);
                                    utils::draw_dict(
                                        ui,
                                        &bonuses,
                                        format!("entity_details_bonuses_grid_{}", index),
                                    );
                                });
                            });
                            ui.push_id(format!("entity_details_resists_label_{}", index), |ui| {
                                ui.collapsing("Resists", |ui| {
                                    let mut resists = entity.resists.clone();
                                    resists.merge(&entity.gear_bonuses);
                                    utils::draw_dict(
                                        ui,
                                        &resists,
                                        format!("entity_details_resists_grid_{}", index),
                                    );
                                })
                            });
                        });
                }
                ui.end_row();

                // Effects
                for (index, entity) in entities.iter().enumerate() {
                    ui.push_id(format!("entity_{}_effects", index), |ui| {
                        ui.collapsing("Effects", |ui| {
                            for effect in &entity.effects {
                                CollapsingHeader::new(format!(
                                    "{} ({} turns left)",
                                    effect.name, effect.duration
                                ))
                                .id_source(format!("entity_{}_effect_{}", index, effect.name))
                                .show(ui, |ui| {
                                    if let Some(bonuses) = &effect.bonuses {
                                        ui.collapsing("Bonuses", |ui| {
                                            utils::draw_dict(
                                                ui,
                                                bonuses,
                                                format!("entity_{}_effect_bonuses_grid", index),
                                            );
                                        });
                                    }
                                    if let Some(resists) = &effect.resists {
                                        ui.collapsing("Resists", |ui| {
                                            utils::draw_dict(
                                                ui,
                                                resists,
                                                format!("entity_{}_effect_bonuses_grid", index),
                                            );
                                        });
                                    }
                                    if let Some(dot) = &effect.dot {
                                        if dot.dmg_range.max < 0.0 {
                                            // HoT
                                            ui.label(format!(
                                                "{}-{} {} HoT",
                                                -dot.dmg_range.min.round() as i32,
                                                -dot.dmg_range.max.round() as i32,
                                                dot.elem
                                            ));
                                        } else {
                                            // DoT
                                            ui.label(format!(
                                                "{}-{} {} DoT",
                                                dot.dmg_range.min.round() as i32,
                                                dot.dmg_range.max.round() as i32,
                                                dot.elem
                                            ));
                                        }
                                    }
                                    if effect.stun.is_some() {
                                        ui.label("Stun");
                                    }
                                    if effect.death_proof {
                                        ui.label("Deathproof");
                                    }
                                    if let Some(description) = &effect.description {
                                        ui.label(description);
                                    }
                                });
                            }
                        });
                    });
                }
            });
    }
}
