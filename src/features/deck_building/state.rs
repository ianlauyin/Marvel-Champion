use bevy::prelude::*;

use crate::systems::{AppState, AssetLoaderPlugin};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(AppState = AppState::DeckBuilding)]
pub enum DeckBuildingState {
    #[default]
    SelectIdentity,
    SelectDeck,
    LoadingCards,
    DeckBuilding,
}

pub struct DeckBuildingStatePlugin;

impl Plugin for DeckBuildingStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<DeckBuildingState>()
            .add_plugins(AssetLoaderPlugin {
                loading_state: DeckBuildingState::LoadingCards,
                next_state: DeckBuildingState::DeckBuilding,
            });
    }
}
