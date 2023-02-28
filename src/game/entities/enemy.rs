use crate::game::entities::entity::{Entity, EntityType};
use crate::game::types::damage::{DamageRange, DamageType};
use crate::game::types::dict::Dict;

#[derive(Clone)]
pub struct Enemy {
    pub base: Entity,
}

impl Enemy {
    pub fn new(
        max_hp: i32,
        max_mp: i32,
        name: String,
        level: i32,
        dmg: DamageRange,
        dmg_type: DamageType,
        elem: &str,
    ) -> Self {
        Enemy {
            base: Entity::new(
                max_hp,
                max_mp,
                name,
                level,
                EntityType::EnemyType,
                dmg,
                dmg_type,
                elem.to_string(),
                Dict::new(),
                Dict::new(),
                Vec::new(),
            ),
        }
    }
}
