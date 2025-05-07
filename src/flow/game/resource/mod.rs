mod enemy_info;
mod players_info;

use enemy_info::EnemyInfo;
pub use players_info::PlayersInfo;

use crate::flow::state::AppState;
use bevy::prelude::*;

pub struct GameResourcePlugin;

impl Plugin for GameResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), init_resources)
            .add_systems(OnExit(AppState::Game), cleanup_resources);
    }
}

fn init_resources(mut commands: Commands) {
    commands.init_resource::<PlayersInfo>();
    commands.init_resource::<EnemyInfo>();
}

fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<PlayersInfo>();
    commands.remove_resource::<EnemyInfo>();
}
