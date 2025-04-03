mod content;
mod editor;
mod header;

use bevy::app::{App, Plugin};

pub struct DeckEditorPlugin;

impl Plugin for DeckEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            header::DeckEditorHeaderPlugin,
            editor::DeckEditorPlugin,
            content::DeckEditorContentPlugin,
        ));
    }
}
