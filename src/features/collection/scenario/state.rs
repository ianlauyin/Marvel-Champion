use bevy::prelude::*;

use crate::{
    features::{collection::state::CollectionState, shared::PreviousButtonPlugin},
    systems::AssetLoaderPlugin,
};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(CollectionState = CollectionState::Scenario)]
pub enum CollectionScenarioState {
    #[default]
    List,
    LoadingCards,
    Cards,
}

pub struct CollectionScenarioStatePlugin;

impl Plugin for CollectionScenarioStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionScenarioState>()
            .add_plugins(PreviousButtonPlugin::<CollectionScenarioState>::default())
            .add_plugins(AssetLoaderPlugin {
                loading_state: CollectionScenarioState::LoadingCards,
                next_state: CollectionScenarioState::Cards,
            });
    }
}
