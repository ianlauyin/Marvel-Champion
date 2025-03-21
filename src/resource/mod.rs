mod aspect_card_datas;
mod asset_loader;

pub use aspect_card_datas::AspectCardDatas;
pub use asset_loader::AssetLoader;

use bevy::prelude::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetLoader::default())
            .insert_resource(AspectCardDatas::new());
    }
}
