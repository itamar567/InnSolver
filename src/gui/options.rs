use crate::game::entities::armors::Armor;
use crate::game::entities::enemies::Challenge;
use crate::game::game_manager::GameManager;
use crate::game::types::dict::Dict;
use crate::gui::stats_handler::StatsHandler;

#[derive(Clone)]
pub struct GameOptions {
    pub level: i32,
    pub stats: StatsHandler,

    pub armor: Armor,
    pub challenge: Challenge,
}

impl GameOptions {
    pub fn create_game(&self) -> GameManager {
        let player = self
            .armor
            .create(Dict::from(self.stats.clone()), self.level);
        let enemies = self.challenge.create(self.level);

        GameManager::new(player, enemies)
    }
}

#[derive(Clone)]
pub struct AIOptions {
    pub depth: String,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            level: 90,
            stats: StatsHandler::new(),
            armor: Armor::Pirate,
            challenge: Challenge::Dummy,
        }
    }
}

impl Default for AIOptions {
    fn default() -> Self {
        Self {
            depth: '4'.to_string(),
        }
    }
}
