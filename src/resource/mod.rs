mod asset_loader;
mod card_datas;

pub use asset_loader::AssetLoader;
pub use card_datas::CardDatas;

use bevy::prelude::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetLoader::default())
            .insert_resource(CardDatas::new());
    }
}
