use bevy::prelude::*;

use crate::{
    features::shared::NextButtonPlugin,
    systems::{AppState, AssetLoaderPlugin},
};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(AppState = AppState::Game)]
pub enum GameState {
    #[default]
    PreGame,
    LoadingCards,
    InGame,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GameState>()
            .add_plugins(NextButtonPlugin::<GameState>::without_interaction())
            .add_plugins(AssetLoaderPlugin {
                loading_state: GameState::LoadingCards,
                next_state: GameState::InGame,
            });
    }
}
