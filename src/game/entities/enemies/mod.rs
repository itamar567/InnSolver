use crate::game::entities::entity::EntityTrait;
use std::fmt::{Display, Formatter};

pub mod dummy;

#[derive(PartialEq, Copy, Clone)]
pub enum Challenge {
    Dummy,
}

impl Challenge {
    pub fn create(&self, level: i32) -> Vec<Box<dyn EntityTrait + Send>> {
        match self {
            Challenge::Dummy => vec![Box::new(dummy::Dummy::new(level))],
        }
    }

    pub fn vec() -> Vec<Challenge> {
        vec![Challenge::Dummy]
    }
}

impl Display for Challenge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Challenge::Dummy => "Dummy",
            }
        )
    }
}
