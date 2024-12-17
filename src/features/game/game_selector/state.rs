use bevy::prelude::*;

use crate::features::game::state::GameState;

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(GameState = GameState::PreGame)]
pub enum GameSelectorState {
    #[default]
    Identity,
    PlayerDeck,
    Villain,
    Modular,
}

pub struct GameSelectorStatePlugin;

impl Plugin for GameSelectorStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GameSelectorState>();
    }
}
