use crate::system::AppState;
use bevy::prelude::*;

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(AppState = AppState::Collection)]
pub enum CollectionState {
    #[default]
    LoadingAsset,
}

pub struct CollectionStatePlugin;

impl Plugin for CollectionStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CollectionState>();
    }
}
