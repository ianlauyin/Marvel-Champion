use crate::{
    features::shared::{handle_previous_interaction, PreviousButtonBuilder},
    systems::clean_up,
};
use bevy::prelude::*;

use super::state::DeckBuildingState;

#[derive(Resource)]
pub struct EditingDeck(pub Option<usize>);

pub struct DeckEditorPlugin;

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckBuilding;

impl Plugin for DeckEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_editor)
            .add_systems(Update, handle_previous_interaction(CURRENT_STATE))
            .add_systems(OnExit(CURRENT_STATE), clean_up::<DeckEditor>);
    }
}

#[derive(Component)]
struct DeckEditor;

pub fn spawn_editor(mut commands: Commands) {
    commands
        .spawn((
            DeckEditor,
            Node {
                width: Val::Percent(90.),
                height: Val::Percent(90.),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip_y(),
                ..default()
            },
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
        ))
        .with_children(|menu| {
            spawn_header(menu);
        });
}

fn spawn_header(menu: &mut ChildBuilder) {
    menu.spawn(Node {
        height: Val::Percent(10.),
        padding: UiRect {
            left: Val::Px(10.),
            top: Val::Px(10.),
            bottom: Val::Px(10.),
            ..default()
        },
        ..default()
    })
    .with_children(|header| {
        PreviousButtonBuilder(DeckBuildingState::SelectDeck).spawn(header);
    });
}
