use crate::systems::{clean_up, Deck};
use bevy::prelude::*;

use super::{super::state::DeckBuildingState, header::spawn_header};

#[derive(Resource, Clone)]
pub struct EditingDeck {
    pub index: Option<usize>,
    pub deck: Deck,
}

impl EditingDeck {
    pub fn new() -> Self {
        Self {
            index: None,
            deck: Deck {
                name: "New Deck".to_string(),
                cards: vec![],
            },
        }
    }
}

pub struct DeckEditorFramePlugin;

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckBuilding;

impl Plugin for DeckEditorFramePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_editor)
            .add_systems(OnExit(CURRENT_STATE), clean_up::<DeckEditor>);
    }
}

#[derive(Component)]
struct DeckEditor;

pub fn spawn_editor(mut commands: Commands, editing_deck: Res<EditingDeck>) {
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
            spawn_header(menu, editing_deck.deck.name.clone());
        });
}
