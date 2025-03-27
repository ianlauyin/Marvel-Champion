use bevy::prelude::*;

use crate::{cards::IdentitySet, flow::state::AppState, util::Deck};

#[derive(Resource, Default)]
pub struct DeckBuildingResource {
    pub identity: Option<IdentitySet>,
    pub deck: Option<Deck>,
}

impl DeckBuildingResource {
    pub fn set_identity(&mut self, identity: IdentitySet) {
        self.identity = Some(identity);
    }
}

pub struct DeckBuildingResourcePlugin;

const CURRENT_STATE: AppState = AppState::DeckBuilding;

impl Plugin for DeckBuildingResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), init_resources)
            .add_systems(OnExit(CURRENT_STATE), cleanup_resources);
    }
}

fn init_resources(mut commands: Commands) {
    commands.insert_resource(DeckBuildingResource::default());
}

fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<DeckBuildingResource>();
}
