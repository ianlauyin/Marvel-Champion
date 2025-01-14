use bevy::prelude::*;

use super::{change_card_state_on_added, CardState};

pub struct EncounterCardPlugin;

impl Plugin for EncounterCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(change_card_state_on_added::<EncounterCard>(
            CardState::OutPlay,
        ));
    }
}

// TODO: Add plugin for handling add will trigger CardState change
#[derive(Component, Clone)]
pub struct EncounterCard(usize);

impl EncounterCard {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}
