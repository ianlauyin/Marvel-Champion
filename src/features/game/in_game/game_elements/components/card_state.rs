use bevy::prelude::Component;

#[derive(Component)]
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
}
