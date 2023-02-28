use crate::ai::types::eval_value::EvalValue;
use crate::game::game_manager::GameManager;

pub struct AIThread {
    game: GameManager,

    depth: u8,
}

impl AIThread {
    pub fn new(game: GameManager, depth: u8) -> Self {
        AIThread { game, depth }
    }

    fn eval_without_depth(&self) -> EvalValue {
        let player_base_entity = self.game.player.get_base_entity();
        let player_hp = player_base_entity.hp as f64;

        if player_hp <= 0.0 {
            return EvalValue::Lost;
        }

        let mut enemies_hp = 0.0;
        let mut enemies_max_hp = 0.0;
        for enemy in &self.game.enemies {
            let enemy_base_entity = enemy.get_base_entity();
            enemies_hp += enemy_base_entity.hp as f64;
            enemies_max_hp += enemy_base_entity.max_hp as f64;
        }

        if enemies_hp <= 0.0 {
            return EvalValue::Won;
        }

        EvalValue::InProgress(-100.0 * enemies_hp / enemies_max_hp)
    }

    pub fn eval(&self) -> EvalValue {
        if self.depth == 0 {
            return self.eval_without_depth();
        }

        let mut best_skill = EvalValue::Lost;

        for skill in self
            .game
            .player
            .get_base_type()
            .as_player()
            .get_available_skills()
        {
            let mut current_game = self.game.clone();
            let mut _player_ref = current_game.player.get_base_type_mut();
            let player = _player_ref.as_player();

            player.set_current_skill(skill);
            current_game.do_turn();

            let ai_thread = AIThread::new(current_game, self.depth - 1);
            let skill_eval = ai_thread.eval();

            if skill_eval == EvalValue::Won {
                return skill_eval;
            }

            if skill_eval > best_skill {
                best_skill = skill_eval
            }
        }

        best_skill
    }
}
