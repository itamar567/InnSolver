use crate::game::types::damage::{DamageRange, DamageType};
use crate::game::types::dict::Dict;
use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Slot {
    Weapon,
    Helm,
    Cape,
    Necklace,
    Belt,
    Ring,
    Bracer,
}

impl Slot {
    pub fn vec() -> Vec<Slot> {
        vec![
            Slot::Weapon,
            Slot::Helm,
            Slot::Cape,
            Slot::Necklace,
            Slot::Belt,
            Slot::Ring,
            Slot::Bracer,
        ]
    }
}

impl ToString for Slot {
    fn to_string(&self) -> String {
        match self {
            Slot::Weapon => "Weapon",
            Slot::Helm => "Helm",
            Slot::Cape => "Cape",
            Slot::Necklace => "Necklace",
            Slot::Belt => "Belt",
            Slot::Ring => "Ring",
            Slot::Bracer => "Bracer",
        }
        .to_string()
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    pub slot: Slot,
    pub level: u8,

    pub dmg: Option<DamageRange>,
    pub dmg_type: Option<DamageType>,
    pub bonuses: Dict,
    pub resists: Dict,
}
