use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AssetLoader(HashMap<String, Handle<Image>>);

impl AssetLoader {
    pub fn get(&self, key: &str) -> &Handle<Image> {
        let Some(image_handle) = self.0.get(key) else {
            panic!("Asset not found: {}", key);
        };
        image_handle
    }

    pub fn load_and_check(&mut self, keys: Vec<String>, asset_server: &Res<AssetServer>) -> bool {
        let mut is_all_loaded = true;
        for key in keys {
            if let Some(handle) = self.0.get(&key) {
                if !asset_server.is_loaded(handle.id()) {
                    is_all_loaded = false;
                }
            } else {
                self.init(key, asset_server);
            }
        }
        is_all_loaded
    }

    fn init(&mut self, key: String, asset_server: &Res<AssetServer>) {
        let handle = asset_server.load(format!("embedded://{key}.png",));
        self.0.insert(key, handle.clone());
    }
}
