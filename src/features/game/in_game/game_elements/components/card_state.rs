use bevy::prelude::*;

#[derive(Component, Clone)]
pub enum CardState {
    InPlay,
    OutPlay,
}

impl CardState {
    pub fn toggle(&mut self) {
        *self = match *self {
            CardState::InPlay => CardState::OutPlay,
            CardState::OutPlay => CardState::InPlay,
        }
    }

    pub fn is_inplay(&self) -> bool {
        match *self {
            CardState::InPlay => true,
            CardState::OutPlay => false,
        }
    }
}
