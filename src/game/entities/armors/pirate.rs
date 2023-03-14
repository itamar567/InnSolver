use crate::game::entities::entity::EntityMutRef::PlayerMutRef;
use crate::game::entities::entity::EntityRef::PlayerRef;
use crate::game::entities::entity::{Entity, EntityMutRef, EntityRef, EntityTrait};
use crate::game::entities::player::Player;
use crate::game::types::damage::DamageRange;
use crate::game::types::dict::Dict;
use crate::game::types::effects::{DoT, Effect, Stun};
use crate::game::types::hit::Hit;
use crate::game::types::skill::Skill;

/// The Pirate armor
///
/// See https://dragonfable-endgame.fandom.com/wiki/Pirate for details
#[derive(Clone)]
pub struct Pirate {
    base: Player,

    plunder: i8,
    retaliation: bool,
    cursed_treasure: bool,
}

impl Pirate {
    pub fn new(stats: Dict, level: i32) -> Self {
        Self {
            base: Player::new(
                stats,
                level,
                "darkness",
                vec![
                    Skill::new("Fury", 25, 9),
                    Skill::new("Lime", 30, 8),
                    Skill::new("Bawk", 25, 5),
                    Skill::new("Locker", 25, 7),
                    Skill::new("Quick", 20, 1),
                    Skill::new("Trick", 30, 12),
                    Skill::new("Sealegs", 20, 6),
                    Skill::new("Attack", 0, 0),
                    Skill::new("Backstab", 30, 8),
                    Skill::new("Plank", 25, 4),
                    Skill::new("Avast", 20, 5),
                    Skill::new("Target", 25, 6),
                    Skill::new("Pistol", 15, 1),
                    Skill::new("Cannon", 25, 7),
                    Skill::new("Curse", 40, 15),
                ],
            ),

            plunder: 0,
            retaliation: false,
            cursed_treasure: false,
        }
    }

    fn attack(&mut self, other: &mut Entity, mut hits: Vec<Hit>, attack_mana: bool) {
        let plunder_multiplier = if self.cursed_treasure { 0.2 } else { 0.1 };
        for hit in &mut hits {
            hit.dmg_range *= 1.0 + self.plunder as f32 * plunder_multiplier;
        }

        self.base.base.attack(other, hits, attack_mana)
    }

    // TODO: Use a variable instead
    fn opening_effect(&self) -> Effect {
        Effect::new(
            "Opening",
            Some("You spot an opening for extra hits on your next attack!".to_string()),
            1,
            None,
            None,
            None,
            None,
            false,
        )
    }

    fn opening(
        &mut self,
        other: &mut Entity,
        bonuses: Option<Dict>,
        before_hit_effects: Option<Vec<Effect>>,
        after_hit_effects: Option<Vec<Effect>>,
    ) {
        if !other.effects.contains(&self.opening_effect()) {
            return;
        }

        let hits =
            self.base
                .base
                .generate_hits(2, 1.0, bonuses, before_hit_effects, after_hit_effects);

        self.base.base.attack(other, hits, false);
    }

    fn skill_fury_of_the_high_seas(&mut self, entity: &mut Entity) {
        self.base.base.add_effect(Effect::new(
            "Fury",
            None,
            6,
            Some(Dict::from([("boost", 25.0)])),
            None,
            None,
            None,
            false,
        ));

        self.opening(entity, None, None, None);

        let hits = self.base.base.generate_hits(6, 1.75, None, None, None);

        self.attack(entity, hits, false)
    }

    fn skill_lime_aid(&mut self, entity: &mut Entity) {
        for (index, effect) in self.base.base.effects.clone().iter().enumerate().rev() {
            if effect.name != "Stuffed" {
                self.base.base.remove_effect(index);
            }
        }

        let heal_value = self.base.base.max_hp as f32 * 0.05;
        self.base.base.heal("health", heal_value, false);

        self.base.base.add_effect(Effect::new(
            "Lime-Aid",
            None,
            2,
            None,
            None,
            Some(DoT::new(-DamageRange::from(heal_value), "health")),
            None,
            false,
        ));

        self.opening(entity, None, None, None);
    }

    fn skill_summon_crackers(&mut self, entity: &mut Entity) {
        let after_hit_effects = Some(vec![Effect::new(
            "Go For The Eyes",
            None,
            3,
            Some(Dict::from([("bonus", -80.0)])),
            None,
            None,
            None,
            false,
        )]);

        self.opening(entity, None, None, after_hit_effects.clone());

        let hits = self
            .base
            .base
            .generate_hits(2, 1.0, None, None, after_hit_effects);

        self.attack(entity, hits, false);
    }

    fn skill_help_from_the_locker(&mut self, entity: &mut Entity) {
        let after_hit_effects = Some(vec![Effect::new(
            "Sunken Crew's Curse",
            None,
            4,
            Some(Dict::from([("crit", -30.0)])),
            Some(Dict::from([("all", -30.0), ("health", 30.0)])),
            None,
            None,
            false,
        )]);

        self.opening(entity, None, None, after_hit_effects.clone());

        let hits = self
            .base
            .base
            .generate_hits(3, 1.65, None, None, after_hit_effects);

        self.attack(entity, hits, false);
    }

    fn skill_quick_shot(&mut self, mut entities: Vec<&mut Entity>) {
        self.opening(entities[self.base.targeted_enemy_index], None, None, None);

        let hits = self.base.base.generate_hits(1, 1.3, None, None, None);

        for entity in entities {
            self.attack(entity, hits.clone(), false);
        }
    }

    fn skill_dirty_trick(&mut self, entity: &mut Entity) {
        let after_hit_effects = Some(vec![Effect::new(
            "Unsteady",
            None,
            3,
            None,
            None,
            None,
            Some(Stun::Normal),
            false,
        )]);

        self.opening(entity, None, None, None);

        let hits = self
            .base
            .base
            .generate_hits(3, 1.5, None, None, after_hit_effects);

        self.attack(entity, hits, false);
    }

    fn skill_sealegs(&mut self, entity: &mut Entity) {
        self.base.base.add_effect(Effect::new(
            "Sealegs",
            None,
            2,
            Some(Dict::from([
                ("melee_def", 180.0),
                ("pierce_def", 180.0),
                ("magic_def", 180.0),
            ])),
            None,
            None,
            None,
            false,
        ));

        self.opening(entity, None, None, None);
    }

    fn skill_attack(&mut self, entity: &mut Entity) {
        self.base.base.mp += 15;

        self.opening(entity, None, None, None);

        let hits = self.base.base.generate_hits(1, 1.25, None, None, None);

        self.attack(entity, hits, false);
    }

    fn skill_backstab(&mut self, entity: &mut Entity) {
        self.base.base.add_effect(Effect::new(
            "Retaliate against your target!",
            None,
            4,
            Some(Dict::from([
                ("block", 140.0),
                ("parry", 140.0),
                ("dodge", 140.0),
            ])),
            None,
            None,
            None,
            false,
        ));

        self.retaliation = true; // TODO: Implement retaliation

        self.opening(entity, None, None, None);
    }

    fn skill_to_the_plank(&mut self, entity: &mut Entity) {
        let after_hit_effects = Some(vec![Effect::new(
            "Planked",
            None,
            3,
            Some(Dict::from([("boost", -50.0)])),
            None,
            None,
            None,
            false,
        )]);

        self.opening(entity, None, None, after_hit_effects.clone());

        let hits = self
            .base
            .base
            .generate_hits(3, 1.2, None, None, after_hit_effects);

        self.attack(entity, hits, false);
    }

    fn skill_avast(&mut self, entity: &mut Entity) {
        let after_hit_effects = Some(vec![Effect::new(
            "Dire Straits",
            None,
            4,
            None,
            None,
            Some(self.base.base.generate_dot(self.base.base.dmg, true) / 2.0),
            None,
            false,
        )]);

        self.opening(entity, None, None, after_hit_effects.clone());

        let hits = self
            .base
            .base
            .generate_hits(1, 1.0, None, None, after_hit_effects);

        self.attack(entity, hits, false);
    }

    fn skill_target_practice(&mut self, entity: &mut Entity) {
        let after_hit_effects = Some(vec![Effect::new(
            "Pierced",
            None,
            4,
            Some(Dict::from([
                ("melee_def", -150.0),
                ("pierce_def", -150.0),
                ("magic_def", -150.0),
                ("block", -150.0),
                ("parry", -150.0),
                ("dodge", -150.0),
            ])),
            None,
            None,
            None,
            false,
        )]);

        self.opening(entity, None, None, after_hit_effects.clone());

        let hits = self
            .base
            .base
            .generate_hits(2, 0.55, None, None, after_hit_effects);

        self.attack(entity, hits, false);
    }

    fn skill_flintlock(&mut self, entity: &mut Entity) {
        self.opening(entity, None, None, None);

        let hits = self.base.base.generate_hits(2, 2.0, None, None, None);

        self.attack(entity, hits, false)
    }

    fn skill_fire_the_broadsides(&mut self, entity: &mut Entity) {
        let after_hit_effects = Some(vec![Effect::new(
            "Resounding Cannonade",
            None,
            5,
            Some(Dict::from([("boost", -20.0), ("bonus", -20.0)])),
            None,
            None,
            None,
            false,
        )]);

        let hit_bonuses = Some(Dict::from([("crit", 200.0)]));

        self.opening(entity, hit_bonuses.clone(), None, after_hit_effects.clone());

        let hits = self
            .base
            .base
            .generate_hits(1, 2.0, hit_bonuses, None, after_hit_effects);

        self.attack(entity, hits, false);
    }

    fn skill_cursed_treasure(&mut self, entity: &mut Entity) {
        self.plunder += 1;
        self.cursed_treasure = true;
        self.base.base.add_effect(Effect::new(
            "Cursed Treasure",
            Some(
                "Effects of Plunder doubled, Plunder vanishes when Cursed Treasure runs out"
                    .to_string(),
            ),
            6,
            None,
            None,
            None,
            None,
            false,
        ));

        // TODO: Add an HP/MP potion

        self.opening(entity, None, None, None);

        let hits = self.base.base.generate_hits(1, 2.0, None, None, None);
        self.attack(entity, hits, false)
    }
}

impl EntityTrait for Pirate {
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
        _: Option<&mut Box<dyn EntityTrait + Send>>,
        entities: &mut Vec<Box<dyn EntityTrait + Send>>,
    ) {
        // TODO: Add openings
        let skill = self.base.get_current_skill();
        self.base.use_skill(skill);

        if skill == 4 {
            let mut base_entities = Vec::new();
            for entity in entities {
                base_entities.push(entity.get_base_entity_mut());
            }
            self.skill_quick_shot(base_entities);
        } else {
            let entity = entities[self.base.targeted_enemy_index].get_base_entity_mut();
            match skill {
                0 => self.skill_fury_of_the_high_seas(entity),
                1 => self.skill_lime_aid(entity),
                2 => self.skill_summon_crackers(entity),
                3 => self.skill_help_from_the_locker(entity),
                5 => self.skill_dirty_trick(entity),
                6 => self.skill_sealegs(entity),
                7 => self.skill_attack(entity),
                8 => self.skill_backstab(entity),
                9 => self.skill_to_the_plank(entity),
                10 => self.skill_avast(entity),
                11 => self.skill_target_practice(entity),
                12 => self.skill_flintlock(entity),
                13 => self.skill_fire_the_broadsides(entity),
                14 => self.skill_cursed_treasure(entity),
                _ => panic!("Unknown skill index {}", skill),
            };
        }
    }

    fn get_base_type(&self) -> EntityRef {
        PlayerRef(&self.base)
    }

    fn get_base_type_mut(&mut self) -> EntityMutRef {
        PlayerMutRef(&mut self.base)
    }

    fn setup(
        &mut self,
        _player: Option<&mut Box<dyn EntityTrait + Send>>,
        enemies: &mut Vec<Box<dyn EntityTrait + Send>>,
    ) {
        enemies[self.base.targeted_enemy_index]
            .get_base_entity_mut()
            .add_effect(self.opening_effect());
    }
}
