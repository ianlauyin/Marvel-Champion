use bevy::app::{App, Plugin};

mod menu;
mod state;
mod sub_menu;

pub struct CollectionPlugin;

impl Plugin for CollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((menu::CollectionMenuPlugin, state::CollectionStatePlugin));
    }
}
