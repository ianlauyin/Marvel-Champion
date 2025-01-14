use bevy::prelude::*;

use super::{change_card_state_on_added, CardState};

pub struct OutOfPlayAreaPlugin;

impl Plugin for OutOfPlayAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(change_card_state_on_added::<OutOfPlayArea>(
            CardState::OutPlay,
        ));
    }
}

#[derive(Component)]
pub struct OutOfPlayArea;
