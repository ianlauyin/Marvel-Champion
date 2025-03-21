use bevy::{ecs::world::World, prelude::Component};

#[derive(Component)]
pub struct CardBoost {
    pub amount: u8,
    pub effect: Option<fn(&mut World)>,
}

impl CardBoost {
    pub fn amount(amount: u8) -> Self {
        Self {
            amount,
            effect: None,
        }
    }

    pub fn with_effect(amount: u8, effect: fn(&mut World)) -> Self {
        Self {
            amount,
            effect: Some(effect),
        }
    }
}
