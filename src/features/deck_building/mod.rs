mod deck_editor;
mod deck_list;
mod identity_list;
mod state;

use bevy::app::{App, Plugin};

pub struct DeckBuildingPlugin;

impl Plugin for DeckBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            identity_list::DeckBuildingIdentityListPlugin,
            state::DeckBuildingStatePlugin,
            deck_list::DeckBuildingDeckListPlugin,
            deck_editor::DeckEditorPlugin,
        ));
    }
}
