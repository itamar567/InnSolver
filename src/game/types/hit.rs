use crate::game::types::damage::{DamageRange, DamageType};
use crate::game::types::dict::Dict;
use crate::game::types::effects::Effect;

#[derive(Debug, Clone)]
pub struct Hit {
    pub elem: String,
    pub dmg_type: DamageType,
    pub dmg_range: DamageRange,
    pub bonuses: Option<Dict>, // Bonuses for this specific hit

    // List of effects to apply before calculating the damage
    pub before_hit_effects: Option<Vec<Effect>>,

    // List of effects to apply after calculating the damage
    pub after_hit_effects: Option<Vec<Effect>>,
}
