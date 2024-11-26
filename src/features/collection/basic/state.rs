use bevy::prelude::*;

use crate::{features::collection::state::CollectionState, systems::AssetLoaderPlugin};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(CollectionState = CollectionState::Basic)]
pub enum CollectionBasicState {
    #[default]
    LoadingCards,
    Cards,
}

pub struct CollectionBasicStatePlugin;

impl Plugin for CollectionBasicStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionBasicState>()
            .add_plugins(AssetLoaderPlugin {
                loading_state: CollectionBasicState::LoadingCards,
                next_state: CollectionBasicState::Cards,
            });
    }
}
