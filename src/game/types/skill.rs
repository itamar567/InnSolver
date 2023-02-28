#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub mana: i32,
    pub cooldown: i32,

    pub current_cooldown: i32,
}

impl Skill {
    pub fn new(name: &str, mana: i32, cooldown: i32) -> Self {
        Self {
            name: name.to_string(),
            mana,
            cooldown,
            current_cooldown: 0,
        }
    }

    pub fn available(&self) -> bool {
        self.current_cooldown <= 0
    }
}
