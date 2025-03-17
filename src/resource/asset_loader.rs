use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AssetLoader(HashMap<String, Handle<Image>>);

impl AssetLoader {
    pub fn load_and_check(&mut self, paths: Vec<&str>, asset_server: &Res<AssetServer>) -> bool {
        let mut is_all_loaded = true;
        for path in paths {
            if let Some(handle) = self.get(path) {
                if !asset_server.is_loaded(handle.id()) {
                    is_all_loaded = false;
                }
            } else {
                self.init(path, asset_server);
            }
        }
        is_all_loaded
    }

    pub fn get(&self, path: &str) -> Option<&Handle<Image>> {
        self.0.get(path)
    }

    fn init(&mut self, path: &str, asset_server: &Res<AssetServer>) {
        let handle = asset_server.load(format!("embedded://{path}.png",));
        self.0.insert(path.to_string(), handle.clone());
    }
}
