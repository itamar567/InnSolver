use crate::game::entities::enemy::Enemy;
use crate::game::entities::entity::EntityType::PlayerType;
use crate::game::entities::player::Player;
use crate::game::types::damage::{DamageRange, DamageType};
use crate::game::types::dict::Dict;
use crate::game::types::effects::{DoT, Effect, Stun};
use crate::game::types::hit::Hit;
use crate::game::utils;

#[derive(Debug, Clone)]
pub enum EntityType {
    PlayerType,
    EnemyType,
}

pub enum EntityRef<'entity_ref> {
    PlayerRef(&'entity_ref Player),
    EnemyRef(&'entity_ref Enemy),
}

impl<'entity_ref> EntityRef<'entity_ref> {
    pub fn as_player(&self) -> &Player {
        match self {
            EntityRef::PlayerRef(player) => player,
            EntityRef::EnemyRef(_) => panic!("Cannot unwrap enemy reference as player"),
        }
    }

    pub fn as_enemy(&self) -> &Enemy {
        match self {
            EntityRef::PlayerRef(_) => panic!("Cannot unwrap player reference as enemy"),
            EntityRef::EnemyRef(enemy) => enemy,
        }
    }
}

pub enum EntityMutRef<'entity_ref> {
    PlayerMutRef(&'entity_ref mut Player),
    EnemyMutRef(&'entity_ref mut Enemy),
}

impl<'entity_ref> EntityMutRef<'entity_ref> {
    pub fn as_player(&mut self) -> &mut Player {
        match self {
            EntityMutRef::PlayerMutRef(player) => player,
            EntityMutRef::EnemyMutRef(_) => panic!("Cannot unwrap enemy reference as player"),
        }
    }

    pub fn as_enemy(&mut self) -> &mut Enemy {
        match self {
            EntityMutRef::PlayerMutRef(_) => panic!("Cannot unwrap player reference as enemy"),
            EntityMutRef::EnemyMutRef(enemy) => enemy,
        }
    }
}

pub trait EntityTrait {
    fn clone_box(&self) -> Box<dyn EntityTrait + Send>;

    fn get_base_entity(&self) -> &Entity;

    fn get_base_entity_mut(&mut self) -> &mut Entity;

    fn do_turn(
        &mut self,
        player: Option<&mut Box<dyn EntityTrait + Send>>,
        enemies: &mut Vec<Box<dyn EntityTrait + Send>>,
    );

    fn get_base_type(&self) -> EntityRef;

    fn get_base_type_mut(&mut self) -> EntityMutRef;
}

impl Clone for Box<dyn EntityTrait + Send> {
    fn clone(&self) -> Box<dyn EntityTrait + Send> {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub max_hp: i32,
    pub hp: i32,
    pub max_mp: i32,
    pub mp: i32,

    pub name: String,
    pub level: i32,

    entity_type: EntityType,

    pub dmg: DamageRange,
    pub dmg_type: DamageType,
    pub elem: String,

    // Bonuses/resistances from effects, armors, etc.
    pub bonuses: Dict,
    pub resists: Dict,

    // Bonuses/resistances from gear
    pub gear_bonuses: Dict,
    pub gear_resists: Dict,

    pub effects: Vec<Effect>,

    // Number of stuns currently active on the entity
    stuns: usize,
}

impl Entity {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        max_hp: i32,
        max_mp: i32,
        name: String,
        level: i32,
        entity_type: EntityType,
        dmg: DamageRange,
        dmg_type: DamageType,
        elem: String,
        bonuses: Dict,
        resists: Dict,
        effects: Vec<Effect>,
    ) -> Self {
        Self {
            max_hp,
            hp: max_hp,
            max_mp,
            mp: max_mp,
            name,
            level,
            entity_type,
            dmg,
            dmg_type,
            elem,
            bonuses,
            resists,
            gear_bonuses: Dict::new(),
            gear_resists: Dict::new(),
            effects,
            stuns: 0,
        }
    }

    pub fn recalculate_stat_bonuses(&mut self, stat_diff: &Dict) {
        let get_floored_value_diff = |e: &Entity, stat: &str, divide_by: f32| {
            let value = e.bonuses.get(stat) + e.gear_bonuses.get(stat);

            ((value + stat_diff.get(stat)) / divide_by).floor() - (value / divide_by).floor()
        };

        self.max_hp += self.get_bonus("END") as i32 * 5;
        self.max_mp += self.get_bonus("WIS") as i32 * 5;

        self.bonuses
            .add("crit", get_floored_value_diff(self, "LUK", 10.0));

        let mpm_diff = get_floored_value_diff(self, "LUK", 20.0);
        for def in ["melee_def", "pierce_def", "magic_def"].iter() {
            self.bonuses.add(def, mpm_diff);
        }
        self.bonuses
            .add("bonus", get_floored_value_diff(self, "WIS", 10.0));

        if matches!(self.entity_type, PlayerType) {
            self.resists
                .add("immobility", get_floored_value_diff(self, "END", 5.0));
        }
        self.resists
            .add("health", get_floored_value_diff(self, "WIS", -5.0));
    }

    pub fn get_resist(&self, elem: &str) -> f32 {
        let mut resist = self.resists.get(elem);
        let mut gear_resist = self.gear_resists.get(elem);

        if elem != "null" {
            resist += self.resists.get("all");
            gear_resist += self.gear_resists.get("all")
        }

        resist + gear_resist.min(80.0)
    }

    pub fn get_bonus(&self, bonus: &str) -> f32 {
        let value = self.bonuses.get(bonus);
        let mut gear_value = self.gear_bonuses.get(bonus);

        // Gear can only give 100 crit
        if bonus == "crit" {
            gear_value = gear_value.max(100.0);
        }

        value + gear_value
    }

    pub fn attack(&mut self, other: &mut Entity, hits: Vec<Hit>, attack_mana: bool) {
        for hit in hits {
            if let Some(bonuses) = &hit.bonuses {
                self.bonuses.merge(bonuses);
            }

            // We first check if the hit is a miss
            let mpm_value =
                other.bonuses.get(self.dmg_type.get_mpm_str()) - self.bonuses.get("bonus");

            if utils::chance(mpm_value / 151.0) {
                continue; // Hit missed, continue to the next hit
            }

            let bpd_value =
                other.bonuses.get(self.dmg_type.get_bpd_str()) - self.bonuses.get("bonus");
            let bpd = utils::chance(bpd_value / 151.0);

            let crit = utils::chance(self.bonuses.get("crit") / 201.0); // Crit rolls are between 0-200 inclusive, so we need to divide by 201

            if let Some(effects) = &hit.before_hit_effects {
                if !bpd || crit {
                    // Hit was a success, we can apply the before-hit effects
                    for eff in effects {
                        other.add_effect((*eff).clone());
                    }
                }
            }

            // We save the damage as f32 and round it to i32 before applying it
            let mut dmg = hit.dmg_range.get();

            let mainstat = self.bonuses.get_by_dmg_type(hit.dmg_type);

            // If `mainstat` is negative, the damage addition is rounded up instead of down
            if mainstat >= 0.0 {
                dmg += (mainstat / 10.0).floor();
            } else {
                dmg += (mainstat / 10.0).ceil();
            }

            let mut hit_modifier: f32;
            if attack_mana && !crit && bpd {
                hit_modifier = 0.0; // Non-crit glancing mana hits result in 0 damage
                                    // TODO: Check whether critical glancing mana hits result in 0 damage
            } else if crit && !bpd {
                // INT boost
                hit_modifier = self.bonuses.get("crit_modifier_bonus") + 1.75;
                hit_modifier *= 1.0 + self.bonuses.get("INT") / 1000.0;
            } else {
                if !crit && bpd {
                    // Glancing blow
                    hit_modifier = 0.05;
                } else {
                    // Normal hit or critical glancing blow
                    hit_modifier = 1.0;
                }

                // STR boost
                hit_modifier *= 1.0 + self.bonuses.get("STR") / 1000.0;
            }

            hit_modifier *= 1.0 + self.bonuses.get("DEX") / 4000.0; // DEX boost
            dmg *= hit_modifier;

            dmg *= 1.0 + self.bonuses.get("boost") / 100.0; // Boost

            // Resistances
            dmg *= (100.0 - self.get_resist(&hit.elem)) / 100.0;

            dmg = dmg.round();
            if attack_mana {
                other.mp -= dmg as i32;
            } else {
                other.hp -= dmg as i32;
            }

            if let Some(effects) = &hit.after_hit_effects {
                if !bpd || crit {
                    // Hit was a success, we can apply the after-hit effects
                    for eff in effects {
                        other.add_effect((*eff).clone());
                    }
                }
            }
            if let Some(bonuses) = &hit.bonuses {
                self.bonuses.unmerge(bonuses);
            }
        }
    }

    pub fn add_effect(&mut self, eff: Effect) {
        // Remove the effect if it already exists
        if let Some(index) = self.effects.iter().position(|r| r == &eff) {
            self.remove_effect(index);
        }

        // Handle stuns
        if let Some(stun) = &eff.stun {
            match stun {
                Stun::Normal => {
                    if !utils::chance(1.0 - (self.get_resist("immobility") / 100.0)) {
                        return; // Stun failed, don't apply the effect
                    }
                }
                Stun::Automatic => {}
            }
            self.stuns += 1;
        }

        // Add the effect's bonuses and resists
        if let Some(bonuses) = &eff.bonuses {
            self.recalculate_stat_bonuses(bonuses);
            self.bonuses.merge(bonuses);
        }
        if let Some(resists) = &eff.resists {
            self.resists.merge(resists);
        }

        self.effects.push(eff);
    }

    pub fn remove_effect(&mut self, index: usize) {
        let eff = self.effects.remove(index);

        if eff.stun.is_some() {
            self.stuns -= 1;
        }

        // Remove the effect's bonuses and resists
        if let Some(bonuses) = &eff.bonuses {
            self.recalculate_stat_bonuses(&-bonuses.clone());
            self.bonuses.unmerge(bonuses);
        }
        if let Some(resists) = &eff.resists {
            self.resists.unmerge(resists);
        }
    }

    pub fn take_dot(&mut self, dot: DoT) {
        let mut dmg = dot.dmg_range.get();

        // Resistances
        dmg *= (100.0 - self.get_resist(&dot.elem)) / 100.0;

        dmg = dmg.round();
        self.hp -= dmg as i32;
    }

    // Updates the effect cooldowns, applies DoT damage if needed
    pub fn tick_effects(&mut self) {
        let mut effects_to_remove: Vec<usize> = Vec::new();
        let mut dots: Vec<DoT> = Vec::new();

        for (index, mut eff) in &mut self.effects.iter_mut().enumerate() {
            eff.duration -= 1;
            if eff.duration <= 0 {
                effects_to_remove.push(index);
                continue;
            }

            if let Some(dot) = &eff.dot {
                dots.push((*dot).clone())
            }
        }

        // Take DoTs
        for dot in dots {
            self.take_dot(dot);
        }

        // Remove effects that faded
        for index in effects_to_remove.iter().rev() {
            self.remove_effect(*index);
        }
    }

    pub fn is_stunned(&self) -> bool {
        self.stuns > 0
    }

    pub fn heal(&mut self, elem: &str, mut amount: f32, heal_mp: bool) {
        amount *= (100.0 - self.get_resist(elem)) / 100.0;
        amount = amount.round();

        if heal_mp {
            self.mp += amount as i32;
        } else {
            self.hp += amount as i32;
        }
    }

    pub fn generate_hits(
        &mut self,
        amount: usize,
        total_damage_multiplier: f32,
        bonuses: Option<Dict>,
        before_hit_effects: Option<Vec<Effect>>,
        after_hit_effects: Option<Vec<Effect>>,
    ) -> Vec<Hit> {
        let mut result = Vec::new();
        let multiplier = total_damage_multiplier / (amount as f32);
        for _ in 0..amount {
            result.push(Hit {
                elem: self.elem.clone(),
                dmg_type: self.dmg_type,
                dmg_range: self.dmg * multiplier,
                bonuses: bonuses.clone(),
                before_hit_effects: before_hit_effects.clone(),
                after_hit_effects: after_hit_effects.clone(),
            });
        }

        result
    }

    pub fn get_mainstat(&self) -> f32 {
        self.bonuses.get_by_dmg_type(self.dmg_type)
    }

    pub fn generate_dot(&self, mut dmg: DamageRange, stat_dmg: bool) -> DoT {
        if stat_dmg {
            dmg += (3.125 * (self.get_mainstat() / 2.5).sqrt() - 5.0).ceil();
        }

        DoT::new(dmg, &self.elem)
    }
}
