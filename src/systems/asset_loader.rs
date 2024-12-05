use std::collections::HashMap;

use bevy::asset::LoadState;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

use crate::features::cards::Card;
use crate::ui::LoadingScreenPlugin;

pub struct AssetLoaderSetupPlugin;

impl Plugin for AssetLoaderSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedAssetMap(HashMap::new()))
            .insert_resource(LoadAsset(Vec::new()));
    }
}

#[derive(Resource)]
pub struct LoadedAssetMap(pub HashMap<String, Handle<Image>>);

#[derive(Resource)]
pub struct LoadAsset(pub Vec<(String, Handle<Image>)>);

impl LoadAsset {
    pub fn add_card(&mut self, card: Card, asset_server: &Res<AssetServer>) {
        self.0
            .push((card.get_id(), asset_server.load(card.get_image_path())));
    }
}

// Remember to Add AssetLoaderPlugin in state.rs
// Add handles in LoadAsset.0, it will check in your defined loading_state.
pub struct AssetLoaderPlugin<S: States + FreelyMutableState> {
    pub loading_state: S,
    pub next_state: S,
}

impl<S: States + FreelyMutableState> Plugin for AssetLoaderPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadingScreenPlugin {
            loading_state: self.loading_state.clone(),
        })
        .add_systems(
            Update,
            check_asset(self.loading_state.clone(), self.next_state.clone()),
        );
    }
}

fn check_asset<S: States + FreelyMutableState>(loading_state: S, next_state: S) -> SystemConfigs {
    IntoSystem::into_system(
        move |mut load_asset: ResMut<LoadAsset>,
              mut loaded_asset: ResMut<LoadedAssetMap>,
              mut next_state_setter: ResMut<NextState<S>>,
              asset_server: Res<AssetServer>| {
            let mut removed_count = 0;
            for (index, (name, handle)) in load_asset.0.clone().iter().enumerate() {
                if loaded_asset.0.contains_key(name) {
                    load_asset.0.remove(index - removed_count);
                    removed_count += 1;
                    continue;
                }

                let asset_id = handle.clone().id();
                let Some(load_state) = asset_server.get_load_state(asset_id) else {
                    panic!("Cannot get load_state");
                };
                match load_state {
                    LoadState::Loaded => {
                        load_asset.0.remove(index - removed_count);
                        loaded_asset.0.insert(name.clone(), handle.clone());
                        removed_count += 1;
                    }
                    LoadState::Failed(err) => panic!("{err}"),
                    _ => return,
                }
            }

            if load_asset.0.is_empty() {
                next_state_setter.set(next_state.clone());
            }
        },
    )
    .run_if(in_state(loading_state))
}
