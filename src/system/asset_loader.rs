use std::collections::HashSet;

use crate::system::AppState;
use bevy::asset::{LoadState, LoadedUntypedAsset};
use bevy::prelude::*;

use super::AppStateChangeEvent;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        let check_asset_condition = in_state(AppState::LoadingAsset);

        app.insert_resource(LoadAsset {
            image_handles: HashSet::new(),
            loading_image_handles: Vec::new(),
            state_loading: None,
        })
        .add_systems(Update, check_asset.run_if(check_asset_condition));
    }
}

#[derive(Clone)]
pub enum StateLoading {
    AppState,
}

#[derive(Resource)]
pub struct LoadAsset {
    pub image_handles: HashSet<Handle<Image>>,
    pub loading_image_handles: Vec<Handle<Image>>,
    pub state_loading: Option<StateLoading>,
}

fn check_asset(
    commands: Commands,
    mut load_asset: ResMut<LoadAsset>,
    asset_server: Res<AssetServer>,
) {
    for (index, loading_handle) in load_asset.loading_image_handles.clone().iter().enumerate() {
        if load_asset.image_handles.contains(loading_handle) {
            load_asset.loading_image_handles.remove(index);
            continue;
        }

        let asset_id = loading_handle.id();
        let Some(load_state) = asset_server.get_load_state(asset_id) else {
            panic!("Cannot get load_state");
        };
        match load_state {
            LoadState::Loaded => {
                load_asset.loading_image_handles.remove(index);
                load_asset.image_handles.insert(loading_handle.clone());
            }
            LoadState::Failed(err) => panic!("{err}"),
            _ => return,
        }
    }

    if load_asset.loading_image_handles.is_empty() {
        let Some(state_loading) = load_asset.state_loading.clone() else {
            warn!("StateLoading is None");
            return;
        };
        trigger_state_changes(commands, state_loading);
        load_asset.state_loading = None;
    }
}

fn trigger_state_changes(mut commands: Commands, state_loading: StateLoading) {
    match state_loading {
        StateLoading::AppState => commands.trigger(AppStateChangeEvent(AppState::MainMenu)),
    };
}
