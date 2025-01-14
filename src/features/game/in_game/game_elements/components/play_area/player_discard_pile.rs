use bevy::prelude::*;

use super::{change_card_state_on_added, CardState};

pub struct PlayerDiscardPilePlugin;

impl Plugin for PlayerDiscardPilePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(change_card_state_on_added::<PlayerDiscardPile>(
            CardState::OutPlay,
        ));
    }
}

#[derive(Component, Clone)]
pub struct PlayerDiscardPile(usize);

impl PlayerDiscardPile {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}
