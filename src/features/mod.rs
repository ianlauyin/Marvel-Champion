use bevy::app::{App, Plugin};

pub mod cards;
mod collection;
mod main_menu;
mod shared;

pub struct FeaturePlugin;

impl Plugin for FeaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((collection::CollectionPlugin, main_menu::MainMenuPlugin));
    }
}
