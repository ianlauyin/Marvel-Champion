use bevy::prelude::*;

use crate::{
    features::collection::state::CollectionState, systems::AssetLoaderPlugin,
    ui::LoadingScreenPlugin,
};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(CollectionState = CollectionState::Hero)]
pub enum CollectionHeroState {
    #[default]
    LoadingList,
    List,
    LoadingCards,
    Cards,
}

pub struct CollectionHeroStatePlugin;

impl Plugin for CollectionHeroStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionHeroState>()
            .add_plugins(AssetLoaderPlugin {
                loading_state: CollectionHeroState::LoadingList,
                next_state: CollectionHeroState::List,
            })
            .add_plugins(AssetLoaderPlugin {
                loading_state: CollectionHeroState::LoadingCards,
                next_state: CollectionHeroState::Cards,
            });
    }
}
