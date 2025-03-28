use crate::{
    features::{
        cards::{CardDatas, Identity},
        deck_building::{deck_list::EditIdentity, state::DeckBuildingState},
        shared::{CustomButton, Popup, PreviousButton},
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
        app.add_plugins(TextInputPlugin).add_systems(
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
            header.spawn(PreviousButton(DeckBuildingState::SelectDeck));
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
            buttons_node.spawn((
                ButtonAction::Save,
                CustomButton {
                    text: "Save".to_string(),
                    color: Color::srgb(0.212, 0.616, 0.263),
                    size: (Val::Px(100.), Val::Px(50.)),
                    ..default()
                },
            ));
            buttons_node.spawn((
                ButtonAction::Remove,
                CustomButton {
                    text: "Remove".to_string(),
                    color: Color::srgb(0.616, 0.212, 0.212),
                    size: (Val::Px(100.), Val::Px(50.)),
                    ..default()
                },
            ));
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
    commands: Commands,
    button_q: Query<(&Interaction, &ButtonAction)>,
    text_value_q: Query<&TextInputValue>,
    editing_deck: ResMut<EditingDeck>,
    edit_identity: Res<EditIdentity>,
    card_datas: Res<CardDatas>,
    pkv: ResMut<PkvStore>,
    mut next_state: ResMut<NextState<DeckBuildingState>>,
) {
    for (interaction, button_action) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            let mut decks_storage = DecksStorage::new(&edit_identity.0, pkv);
            match *button_action {
                ButtonAction::Save => handle_save(
                    commands,
                    decks_storage,
                    editing_deck.clone(),
                    card_datas,
                    edit_identity.0.clone(),
                    text_value_q,
                    next_state,
                ),
                ButtonAction::Remove => {
                    if let Some(index) = editing_deck.index {
                        decks_storage.remove_deck(index);
                    }
                    next_state.set(DeckBuildingState::SelectDeck);
                }
            }
            return;
        }
    }
}

fn handle_save(
    mut commands: Commands,
    mut decks_storage: DecksStorage,
    mut editing_deck: EditingDeck,
    card_datas: Res<CardDatas>,
    identity: Identity,
    text_value_q: Query<&TextInputValue>,
    mut next_state: ResMut<NextState<DeckBuildingState>>,
) {
    let cards = card_datas.from_ids(&editing_deck.deck.card_ids);
    match identity.get_validator().validate(&cards) {
        Ok(_) => (),
        Err(message) => {
            commands.spawn(Popup::new(message));
            return;
        }
    }

    if let Ok(text_value) = text_value_q.get_single() {
        editing_deck.deck.name = text_value.0.clone();
    }

    if let Some(index) = editing_deck.index {
        decks_storage.save_deck(editing_deck.deck, index);
    } else {
        decks_storage.add_deck(editing_deck.deck);
    }
    next_state.set(DeckBuildingState::SelectDeck);
}
