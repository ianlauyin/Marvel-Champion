use crate::{features::shared::PreviousButtonPlugin, systems::AppState};
use bevy::prelude::*;

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(AppState = AppState::Collection)]
pub enum CollectionState {
    #[default]
    Menu,
    Hero,
    Basic,
    Aggression,
    Leadership,
    Protection,
    Justice,
    Pool,
    Scenario,
    Modular,
}

pub struct CollectionStatePlugin;

impl Plugin for CollectionStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionState>()
            .add_plugins(PreviousButtonPlugin::<CollectionState>::default());
    }
}
