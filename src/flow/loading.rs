use bevy::prelude::*;

use crate::{resource::AssetLoader, ui_component::LoadingScreen, util::ComponentUtil};

use super::state::AppState;

pub struct LoadingPlugin;

const ASSETS_PATHS: [&str; 3] = [
    "cards/card_backs/encounter_card_back",
    "cards/card_backs/player_card_back",
    "cards/card_backs/villain_card_back",
];

const CURRENT_STATE: AppState = AppState::Loading;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), on_enter)
            .add_systems(Update, on_update.run_if(in_state(CURRENT_STATE)))
            .add_systems(
                OnExit(CURRENT_STATE),
                ComponentUtil::cleanup_all::<LoadingScreen>,
            );
    }
}

fn on_enter(mut commands: Commands) {
    commands.spawn(LoadingScreen);
}

fn on_update(
    mut asset_loader: ResMut<AssetLoader>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if asset_loader.load_and_check(ASSETS_PATHS.to_vec(), &asset_server) {
        next_state.set(AppState::MainMenu);
    }
}
