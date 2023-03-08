use crate::game::entities::entity::Entity;
use crate::game::entities::entity::EntityType::PlayerType;
use crate::game::types::damage::{DamageRange, DamageType};
use crate::game::types::dict::Dict;
use crate::game::types::gear::{Item, Slot};
use crate::game::types::skill::Skill;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    pub base: Entity,

    pub targeted_enemy_index: usize,

    pub skills: Vec<Skill>,
    pub current_skill_index: Option<usize>,

    pub items: HashMap<Slot, Item>,
}

impl Player {
    pub fn new(stats: Dict, level: i32, elem: &str, skills: Vec<Skill>) -> Self {
        let mut result = Self {
            base: Entity::new(
                100 + (level - 1) * 20,
                100 + (level - 1) * 5,
                "Player".to_string(),
                level,
                PlayerType,
                DamageRange::from(20.0),
                DamageType::Melee,
                elem.to_string(),
                stats.clone(),
                Dict::new(),
                Vec::new(),
            ),
            targeted_enemy_index: 0,
            skills,
            current_skill_index: None,
            items: HashMap::new(),
        };

        result.base.recalculate_stat_bonuses(&stats);
        result.base.hp = result.base.max_hp;
        result.base.mp = result.base.max_mp;

        result
    }

    pub fn use_skill(&mut self, index: usize) {
        let skill = self.skills.get_mut(index).unwrap();
        skill.current_cooldown = skill.cooldown + 1;
        self.base.mp -= skill.mana;
    }

    pub fn update_skill_cooldowns(&mut self) {
        for mut skill in &mut self.skills {
            skill.current_cooldown = (skill.current_cooldown - 1).max(0);
        }
    }

    pub fn get_available_skills(&self) -> Vec<usize> {
        let mut skills = Vec::new();
        for (index, skill) in self.skills.iter().enumerate() {
            if skill.available() && self.base.mp >= skill.mana {
                skills.push(index);
            }
        }

        skills
    }

    pub fn set_current_skill(&mut self, index: usize) {
        self.current_skill_index = Some(index);
    }

    pub fn get_current_skill(&self) -> usize {
        self.current_skill_index.unwrap()
    }

    pub fn reset_current_skill(&mut self) {
        self.current_skill_index = None;
    }

    pub fn equip(&mut self, item: Item) {
        self.unequip(item.slot);

        self.base.bonuses.merge(&item.bonuses);
        self.base.resists.merge(&item.resists);

        self.items.insert(item.slot, item);
    }

    pub fn unequip(&mut self, slot: Slot) {
        if let Some(item) = self.items.remove(&slot) {
            self.base.bonuses.unmerge(&item.bonuses);
            self.base.resists.unmerge(&item.resists);
        }
    }
}
