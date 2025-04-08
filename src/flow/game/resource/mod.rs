use bevy::prelude::*;

use crate::flow::state::AppState;

pub struct GameResourcePlugin;

impl Plugin for GameResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), init_resources)
            .add_systems(OnExit(AppState::Game), cleanup_resources);
    }
}

fn init_resources(mut commands: Commands) {}

fn cleanup_resources(mut commands: Commands) {}
