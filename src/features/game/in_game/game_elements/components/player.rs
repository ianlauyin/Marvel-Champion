use bevy::prelude::Component;

use crate::features::cards::Identity;

/// From 1-4
#[derive(Component, Clone)]
pub struct Player {
    tag: usize,
    pub identity: Identity,
}

impl Player {
    pub fn new(player_tag: usize, identity: &Identity) -> Self {
        Self {
            tag: player_tag,
            identity: identity.clone(),
        }
    }

    pub fn is_equal(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}
