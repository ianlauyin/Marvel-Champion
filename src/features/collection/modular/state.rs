use bevy::prelude::*;

use crate::{
    features::{collection::state::CollectionState, shared::PreviousButtonPlugin},
    systems::AssetLoaderPlugin,
};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(CollectionState = CollectionState::Modular)]
pub enum CollectionModularState {
    #[default]
    List,
    LoadingCards,
    Cards,
}

pub struct CollectionModularStatePlugin;

impl Plugin for CollectionModularStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionModularState>()
            .add_plugins(PreviousButtonPlugin::<CollectionModularState>::default())
            .add_plugins(AssetLoaderPlugin {
                loading_state: CollectionModularState::LoadingCards,
                next_state: CollectionModularState::Cards,
            });
    }
}
