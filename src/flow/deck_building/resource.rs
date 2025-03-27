use bevy::prelude::*;

use crate::{cards::IdentitySet, flow::state::AppState};

#[derive(Resource)]
pub struct SelectedIdentity(Option<IdentitySet>);

impl SelectedIdentity {
    pub fn set(&mut self, identity: IdentitySet) {
        self.0 = Some(identity);
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
    commands.insert_resource(SelectedIdentity(None));
}

fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<SelectedIdentity>();
}
