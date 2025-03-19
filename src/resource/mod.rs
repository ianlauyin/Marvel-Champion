mod asset_loader;
mod aspect_card_datas;

pub use asset_loader::AssetLoader;
pub use aspect_card_datas::AspectCardDatas;

use bevy::prelude::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetLoader::default())
            .insert_resource(AspectCardDatas::new());
    }
}
