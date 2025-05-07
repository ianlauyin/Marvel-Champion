mod content;
mod content_system;
mod editor;
mod header_button;
mod title;

use bevy::app::{App, Plugin};

pub struct DeckEditorPlugin;

impl Plugin for DeckEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            header_button::HeaderButtonPlugin,
            editor::DeckEditorPlugin,
            title::DeckEditorTitlePlugin,
            content::DeckEditorContentPlugin,
            content_system::DeckEditorContentSystemPlugin,
        ));
    }
}
