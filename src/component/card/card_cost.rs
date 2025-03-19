use bevy::prelude::Component;

#[derive(Component)]
pub enum CardCost {
    Constant(u8),
    PerPlayer(u8),
}

impl CardCost {
    pub fn constant(cost: u8) -> Self {
        Self::Constant(cost)
    }

    pub fn per_player(cost: u8) -> Self {
        Self::PerPlayer(cost)
    }
}
