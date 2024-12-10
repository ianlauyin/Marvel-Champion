use crate::{
    features::{
        deck_building::{deck_list::DeckListIdentity, state::DeckBuildingState},
        shared::{handle_previous_interaction, ButtonBuilder, PreviousButtonBuilder},
    },
    systems::DecksStorage,
};
use bevy::prelude::*;
use bevy_pkv::PkvStore;
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputPlugin, TextInputSubmitEvent, TextInputValue,
};

use super::EditingDeck;

pub struct DeckEditorHeaderPlugin;

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckBuilding;

impl Plugin for DeckEditorHeaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin)
            .add_systems(Update, handle_previous_interaction(CURRENT_STATE))
            .add_systems(
                Update,
                (
                    handle_button_interaction,
                    handle_title_interaction,
                    handle_text_input_finished,
                    handle_text_input_escape,
                )
                    .run_if(in_state(CURRENT_STATE)),
            );
    }
}

#[derive(Component)]
enum ButtonAction {
    Save,
    Remove,
}

pub fn spawn_header(mut commands: Commands, name: String) -> Entity {
    commands
        .spawn(Node {
            padding: UiRect::all(Val::Px(10.)),
            display: Display::Flex,
            column_gap: Val::Px(30.),
            ..default()
        })
        .with_children(|header| {
            PreviousButtonBuilder(DeckBuildingState::SelectDeck).spawn(header);
            spawn_title(header, name);
            spawn_buttons(header);
        })
        .id()
}

fn spawn_title(header: &mut ChildBuilder, name: String) {
    header
        .spawn((
            Node {
                flex_grow: 1.,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.)),
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            BorderColor::from(Color::WHITE),
            BorderRadius::all(Val::Px(5.)),
            Interaction::default(),
        ))
        .with_children(|title_node| {
            title_node.spawn((TextInput, TextInputValue(name), TextInputInactive(true)));
        });
}

fn spawn_buttons(header: &mut ChildBuilder) {
    header
        .spawn(Node {
            display: Display::Flex,
            column_gap: Val::Px(5.),
            ..default()
        })
        .with_children(|buttons_node| {
            let mut button_builder = ButtonBuilder {
                text: "Save".to_string(),
                color: Color::srgb(0.212, 0.616, 0.263),
                size: (Val::Px(100.), Val::Px(50.)),
                ..default()
            };

            button_builder
                .spawn(buttons_node)
                .insert(ButtonAction::Save);

            button_builder.text = "Remove".to_string();
            button_builder.color = Color::srgb(0.616, 0.212, 0.212);
            button_builder
                .spawn(buttons_node)
                .insert(ButtonAction::Remove);
        });
}

fn handle_title_interaction(mut text_input_q: Query<(&Interaction, &mut TextInputInactive)>) {
    let (interaction, mut inactive) = text_input_q.get_single_mut().unwrap();
    if *interaction == Interaction::Pressed {
        inactive.0 = false;
    }
}

fn handle_text_input_finished(
    mut events: EventReader<TextInputSubmitEvent>,
    mut text_input_q: Query<(&mut TextInputInactive, &mut TextInputValue)>,
    mut editing_deck: ResMut<EditingDeck>,
) {
    for event in events.read() {
        let name = event.value.clone();
        let Ok((mut inactive, mut value)) = text_input_q.get_single_mut() else {
            return;
        };

        editing_deck.deck.name = name.clone();
        value.0 = name.clone();
        inactive.0 = true;
    }
}

fn handle_text_input_escape(
    keys: Res<ButtonInput<KeyCode>>,
    mut text_input_q: Query<(&mut TextInputInactive, &mut TextInputValue)>,
    editing_deck: Res<EditingDeck>,
) {
    let Ok((mut inactive, mut value)) = text_input_q.get_single_mut() else {
        return;
    };
    if keys.just_pressed(KeyCode::Escape) {
        value.0 = editing_deck.deck.name.clone();
        inactive.0 = true;
    }
}

fn handle_button_interaction(
    button_q: Query<(&Interaction, &ButtonAction)>,
    text_value_q: Query<&TextInputValue>,
    editing_deck: ResMut<EditingDeck>,
    deck_list_identity: Res<DeckListIdentity>,
    pkv: ResMut<PkvStore>,
    mut next_state: ResMut<NextState<DeckBuildingState>>,
) {
    for (interaction, button_action) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            let mut decks_storage = DecksStorage {
                pkv,
                identity: deck_list_identity.0.clone(),
            };
            match *button_action {
                ButtonAction::Save => {
                    handle_save(decks_storage, editing_deck.clone(), text_value_q)
                }
                ButtonAction::Remove => {
                    if let Some(index) = editing_deck.index {
                        decks_storage.remove_deck(index);
                    }
                }
            }
            next_state.set(DeckBuildingState::SelectDeck);
            return;
        }
    }
}

fn handle_save(
    mut decks_storage: DecksStorage,
    mut editing_deck: EditingDeck,
    text_value_q: Query<&TextInputValue>,
) {
    if let Ok(text_value) = text_value_q.get_single() {
        editing_deck.deck.name = text_value.0.clone();
    }

    if let Some(index) = editing_deck.index {
        decks_storage.save_deck(editing_deck.deck, index);
    } else {
        decks_storage.add_deck(editing_deck.deck);
    }
}
