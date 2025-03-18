use bevy::prelude::Component;

#[derive(Component)]
pub enum CardCost {
    Constant(u8),
    PerPlayer(u8),
}
