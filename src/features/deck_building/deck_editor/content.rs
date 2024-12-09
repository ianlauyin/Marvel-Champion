use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    constants::CARD_SIZE,
    features::{
        cards::{Card, CardAspect, CardDatas},
        deck_building::state::DeckBuildingState,
        shared::{spawn_card_list, ListItem},
    },
    systems::{
        LoadedAssetMap, MouseDragDropClick, MouseDragEvent, MouseDropEvent, MousePlugin,
        MouseShortClickEvent,
    },
    utils::get_card_amount,
};

pub struct DeckEditorContentPlugin;

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckBuilding;

impl Plugin for DeckEditorContentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MousePlugin {
            current_state: CURRENT_STATE,
        })
        .add_systems(
            Update,
            (handle_click, handle_drag, handle_drop).run_if(in_state(CURRENT_STATE)),
        );
    }
}

// UI
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
            let aspect_names = get_aspect_names(&deck_cards);
            spawn_deck_card_list(content, &deck_cards, &loaded_asset);
            spawn_info(content, get_card_amount(&deck_cards), aspect_names);
            spawn_selection_list(content, &loaded_asset);
        });
}

#[derive(Component)]
enum CardList {
    Deck,
    Selection,
}

fn spawn_deck_card_list(
    content: &mut ChildBuilder,
    deck_cards: &Vec<Card>,
    loaded_asset: &Res<LoadedAssetMap>,
) {
    content
        .spawn((
            CardList::Deck,
            Node {
                width: Val::Percent(45.),
                ..default()
            },
        ))
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

fn spawn_info(content: &mut ChildBuilder, deck_cards_amount: u8, aspects: Vec<(String, Color)>) {
    content
        .spawn(Node {
            width: Val::Percent(10.),
            padding: UiRect::vertical(Val::Px(30.)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|info_container| {
            info_container.spawn((
                Text::new(format!("Cards: {}", deck_cards_amount)),
                Node {
                    margin: UiRect::bottom(Val::Px(50.)),
                    ..default()
                },
            ));
            info_container.spawn(Text::new("Aspects:"));
            for (aspect, color) in aspects {
                info_container.spawn((
                    Text::new(aspect),
                    TextColor(color),
                    TextFont::from_font_size(16.),
                ));
            }
        });
}

fn spawn_selection_list(content: &mut ChildBuilder, loaded_asset: &Res<LoadedAssetMap>) {
    content
        .spawn((
            CardList::Selection,
            Node {
                width: Val::Percent(45.),
                ..default()
            },
        ))
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

// System
fn handle_click(
    mut click_ev: EventReader<MouseShortClickEvent>,
    card_q: Query<&Card, With<MouseDragDropClick>>,
) {
    for ev in click_ev.read() {
        if let Ok(card) = card_q.get(ev.0) {
            println!("{}", card.get_name());
        }
    }
}

fn handle_drag(
    mut click_ev: EventReader<MouseDragEvent>,
    card_q: Query<&Card, With<MouseDragDropClick>>,
) {
    for ev in click_ev.read() {
        if let Ok(card) = card_q.get(ev.entity) {
            println!("{}", card.get_name());
            println!("{}", ev.position);
            println!("drag");
        }
    }
}

fn handle_drop(
    mut click_ev: EventReader<MouseDropEvent>,
    card_q: Query<&Card, With<MouseDragDropClick>>,
) {
    for ev in click_ev.read() {
        if let Ok(card) = card_q.get(ev.entity) {
            println!("{}", card.get_name());
            println!("{}", ev.position);
            println!("drop");
        }
    }
}

// Util
#[derive(Bundle)]
struct DragDropCard {
    interaction: MouseDragDropClick,
    card: Card,
}

fn convert_card_into_button_map(
    deck_cards: &Vec<Card>,
    loaded_asset: &Res<LoadedAssetMap>,
) -> Vec<(DragDropCard, ListItem)> {
    deck_cards
        .iter()
        .map(|card| {
            (
                DragDropCard {
                    interaction: MouseDragDropClick::default(),
                    card: card.clone(),
                },
                ListItem {
                    text: "".to_string(),
                    color: Color::NONE,
                    image: ImageNode::new(loaded_asset.0.get(&card.get_id()).unwrap().clone()),
                },
            )
        })
        .collect()
}

fn get_aspect_names(deck_cards: &Vec<Card>) -> Vec<(String, Color)> {
    let mut hash_map: HashMap<String, Color> = HashMap::new();
    for card in deck_cards {
        let Ok(aspect) = card.get_aspect() else {
            continue;
        };
        match aspect {
            CardAspect::Justice => {
                hash_map.insert(aspect.to_string(), Color::srgb(0.871, 0.941, 0.086));
            }
            CardAspect::Aggression => {
                hash_map.insert(aspect.to_string(), Color::srgb(0.741, 0.192, 0.192));
            }
            CardAspect::Protection => {
                hash_map.insert(aspect.to_string(), Color::srgb(0.075, 0.773, 0.075));
            }
            CardAspect::Leadership => {
                hash_map.insert(aspect.to_string(), Color::srgb(0.125, 0.769, 0.882));
            }
            CardAspect::Pool => {
                hash_map.insert(aspect.to_string(), Color::srgb(0.89, 0.149, 0.816));
            }
            _ => continue,
        }
    }
    hash_map.into_iter().collect()
}
