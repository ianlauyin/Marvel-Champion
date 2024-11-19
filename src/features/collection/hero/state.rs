use bevy::prelude::*;

use crate::{features::collection::state::CollectionState, ui::LoadingScreenPlugin};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(CollectionState = CollectionState::Hero)]
pub enum CollectionHeroState {
    #[default]
    LoadingAsset,
    List,
}

pub struct CollectionHeroStatePlugin;

impl Plugin for CollectionHeroStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionHeroState>()
            .add_plugins(LoadingScreenPlugin {
                loading_state: CollectionHeroState::LoadingAsset,
            });
    }
}
