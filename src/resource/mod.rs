mod asset_loader;

pub use asset_loader::AssetLoader;

use bevy::prelude::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetLoader::default());
    }
}
