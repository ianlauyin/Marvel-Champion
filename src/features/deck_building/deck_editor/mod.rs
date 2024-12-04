use bevy::prelude::*;

mod content;
mod frame;
mod header;

pub struct DeckBuildingPlugin;

impl Plugin for DeckBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((frame::DeckEditorFramePlugin, header::DeckEditorHeaderPlugin));
    }
}

pub use frame::EditingDeck;
