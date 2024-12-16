use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{
    features::{
        cards::{CardDatas, Identity},
        shared::{handle_previous_interaction, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::{clean_up, DecksStorage, LoadAsset},
};

use super::{deck_editor::EditingDeck, state::DeckBuildingState};

pub struct DeckBuildingDeckListPlugin;

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::SelectDeck;

impl Plugin for DeckBuildingDeckListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_deck_list)
            .add_systems(Update, handle_previous_interaction(CURRENT_STATE))
            .add_systems(
                Update,
                handle_button_interaction.run_if(in_state(CURRENT_STATE)),
            )
            .add_systems(OnExit(CURRENT_STATE), clean_up::<DeckList>);
    }
}

#[derive(Resource)]
pub struct EditIdentity(pub Identity);

#[derive(Component, Clone)]
struct DeckList;

#[derive(Component, Clone)]
struct DeckListButton(EditingDeck);

fn spawn_deck_list(commands: Commands, pkv: ResMut<PkvStore>, identity: Res<EditIdentity>) {
    let mut deck_storage = DecksStorage {
        pkv,
        identity: identity.0.clone(),
    };
    let decks = deck_storage.get_decks();
    let mut button_map: Vec<(DeckListButton, ListItem)> = decks
        .iter()
        .enumerate()
        .map(|(index, deck)| {
            (
                DeckListButton(EditingDeck {
                    index: Some(index),
                    deck: deck.clone(),
                }),
                ListItem {
                    text: deck.name.clone(),
                    color: Color::srgb(0.576, 0.576, 0.576),
                    ..default()
                },
            )
        })
        .collect();
    button_map.push((
        DeckListButton(EditingDeck::new()),
        ListItem {
            text: "+".to_string(),
            color: Color::srgb(0.576, 0.576, 0.576),
            ..default()
        },
    ));
    MenuBuilder {
        component: DeckList,
        previous_state: DeckBuildingState::SelectIdentity,
        list_items: button_map,
        display_method: DisplayMethod::ButtonList,
    }
    .spawn(commands);
}

fn handle_button_interaction(
    mut commands: Commands,
    button_q: Query<(&Interaction, &DeckListButton)>,
    mut next_state: ResMut<NextState<DeckBuildingState>>,
    mut load_asset: ResMut<LoadAsset>,
    asset_server: Res<AssetServer>,
    deck_list_identity: Res<EditIdentity>,
) {
    for (interaction, button) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            let identity = deck_list_identity.0.clone();
            let mut editing_deck = button.0.clone();
            let cards = [identity.get_player_cards(), CardDatas::get_aspect_cards()].concat();
            for card in cards {
                load_asset.add_card(card, &asset_server);
            }

            // Add identity_cards when create new deck
            if editing_deck.index.is_none() {
                let mut identity_cards_ids = identity
                    .get_player_cards()
                    .iter()
                    .map(|card| card.get_id())
                    .collect();
                editing_deck.deck.card_ids.append(&mut identity_cards_ids);
            }

            commands.insert_resource(editing_deck);
            next_state.set(DeckBuildingState::LoadingCards);
            return;
        }
    }
}
