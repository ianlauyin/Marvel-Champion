use bevy::prelude::*;

use crate::{flow::state::AppState, ui_component::MainContainer, util::SystemUtil};

use super::{
    super::resource::{DeckBuildingResource, DeckBuildingState},
    header::DeckEditorHeader,
};

pub struct DeckEditorPlugin;

impl Plugin for DeckEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_deck_editor, despawn_deck_editor).run_if(in_state(AppState::DeckBuilding)),
        );
    }
}

#[derive(Component)]
struct DeckEditor;

fn spawn_deck_editor(mut commands: Commands, res: Res<DeckBuildingResource>) {
    if res.is_changed() && res.get_state() == DeckBuildingState::DeckEditor {
        commands
            .spawn((MainContainer::new(), DeckEditor))
            .with_children(|parent| {
                parent.spawn(DeckEditorHeader);
            });
    }
}

fn despawn_deck_editor(
    commands: Commands,
    deck_editor_q: Query<Entity, With<DeckEditor>>,
    res: Res<DeckBuildingResource>,
) {
    if res.is_changed() && res.get_state() != DeckBuildingState::DeckEditor {
        SystemUtil::cleanup_all(commands, deck_editor_q);
    }
}
