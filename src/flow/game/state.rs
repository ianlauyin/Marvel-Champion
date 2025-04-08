use bevy::prelude::*;

use crate::flow::state::AppState;

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(AppState = AppState::Game)]
pub enum GameState {
    #[default]
    PreGame,
    InGame,
}
