use bevy::prelude::Component;

/// From 1-4
#[derive(Component, Clone)]
pub struct Player(usize);

impl Player {
    pub fn new(player_tag: usize) -> Self {
        Self(player_tag)
    }

    pub fn is_equal(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
