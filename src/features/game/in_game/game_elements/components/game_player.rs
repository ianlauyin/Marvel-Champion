use bevy::prelude::Component;

/// From 1-4
#[derive(Component)]
pub struct GamePlayer(usize);

impl GamePlayer {
    pub fn new(player_order: usize) -> Self {
        Self(player_order)
    }
}
