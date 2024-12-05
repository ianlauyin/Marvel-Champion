use bevy::prelude::*;

use crate::{
    constants::CARD_SIZE,
    features::{
        cards::Card,
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
            spawn_deck_card_list(content, deck_cards, loaded_asset);
            spawn_selection_list(content);
        });
}

fn spawn_deck_card_list(
    content: &mut ChildBuilder,
    deck_cards: Vec<Card>,
    loaded_asset: Res<LoadedAssetMap>,
) {
    content
        .spawn(Node {
            width: Val::Percent(40.),
            ..default()
        })
        .with_children(|deck_card_list_node| {
            let list_items = convert_card_into_button_map(deck_cards, loaded_asset);
            spawn_card_list(
                deck_card_list_node,
                list_items,
                (
                    Val::Px(CARD_SIZE.truncate().x),
                    Val::Px(CARD_SIZE.truncate().y),
                ),
                Val::Percent(90.),
                6,
            );
        });
}
fn spawn_selection_list(content: &mut ChildBuilder) {}

fn convert_card_into_button_map(
    deck_cards: Vec<Card>,
    loaded_asset: Res<LoadedAssetMap>,
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
