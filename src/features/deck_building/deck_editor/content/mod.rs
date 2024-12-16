use bevy::prelude::*;

mod systems;
mod ui;
mod utils;

pub struct DeckEditorContentPlugin;

impl Plugin for DeckEditorContentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ui::DeckEditorContentUIPlugin,
            systems::DeckEditorContentSystemPlugin,
        ));
    }
}

pub use ui::spawn_content;
