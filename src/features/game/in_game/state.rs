use bevy::prelude::*;

use crate::{features::game::state::GameState, systems::AssetLoaderPlugin};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(GameState = GameState::InGame)]
pub enum InGameState {
    #[default]
    LoadingCards,
    Setup,
    HeroTurn,
    VillainTurn,
    Result,
}

pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<InGameState>()
            .add_plugins(AssetLoaderPlugin {
                loading_state: InGameState::LoadingCards,
                next_state: InGameState::Setup,
            });
    }
}
