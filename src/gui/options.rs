use crate::game::entities::armors::Armor;
use crate::game::entities::enemies::Challenge;
use crate::game::game_manager::GameManager;
use crate::game::types::dict::Dict;
use crate::game::types::gear::Slot;
use crate::gui::gear::GearSet;
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

#[derive(Clone)]
pub struct AIOptions {
    pub depth: String,
}

impl Default for AIOptions {
    fn default() -> Self {
        Self {
            depth: '4'.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct GearOptions {
    pub gear_sets: Vec<GearSet>,
    pub current_set_index: Option<usize>,
    pub slot: Slot,
}

impl Default for GearOptions {
    fn default() -> Self {
        Self {
            gear_sets: Vec::new(),
            current_set_index: None,
            slot: Slot::Weapon,
        }
    }
}

impl GearOptions {
    pub fn get_current_set(&self) -> Option<&GearSet> {
        if let Some(index) = self.current_set_index {
            return self.gear_sets.get(index);
        }

        None
    }

    pub fn get_current_set_mut(&mut self) -> Option<&mut GearSet> {
        if let Some(index) = self.current_set_index {
            return self.gear_sets.get_mut(index);
        }

        None
    }
}
