use bevy::prelude::*;

use crate::{features::collection::state::CollectionState, systems::AssetLoaderPlugin};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(CollectionState = CollectionState::Pool)]
pub enum CollectionPoolState {
    #[default]
    LoadingCards,
    Cards,
}

pub struct CollectionPoolStatePlugin;

impl Plugin for CollectionPoolStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionPoolState>()
            .add_plugins(AssetLoaderPlugin {
                loading_state: CollectionPoolState::LoadingCards,
                next_state: CollectionPoolState::Cards,
            });
    }
}
