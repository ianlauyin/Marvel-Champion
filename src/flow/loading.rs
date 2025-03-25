use std::sync::LazyLock;

use bevy::prelude::*;

use crate::{cards::*, resource::AssetLoader, ui_component::LoadingScreen, util::ComponentUtil};

use super::state::AppState;

pub struct LoadingPlugin;

const ASSETS_PATHS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut paths = vec![
        "cards/card_backs/encounter_card_back".to_string(),
        "cards/card_backs/player_card_back".to_string(),
        "cards/card_backs/villain_card_back".to_string(),
    ];

    let mut set_vec: Vec<Box<dyn SetTrait>> = vec![];
    for aspect in Aspect::get_all() {
        set_vec.push(Box::new(aspect));
    }
    for set in ExpertSet::get_all() {
        set_vec.push(Box::new(set));
    }
    for set in IdentitySet::get_all() {
        set_vec.push(Box::new(set));
    }
    for set in ModularSet::get_all() {
        set_vec.push(Box::new(set));
    }
    for set in Scenario::get_all() {
        set_vec.push(Box::new(set));
    }
    for set in StandardSet::get_all() {
        set_vec.push(Box::new(set));
    }
    for set in set_vec {
        if let Some(thumbnail_path) = set.get_thumbnail_key() {
            paths.push(thumbnail_path);
        }
        for card in set.get_card_infos() {
            paths.push(card.get_key());
        }
    }

    paths
});

const CURRENT_STATE: AppState = AppState::Loading;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), on_enter)
            .add_systems(Update, check_assets.run_if(in_state(CURRENT_STATE)))
            .add_systems(
                OnExit(CURRENT_STATE),
                ComponentUtil::cleanup_all::<LoadingScreen>,
            );
    }
}

fn on_enter(mut commands: Commands) {
    commands.spawn(LoadingScreen);
}

fn check_assets(
    mut asset_loader: ResMut<AssetLoader>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if asset_loader.load_and_check(ASSETS_PATHS.to_vec(), &asset_server) {
        next_state.set(AppState::MainMenu);
    }
}
