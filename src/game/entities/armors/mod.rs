use crate::game::entities::entity::EntityTrait;
use crate::game::types::dict::Dict;
use std::fmt::{Display, Formatter};

pub mod pirate;

#[derive(PartialEq, Copy, Clone)]
pub enum Armor {
    Pirate,
}

impl Armor {
    pub fn create(&self, stats: Dict, level: i32) -> Box<dyn EntityTrait + Send> {
        Box::new(match self {
            Armor::Pirate => pirate::Pirate::new(stats, level),
        })
    }

    pub fn vec() -> Vec<Armor> {
        vec![Armor::Pirate]
    }
}

impl Display for Armor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Armor::Pirate => "Pirate",
            }
        )
    }
}
