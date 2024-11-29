use bevy::prelude::*;

use crate::{features::collection::state::CollectionState, systems::AssetLoaderPlugin};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(CollectionState = CollectionState::Villain)]
pub enum CollectionVillainState {
    #[default]
    List,
    LoadingCards,
    Cards,
}

pub struct CollectionVillainStatePlugin;

impl Plugin for CollectionVillainStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionVillainState>()
            .add_plugins(AssetLoaderPlugin {
                loading_state: CollectionVillainState::LoadingCards,
                next_state: CollectionVillainState::Cards,
            });
    }
}
