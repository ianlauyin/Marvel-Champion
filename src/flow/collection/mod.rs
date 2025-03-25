mod component;
mod menu;
mod state;

use bevy::prelude::*;

pub struct CollectionPlugin;

impl Plugin for CollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<state::CollectionState>()
            .add_plugins(menu::CollectionMenuPlugin);
    }
}
