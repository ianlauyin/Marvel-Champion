use bevy::prelude::*;

use crate::{
    flow::deck_building::state::DeckBuildingState, node_ui::MainContainer, util::SystemUtil,
};

use super::{content::DeckEditorContent, header::DeckEditorHeader};

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckEditor;

pub struct DeckEditorPlugin;

impl Plugin for DeckEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_deck_editor)
            .add_systems(OnExit(CURRENT_STATE), SystemUtil::cleanup_all::<DeckEditor>);
    }
}

#[derive(Component)]
struct DeckEditor;

fn spawn_deck_editor(mut commands: Commands, z_index_q: Query<&ZIndex>) {
    commands
        .spawn((MainContainer::default(&z_index_q), DeckEditor))
        .with_children(|parent| {
            parent.spawn(DeckEditorHeader);
            parent.spawn(DeckEditorContent);
        });
}
