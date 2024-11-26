use bevy::prelude::*;

use crate::constants::{
    ENCOUNTER_CARD_BACK_PATH, GAME_MAT_PATH, PLAYER_CARD_BACK_PATH, VILLAIN_CARD_BACK_PATH,
};

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
            .add_plugins(AssetLoaderPlugin {
                loading_state: AppState::LoadingAsset,
                next_state: AppState::MainMenu,
            })
            .add_systems(Startup, load_asset);
    }
}

fn load_asset(mut load_asset_res: ResMut<LoadAsset>, asset_server: Res<AssetServer>) {
    load_asset_res.0.append(
        &mut [
            ("game_mat".to_string(), asset_server.load(GAME_MAT_PATH)),
            (
                "player_card_back".to_string(),
                asset_server.load(PLAYER_CARD_BACK_PATH),
            ),
            (
                "encounter_card_back".to_string(),
                asset_server.load(ENCOUNTER_CARD_BACK_PATH),
            ),
            (
                "villain_card_back".to_string(),
                asset_server.load(VILLAIN_CARD_BACK_PATH),
            ),
        ]
        .to_vec(),
    );
}
