use crate::game::entities::entity::EntityTrait;

pub enum GameStatus {
    Won,
    Lost,
    InProgress,
}

#[derive(Clone)]
pub struct GameManager {
    pub player: Box<dyn EntityTrait + Send>,
    pub enemies: Vec<Box<dyn EntityTrait + Send>>,
    pub turn: i32,
}

impl GameManager {
    pub fn new(
        player: Box<dyn EntityTrait + Send>,
        enemies: Vec<Box<dyn EntityTrait + Send>>,
    ) -> GameManager {
        GameManager {
            player,
            enemies,
            turn: 1,
        }
    }

    pub fn do_turn(&mut self) {
        self.turn += 1;

        self.player.get_base_entity_mut().tick_effects();
        self.player.do_turn(None, &mut self.enemies);
        self.player
            .get_base_type_mut()
            .as_player()
            .update_skill_cooldowns();

        for entity_index in 0..self.enemies.len() {
            // We first remove the current enemy from `self.enemies` to avoid a double mutable borrow
            let mut enemy = self.enemies.remove(entity_index);

            enemy.get_base_entity_mut().tick_effects();
            enemy.do_turn(Some(&mut self.player), &mut self.enemies);

            // Insert the enemy back
            self.enemies.insert(entity_index, enemy);
        }
    }

    pub fn get_status(&self) -> GameStatus {
        if self.player.get_base_entity().hp <= 0 {
            return GameStatus::Lost;
        }

        for enemy in &self.enemies {
            if enemy.get_base_entity().hp > 0 {
                return GameStatus::InProgress;
            }
        }

        GameStatus::Won
    }
}
