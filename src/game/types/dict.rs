use crate::game::types::damage::DamageType;
use std::collections::hash_map::{Iter, Keys};
use std::collections::HashMap;
use std::ops::Neg;

// A HashMap wrapper for bonus and resistance dictionaries
// Has shorter syntax, and a default value of 0
#[derive(Debug, Clone)]
pub struct Dict {
    // TODO: Use a faster hash algorithm
    map: HashMap<String, f32>,
}

impl Dict {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn keys(&self) -> Keys<'_, String, f32> {
        self.map.keys()
    }

    pub fn set(&mut self, key: &str, value: f32) {
        self.map.insert(key.to_string(), value);
    }

    pub fn add(&mut self, key: &str, value: f32) {
        self.map
            .insert(key.to_string(), value + self.map.get(key).unwrap_or(&0.0));
    }

    pub fn get(&self, key: &str) -> f32 {
        let default_value = match key {
            "crit_modifier" => 1.75,
            _ => 0.0,
        };
        *self.map.get(key).unwrap_or(&default_value)
    }

    pub fn get_by_dmg_type(&self, dmg_type: DamageType) -> f32 {
        match dmg_type {
            DamageType::Melee => self.get("STR"),
            DamageType::Pierce => self.get("DEX"),
            DamageType::Magic => self.get("INT"),
            DamageType::Constant => 0.0,
        }
    }

    // Combine the dict with another
    pub fn combine(&self, other: &Dict) -> Dict {
        let mut result = (*self).clone();
        for key in other.keys() {
            result.add(key, other.get(key))
        }

        result
    }

    // Add the values of another Dict
    pub fn merge(&mut self, other: &Dict) {
        for key in other.keys() {
            self.add(key, other.get(key))
        }
    }

    // Subtract the values of another Dict
    pub fn unmerge(&mut self, other: &Dict) {
        for key in other.keys() {
            self.add(key, -other.get(key))
        }
    }

    pub fn iter(&self) -> Iter<String, f32> {
        self.map.iter()
    }

    pub fn iter_sorted(&self) -> Vec<(&String, &f32)> {
        let mut data = Vec::new();
        for item in self.iter() {
            data.push(item);
        }

        data.sort_by(|a, b| a.0.cmp(b.0));

        data
    }
}

impl Default for Dict {
    fn default() -> Self {
        Dict::new()
    }
}

impl<const N: usize> From<[(&str, f32); N]> for Dict {
    fn from(value: [(&str, f32); N]) -> Self {
        let mut map = HashMap::new();
        for (key, val) in value {
            map.insert(key.to_string(), val);
        }

        Self { map }
    }
}

impl Neg for Dict {
    type Output = Dict;

    fn neg(self) -> Self::Output {
        let mut map = HashMap::new();
        for (key, value) in self.map {
            map.insert(key, -value);
        }

        Dict { map }
    }
}
