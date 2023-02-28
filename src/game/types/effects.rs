use crate::game::types::damage::DamageRange;
use crate::game::types::dict::Dict;
use std::ops::Div;

#[derive(Debug, Clone)]
pub enum Stun {
    Normal,
    Automatic,
}

#[derive(Debug, Clone)]
pub struct DoT {
    pub dmg_range: DamageRange,
    pub elem: String,
}

impl DoT {
    pub fn new(dmg_range: DamageRange, elem: &str) -> Self {
        DoT {
            dmg_range,
            elem: elem.to_string(),
        }
    }
}

impl Div<f32> for DoT {
    type Output = DoT;

    fn div(self, rhs: f32) -> Self::Output {
        DoT {
            dmg_range: self.dmg_range / rhs,
            elem: self.elem,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Effect {
    pub name: String,
    pub duration: i8,

    pub bonuses: Option<Dict>,
    pub resists: Option<Dict>,

    pub dot: Option<DoT>,
    pub stun: Option<Stun>,
    pub death_proof: bool,
    pub description: Option<String>,
}

// In DragonFable, effects are considered equal if the name is the same.
impl PartialEq for Effect {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Effect {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        description: Option<String>,
        duration: i8,
        bonuses: Option<Dict>,
        resists: Option<Dict>,
        dot: Option<DoT>,
        stun: Option<Stun>,
        death_proof: bool,
    ) -> Self {
        Effect {
            name: name.to_string(),
            description,
            duration,
            bonuses,
            resists,
            dot,
            stun,
            death_proof,
        }
    }
}
