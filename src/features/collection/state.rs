use crate::systems::AppState;
use bevy::prelude::*;

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(AppState = AppState::Collection)]
pub enum CollectionState {
    #[default]
    Menu,
}

pub struct CollectionStatePlugin;

impl Plugin for CollectionStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CollectionState>();
    }
}
