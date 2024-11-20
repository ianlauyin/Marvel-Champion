use bevy::app::{App, Plugin};

mod hero;
mod menu;
mod state;

pub struct CollectionPlugin;

impl Plugin for CollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            menu::CollectionMenuPlugin,
            state::CollectionStatePlugin,
            hero::CollectionHeroStatePlugin,
            hero::CollectionHeroListPlugin,
        ));
    }
}
