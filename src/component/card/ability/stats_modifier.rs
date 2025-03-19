use bevy::prelude::Component;

#[derive(Component)]
pub struct StatsModifier {
    thw: u8,
    atk: u8,
    def: u8,
}

impl StatsModifier {
    pub fn new(thw: u8, atk: u8, def: u8) -> Self {
        Self { thw, atk, def }
    }
}
