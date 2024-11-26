use bevy::prelude::*;

use crate::constants::{
    ACCELERATION_ICON_PATH, AMPLIFY_ICON_PATH, CRISIS_ICON_PATH, ENCOUNTER_CARD_BACK_PATH,
    GAME_MAT_PATH, HAZARD_ICON_PATH, PLAYER_CARD_BACK_PATH, VILLAIN_CARD_BACK_PATH,
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
            asset_server.load(GAME_MAT_PATH),
            asset_server.load(PLAYER_CARD_BACK_PATH),
            asset_server.load(ENCOUNTER_CARD_BACK_PATH),
            asset_server.load(VILLAIN_CARD_BACK_PATH),
            asset_server.load(ACCELERATION_ICON_PATH),
            asset_server.load(AMPLIFY_ICON_PATH),
            asset_server.load(CRISIS_ICON_PATH),
            asset_server.load(HAZARD_ICON_PATH),
        ]
        .to_vec(),
    );
}
