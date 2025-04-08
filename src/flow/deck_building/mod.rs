mod deck_editor;
mod deck_menu;
mod hero_menu;
mod resource;
mod state;

use bevy::prelude::{App, AppExtStates, Plugin};

pub struct DeckBuildingPlugin;

impl Plugin for DeckBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<state::DeckBuildingState>()
            .add_plugins((
                hero_menu::HeroMenuPlugin,
                resource::DeckBuildingResourcePlugin,
                deck_menu::DeckMenuPlugin,
                deck_editor::DeckEditorPlugin,
            ));
    }
}
