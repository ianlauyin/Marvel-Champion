use bevy::prelude::*;

use crate::{features::collection::state::CollectionState, systems::AssetLoaderPlugin};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(CollectionState = CollectionState::Aggression)]
pub enum CollectionAggressionState {
    #[default]
    LoadingCards,
    Cards,
}

pub struct CollectionAggressionStatePlugin;

impl Plugin for CollectionAggressionStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionAggressionState>()
            .add_plugins(AssetLoaderPlugin {
                loading_state: CollectionAggressionState::LoadingCards,
                next_state: CollectionAggressionState::Cards,
            });
    }
}
