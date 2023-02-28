use crate::ai::ai_communication::{AICommunicator, AI};
use crate::game::game_manager::GameStatus;
use crate::gui::options::{AIOptions, GameOptions};
use egui::Ui;

pub struct AIView {
    ai: Option<AICommunicator>,
    current_rotation: String,

    game_options: GameOptions,
    ai_options: AIOptions,
}

impl AIView {
    pub fn new() -> Self {
        Self {
            ai: None,
            current_rotation: String::new(),
            game_options: GameOptions::default(),
            ai_options: AIOptions::default(),
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        if !self.current_rotation.is_empty() {
            ui.label(self.current_rotation.clone());
        }

        if let Some(ai) = &mut self.ai {
            if let Ok(skill) = ai.try_get_skill() {
                self.current_rotation.push_str(&format!("{} ➡ ", skill));

                match ai.ai.game.get_status() {
                    GameStatus::Won => {
                        self.ai = None;
                        self.current_rotation.push_str("Game Won");
                    }
                    GameStatus::Lost => {
                        self.ai = None;
                        self.current_rotation.push_str(
                            "Game Lost (try changing the stats or increasing the AI depth)",
                        );
                    }
                    GameStatus::InProgress => {
                        self.ai.as_mut().unwrap().do_turn();
                    }
                }
            }
        } else if ui.button("Start").clicked() {
            self.current_rotation.clear();
            self.ai = Some(AICommunicator::new(AI::new(
                self.game_options.create_game(),
                self.ai_options.depth.parse().unwrap(),
            )));
            self.ai.as_mut().unwrap().do_turn();
        }
    }

    pub fn configure(&mut self, game_options: GameOptions, ai_options: AIOptions) {
        self.game_options = game_options;
        self.ai_options = ai_options;
    }
}
