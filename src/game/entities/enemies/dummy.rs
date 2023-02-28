use crate::game::entities::enemy::Enemy;
use crate::game::entities::entity::EntityMutRef::EnemyMutRef;
use crate::game::entities::entity::EntityRef::EnemyRef;
use crate::game::entities::entity::{Entity, EntityMutRef, EntityRef, EntityTrait};
use crate::game::types::damage::{DamageRange, DamageType};

#[derive(Clone)]
pub struct Dummy {
    base: Enemy,
}

impl EntityTrait for Dummy {
    fn clone_box(&self) -> Box<dyn EntityTrait + Send> {
        Box::new(self.clone())
    }

    fn get_base_entity(&self) -> &Entity {
        &self.base.base
    }

    fn get_base_entity_mut(&mut self) -> &mut Entity {
        &mut self.base.base
    }

    fn do_turn(
        &mut self,
        _player: Option<&mut Box<dyn EntityTrait + Send>>,
        _enemies: &mut Vec<Box<dyn EntityTrait + Send>>,
    ) {
    }

    fn get_base_type(&self) -> EntityRef {
        EnemyRef(&self.base)
    }

    fn get_base_type_mut(&mut self) -> EntityMutRef {
        EnemyMutRef(&mut self.base)
    }
}

impl Dummy {
    pub fn new(_level: i32) -> Self {
        Dummy {
            base: Enemy::new(
                5000,
                50,
                "Dummy".to_string(),
                1,
                DamageRange::from(0.0),
                DamageType::Melee,
                "None",
            ),
        }
    }
}
