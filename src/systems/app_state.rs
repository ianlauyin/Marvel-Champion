use bevy::prelude::*;

use crate::constants::GAME_MAT_PATH;

use super::{asset_loader::LoadAsset, AssetLoaderPlugin};

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
            .add_plugins(AssetLoaderPlugin {
                loading_state: AppState::LoadingAsset,
                next_state: AppState::MainMenu,
            })
            .add_systems(Startup, load_asset);
    }
}

fn load_asset(mut load_asset_res: ResMut<LoadAsset>, asset_server: Res<AssetServer>) {
    load_asset_res.0.push(asset_server.load(GAME_MAT_PATH));
}
