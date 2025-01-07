use bevy::prelude::*;

use crate::{
    features::shared::NextButtonPlugin,
    systems::{AppState, AssetLoaderPlugin},
};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(GameState = GameState::InGame)]
pub enum InGameState {
    #[default]
    LoadingCards,
    Ready,
    InGame,
}

pub struct InGameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<InGameState>()
            .add_plugins(AssetLoaderPlugin {
                loading_state: InGameState::LoadingCards,
                next_state: InGameState::Ready,
            });
    }
}
