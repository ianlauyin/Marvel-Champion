mod deck_editor;
mod deck_menu;
mod hero_menu;
mod resource;

use bevy::app::{App, Plugin};

pub struct DeckBuildingPlugin;

impl Plugin for DeckBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hero_menu::HeroMenuPlugin,
            resource::DeckBuildingResourcePlugin,
            deck_menu::DeckMenuPlugin,
            deck_editor::DeckEditorPlugin,
        ));
    }
}
