use bevy::prelude::Component;

#[derive(Component)]
pub struct CardBoost(u8);

impl CardBoost {
    pub fn new(boost: u8) -> Self {
        Self(boost)
    }
}
