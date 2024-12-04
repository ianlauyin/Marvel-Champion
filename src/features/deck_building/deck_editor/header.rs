use crate::{
    features::{
        deck_building::{deck_list::DeckListIdentity, state::DeckBuildingState},
        shared::{handle_previous_interaction, ButtonBuilder, PreviousButtonBuilder},
    },
    systems::DecksStorage,
};
use bevy::prelude::*;
use bevy_pkv::PkvStore;
use bevy_simple_text_input::{TextInput, TextInputPlugin, TextInputSubmitEvent};

use super::EditingDeck;

pub struct DeckEditorHeaderPlugin;

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckBuilding;

impl Plugin for DeckEditorHeaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin)
            .add_systems(Update, handle_previous_interaction(CURRENT_STATE))
            .add_systems(Update, handle_text_input_finished)
            .add_systems(
                Update,
                handle_button_interaction.run_if(in_state(CURRENT_STATE)),
            );
    }
}

#[derive(Component)]
struct Title {
    is_editing: bool,
}

#[derive(Component)]
enum ButtonAction {
    Save,
    Remove,
}

pub fn spawn_header(menu: &mut ChildBuilder, name: String) {
    menu.spawn(Node {
        padding: UiRect::all(Val::Px(10.)),
        display: Display::Flex,
        ..default()
    })
    .with_children(|header| {
        PreviousButtonBuilder(DeckBuildingState::SelectDeck).spawn(header);
        spawn_title(header, name);
        spawn_buttons(header);
    });
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
                ..default()
            },
            Title { is_editing: false },
        ))
        .with_children(|title_node| {
            title_node.spawn((Text::new(name), Title { is_editing: false }));
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

fn handle_text_input_finished(mut events: EventReader<TextInputSubmitEvent>) {
    for event in events.read() {
        info!("{:?} submitted: {}", event.entity, event.value);
    }
}

fn handle_button_interaction(
    button_q: Query<(&Interaction, &ButtonAction)>,
    editing_deck: Res<EditingDeck>,
    pkv: ResMut<PkvStore>,
    mut next_state: ResMut<NextState<DeckBuildingState>>,
    deck_list_identity: Res<DeckListIdentity>,
) {
    for (interaction, button_action) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            let mut decks_storage = DecksStorage {
                pkv,
                identity: deck_list_identity.0.clone(),
            };
            match *button_action {
                ButtonAction::Save => handle_save(decks_storage, editing_deck.clone()),
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

fn handle_save(mut decks_storage: DecksStorage, editing_deck: EditingDeck) {
    if let Some(index) = editing_deck.index {
        decks_storage.save_deck(editing_deck.deck, index);
    } else {
        decks_storage.add_deck(editing_deck.deck);
    }
}
