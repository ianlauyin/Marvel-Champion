use bevy::{log::warn, prelude::Component};

/// From 1-4
#[derive(Component, Clone)]
pub struct Player(usize);

impl Player {
    pub fn new(player_tag: usize) -> Self {
        if player_tag < 1 || player_tag > 4 {
            warn!("player_tag should between 1 to 4 ,current: {}", player_tag);
        }
        Self(player_tag)
    }

    pub fn is(&self, player_tag: usize) -> bool {
        self.0 == player_tag
    }
}
