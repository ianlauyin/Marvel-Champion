mod component;
mod menu;

use bevy::prelude::*;

pub struct CollectionPlugin;

impl Plugin for CollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(menu::CollectionMenuPlugin);
    }
}
