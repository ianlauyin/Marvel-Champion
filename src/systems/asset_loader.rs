use std::collections::HashSet;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedAsset(HashSet::new()));
    }
}

#[derive(Resource)]
pub struct LoadedAsset(HashSet<Handle<Image>>);

#[derive(Resource)]
pub struct LoadAsset<S: States + FreelyMutableState> {
    pub image_handles: Vec<Handle<Image>>,
    pub next_state: S,
}

/// Reminder: Pair with check_asset
pub fn load_asset<S: States + FreelyMutableState>(
    next_state: S,
    paths: Vec<String>,
) -> impl Fn(Commands, Res<AssetServer>) {
    move |mut commands: Commands, asset_server: Res<AssetServer>| {
        commands.insert_resource(LoadAsset {
            next_state: next_state.clone(),
            image_handles: paths.iter().map(|path| asset_server.load(path)).collect(),
        });
    }
}

/// Reminder: Pair with load_asset
pub fn check_asset<S: States + FreelyMutableState>(
    mut load_asset: ResMut<LoadAsset<S>>,
    mut loaded_asset: ResMut<LoadedAsset>,
    mut next_state: ResMut<NextState<S>>,
    asset_server: Res<AssetServer>,
) {
    let mut removed_count = 0;
    for (index, handle) in load_asset.image_handles.clone().iter().enumerate() {
        if loaded_asset.0.contains(&handle) {
            load_asset.image_handles.remove(index);
            continue;
        }

        let asset_id = handle.id();
        let Some(load_state) = asset_server.get_load_state(asset_id) else {
            panic!("Cannot get load_state");
        };
        match load_state {
            LoadState::Loaded => {
                load_asset.image_handles.remove(index - removed_count);
                loaded_asset.0.insert(handle.clone());
                removed_count += 1;
            }
            LoadState::Failed(err) => panic!("{err}"),
            _ => return,
        }
    }

    if load_asset.image_handles.is_empty() {
        next_state.set(load_asset.next_state.clone());
    }
}
