use bevy::prelude::*;

use crate::features::{
    game::state::GameState,
    shared::{NextButtonPlugin, PreviousButtonPlugin},
};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(GameState = GameState::PreGame)]
pub enum GameSelectorState {
    #[default]
    Identity,
    Deck,
    Encounter,
}

pub struct GameSelectorStatePlugin;

impl Plugin for GameSelectorStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GameSelectorState>().add_plugins((
            PreviousButtonPlugin::<GameSelectorState>::default(),
            NextButtonPlugin::<GameSelectorState>::default(),
        ));
    }
}
