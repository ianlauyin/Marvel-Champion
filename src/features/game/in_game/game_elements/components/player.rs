use bevy::{log::warn, prelude::Component};

/// From 1-4
#[derive(Component, Clone)]
pub struct Player {
    tag: usize,
    max_health: u8,
    current_health: u8,
}

impl Player {
    pub fn new(tag: usize, health: u8) -> Self {
        if tag < 1 || tag > 4 {
            warn!("player_tag should between 1 to 4 ,current: {}", tag);
        }
        Self {
            tag,
            max_health: health,
            current_health: health,
        }
    }

    pub fn is(&self, tag: usize) -> bool {
        self.tag == tag
    }
}
