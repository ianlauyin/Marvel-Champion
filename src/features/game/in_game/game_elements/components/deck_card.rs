use bevy::prelude::*;

use super::{change_card_state_on_added, CardState};

pub struct DeckCardPlugin;

impl Plugin for DeckCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(change_card_state_on_added::<DeckCard>(CardState::OutPlay));
    }
}

#[derive(Component, Clone)]
pub struct DeckCard(usize);

impl DeckCard {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}
