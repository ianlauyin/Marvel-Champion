use bevy::prelude::*;

use crate::{constants::GAME_MAT_ASSET, features::shared::PreviousButtonPlugin};

use super::{asset_loader::LoadAsset, AssetLoaderPlugin};

#[derive(States, Default, Hash, PartialEq, Eq, Debug, Clone)]
pub enum AppState {
    #[default]
    LoadingAsset,
    MainMenu,
    Game,
    DeckBuilding,
    Collection,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(PreviousButtonPlugin::<AppState>::default())
            .add_plugins(AssetLoaderPlugin {
                loading_state: AppState::LoadingAsset,
                next_state: AppState::MainMenu,
            })
            .add_systems(Startup, load_asset);
    }
}

fn load_asset(mut load_asset_res: ResMut<LoadAsset>, asset_server: Res<AssetServer>) {
    load_asset_res
        .0
        .push(GAME_MAT_ASSET.into_load_asset(&asset_server));
}
