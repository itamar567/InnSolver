pub fn chance(value: f32) -> bool {
    let roll: f32 = rand::random();

    roll <= value
}
