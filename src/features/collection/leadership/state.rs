use bevy::prelude::*;

use crate::{features::collection::state::CollectionState, systems::AssetLoaderPlugin};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(CollectionState = CollectionState::Leadership)]
pub enum CollectionLeadershipState {
    #[default]
    LoadingCards,
    Cards,
}

pub struct CollectionLeadershipStatePlugin;

impl Plugin for CollectionLeadershipStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionLeadershipState>()
            .add_plugins(AssetLoaderPlugin {
                loading_state: CollectionLeadershipState::LoadingCards,
                next_state: CollectionLeadershipState::Cards,
            });
    }
}
