use bevy::prelude::*;

use crate::{
    features::{collection::state::CollectionState, shared::PreviousButtonPlugin},
    systems::AssetLoaderPlugin,
};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(CollectionState = CollectionState::Hero)]
pub enum CollectionHeroState {
    #[default]
    List,
    LoadingCards,
    Cards,
}

pub struct CollectionHeroStatePlugin;

impl Plugin for CollectionHeroStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionHeroState>()
            .add_plugins(PreviousButtonPlugin::<CollectionHeroState>::default())
            .add_plugins(AssetLoaderPlugin {
                loading_state: CollectionHeroState::LoadingCards,
                next_state: CollectionHeroState::Cards,
            });
    }
}
