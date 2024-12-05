use bevy::prelude::*;

use crate::{
    constants::CARD_SIZE,
    features::{
        cards::{Card, CardDatas},
        deck_building::state::DeckBuildingState,
        shared::{spawn_card_list, ListItem},
    },
    systems::LoadedAssetMap,
};

pub struct DeckEditorContentPlugin;

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckBuilding;

impl Plugin for DeckEditorContentPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn spawn_content(
    content_container: &mut ChildBuilder,
    deck_cards: Vec<Card>,
    loaded_asset: Res<LoadedAssetMap>,
) {
    content_container
        .spawn(Node {
            display: Display::Flex,
            height: Val::Percent(100.),
            ..default()
        })
        .with_children(|content| {
            spawn_deck_card_list(content, &deck_cards, &loaded_asset);
            spawn_info(content, deck_cards.len() as u8);
            spawn_selection_list(content, &loaded_asset);
        });
}

fn spawn_deck_card_list(
    content: &mut ChildBuilder,
    deck_cards: &Vec<Card>,
    loaded_asset: &Res<LoadedAssetMap>,
) {
    content
        .spawn(Node {
            width: Val::Percent(45.),
            ..default()
        })
        .with_children(|deck_card_list_node| {
            let list_items = convert_card_into_button_map(deck_cards, &loaded_asset);
            spawn_card_list(
                deck_card_list_node,
                list_items,
                (
                    Val::Px(CARD_SIZE.truncate().x),
                    Val::Px(CARD_SIZE.truncate().y),
                ),
                Val::Percent(90.),
                7,
            );
        });
}

fn spawn_info(content: &mut ChildBuilder, deck_cards_amount: u8) {
    content.spawn(Node {
        width: Val::Percent(10.),
        ..default()
    });
}

fn spawn_selection_list(content: &mut ChildBuilder, loaded_asset: &Res<LoadedAssetMap>) {
    content
        .spawn(Node {
            width: Val::Percent(45.),
            ..default()
        })
        .with_children(|card_list_node| {
            let cards = CardDatas::get_aspect_cards();
            let list_items = convert_card_into_button_map(&cards, loaded_asset);
            spawn_card_list(
                card_list_node,
                list_items,
                (
                    Val::Px(CARD_SIZE.truncate().x),
                    Val::Px(CARD_SIZE.truncate().y),
                ),
                Val::Percent(90.),
                7,
            );
        });
}

fn convert_card_into_button_map(
    deck_cards: &Vec<Card>,
    loaded_asset: &Res<LoadedAssetMap>,
) -> Vec<(Card, ListItem)> {
    deck_cards
        .iter()
        .map(|card| {
            (
                card.clone(),
                ListItem {
                    text: "".to_string(),
                    color: Color::NONE,
                    image: ImageNode::new(loaded_asset.0.get(&card.get_id()).unwrap().clone()),
                },
            )
        })
        .collect()
}
