use bevy::prelude::*;

use crate::{features::collection::state::CollectionState, systems::AssetLoaderPlugin};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(CollectionState = CollectionState::Justice)]
pub enum CollectionJusticeState {
    #[default]
    LoadingCards,
    Cards,
}

pub struct CollectionJusticeStatePlugin;

impl Plugin for CollectionJusticeStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionJusticeState>()
            .add_plugins(AssetLoaderPlugin {
                loading_state: CollectionJusticeState::LoadingCards,
                next_state: CollectionJusticeState::Cards,
            });
    }
}
