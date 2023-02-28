use crate::game::types::dict::Dict;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StatsHandler {
    pub map: HashMap<String, String>,
}

impl StatsHandler {
    pub fn new() -> Self {
        Self {
            map: HashMap::from([
                ("STR".to_string(), "0".to_string()),
                ("DEX".to_string(), "0".to_string()),
                ("INT".to_string(), "0".to_string()),
                ("CHA".to_string(), "0".to_string()),
                ("LUK".to_string(), "0".to_string()),
                ("END".to_string(), "0".to_string()),
                ("WIS".to_string(), "0".to_string()),
            ]),
        }
    }
}

impl Default for StatsHandler {
    fn default() -> Self {
        StatsHandler::new()
    }
}

impl From<StatsHandler> for Dict {
    fn from(value: StatsHandler) -> Self {
        Dict::from([
            ("STR", value.map["STR"].parse().unwrap()),
            ("DEX", value.map["DEX"].parse().unwrap()),
            ("INT", value.map["INT"].parse().unwrap()),
            ("CHA", value.map["CHA"].parse().unwrap()),
            ("LUK", value.map["LUK"].parse().unwrap()),
            ("END", value.map["END"].parse().unwrap()),
            ("WIS", value.map["WIS"].parse().unwrap()),
        ])
    }
}
