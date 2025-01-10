use bevy::prelude::Resource;

use super::PlayerNumber;

/// From 1 - 4
#[derive(Resource)]
pub struct FirstPlayer(usize);

impl FirstPlayer {
    pub fn new() -> Self {
        Self(1)
    }

    pub fn rotate(&mut self, player_number: &PlayerNumber) {
        if player_number.0 == self.0 {
            self.0 = 1;
        } else {
            self.0 += 1;
        }
    }
}
