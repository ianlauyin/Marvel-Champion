use bevy::prelude::*;

use crate::flow::state::AppState;

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(AppState = AppState::DeckBuilding)]
pub enum DeckBuildingState {
    #[default]
    HeroMenu,
    DeckMenu,
    DeckEditor,
}
