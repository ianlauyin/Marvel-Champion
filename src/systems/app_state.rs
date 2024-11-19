use bevy::prelude::*;

use crate::{constants::GAME_MAT_PATH, ui::LoadingScreenPlugin};

use super::asset_loader::{check_asset, load_asset};

#[derive(States, Default, Hash, PartialEq, Eq, Debug, Clone)]
pub enum AppState {
    #[default]
    LoadingAsset,
    MainMenu,
    Game,
    DeckBuilding,
    Collection,
    Record,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(LoadingScreenPlugin {
                loading_state: AppState::LoadingAsset,
            })
            .add_systems(
                Startup,
                load_asset(AppState::MainMenu, vec![GAME_MAT_PATH.to_string()]),
            )
            .add_systems(
                Update,
                check_asset::<AppState>.run_if(in_state(AppState::LoadingAsset)),
            );
    }
}
