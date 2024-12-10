use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    constants::CARD_SIZE,
    features::{
        cards::{Card, CardAspect, CardDatas},
        deck_building::state::DeckBuildingState,
        shared::{CardDetailBuilder, CardListBuilder, ListItem},
    },
    systems::{
        LoadedAssetMap, MouseDragDropClick, MouseDragEvent, MouseDropEvent, MousePlugin,
        MouseShortClickEvent,
    },
    utils::{get_card_amount, get_largest_z_index, is_cusrsor_in_container},
};

use super::EditingDeck;

pub struct DeckEditorContentPlugin;

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckBuilding;

impl Plugin for DeckEditorContentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MousePlugin {
            current_state: CURRENT_STATE,
        })
        .add_systems(
            Update,
            (
                handle_click,
                handle_drag,
                handle_drop,
                handle_editing_deck_changed,
            )
                .run_if(in_state(CURRENT_STATE)),
        );
    }
}

// UI
pub fn spawn_content(
    mut commands: Commands,
    deck_cards: Vec<Card>,
    loaded_asset: Res<LoadedAssetMap>,
) -> Entity {
    let aspect_names = get_aspect_names(&deck_cards);
    let deck_card_list = spawn_deck_card_list(commands.reborrow(), &deck_cards, &loaded_asset);
    let info = spawn_info(
        commands.reborrow(),
        get_card_amount(&deck_cards),
        aspect_names,
    );
    let selection_list = spawn_selection_list(commands.reborrow(), &loaded_asset);

    commands
        .spawn(Node {
            display: Display::Flex,
            height: Val::Percent(100.),
            ..default()
        })
        .add_children(&[deck_card_list, info, selection_list])
        .id()
}

#[derive(Component, Clone, PartialEq, Eq)]
enum CardList {
    Deck,
    Selection,
}

#[derive(Component, Clone)]
enum CardListItem {
    Deck,
    Selection,
}

fn spawn_deck_card_list(
    mut commands: Commands,
    deck_cards: &Vec<Card>,
    loaded_asset: &Res<LoadedAssetMap>,
) -> Entity {
    let list_items = convert_card_into_button_map(CardListItem::Deck, deck_cards, &loaded_asset);
    let list = CardListBuilder {
        button_map: list_items.clone(),
        card_size: (
            Val::Px(CARD_SIZE.truncate().x),
            Val::Px(CARD_SIZE.truncate().y),
        ),
        height: Val::Percent(90.),
        columns: 7,
    }
    .spawn(commands.reborrow());
    commands
        .spawn((
            CardList::Deck,
            Node {
                width: Val::Percent(45.),
                ..default()
            },
        ))
        .add_child(list)
        .id()
}

fn spawn_info(
    mut commands: Commands,
    deck_cards_amount: u8,
    aspects: Vec<(String, Color)>,
) -> Entity {
    commands
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
        })
        .id()
}

fn spawn_selection_list(mut commands: Commands, loaded_asset: &Res<LoadedAssetMap>) -> Entity {
    let cards = CardDatas::get_aspect_cards();
    let list_items = convert_card_into_button_map(CardListItem::Selection, &cards, loaded_asset);
    let list = CardListBuilder {
        button_map: list_items,
        card_size: (
            Val::Px(CARD_SIZE.truncate().x),
            Val::Px(CARD_SIZE.truncate().y),
        ),
        height: Val::Percent(90.),
        columns: 7,
    }
    .spawn(commands.reborrow());

    commands
        .spawn((
            CardList::Selection,
            Node {
                width: Val::Percent(45.),
                ..default()
            },
        ))
        .add_child(list)
        .id()
}

fn handle_editing_deck_changed(
    mut commands: Commands,
    editing_deck: Res<EditingDeck>,
    card_list_q: Query<(Entity, &CardList)>,
    loaded_asset: Res<LoadedAssetMap>,
    card_datas: Res<CardDatas>,
) {
    if editing_deck.is_changed() {
        for (entity, card_list) in card_list_q.iter() {
            if *card_list == CardList::Deck {
                let cards = card_datas.from_ids(&editing_deck.deck.card_ids);
                let deck_list = CardListBuilder {
                    button_map: convert_card_into_button_map(
                        CardListItem::Deck,
                        &cards,
                        &loaded_asset,
                    ),
                    card_size: (
                        Val::Px(CARD_SIZE.truncate().x),
                        Val::Px(CARD_SIZE.truncate().y),
                    ),
                    height: Val::Percent(90.),
                    columns: 7,
                }
                .spawn(commands.reborrow());
                let mut deck_list_container = commands.entity(entity);

                deck_list_container.despawn_descendants();
                deck_list_container.add_child(deck_list);
                return;
            }
        }
    }
}

// System
fn handle_click(
    mut click_ev: EventReader<MouseShortClickEvent>,
    commands: Commands,
    card_q: Query<&Card, With<MouseDragDropClick>>,
    z_index_q: Query<&ZIndex>,
    asset_server: Res<AssetServer>,
) {
    for ev in click_ev.read() {
        if let Ok(card) = card_q.get(ev.0) {
            CardDetailBuilder {
                card: card.clone(),
                position: Vec2::ZERO,
                z_index: get_largest_z_index(z_index_q),
            }
            .spawn(commands, asset_server);
            return;
        }
    }
}

#[derive(Component)]
struct DraggingCard {
    card_list_item: CardListItem,
    card: Card,
}

fn handle_drag(
    mut click_ev: EventReader<MouseDragEvent>,
    commands: Commands,
    card_q: Query<(&Card, &CardListItem), With<MouseDragDropClick>>,
    mut drag_card_q: Query<&mut Node, With<DraggingCard>>,
    asset_server: Res<AssetServer>,
) {
    for ev in click_ev.read() {
        if let Ok((card, card_list_item)) = card_q.get(ev.entity) {
            if drag_card_q.is_empty() {
                let image = asset_server.load(card.get_image_path());
                spawn_dragging_card(
                    commands,
                    card.clone(),
                    card_list_item.clone(),
                    image,
                    ev.position,
                );
                return;
            }
            let Ok(mut drag_card_node) = drag_card_q.get_single_mut() else {
                warn!("Should not have more than one dragging card.");
                return;
            };
            let (Some(delta), Val::Px(y), Val::Px(x)) =
                (ev.delta_position, drag_card_node.top, drag_card_node.left)
            else {
                return;
            };
            drag_card_node.top = Val::Px(y + delta.y);
            drag_card_node.left = Val::Px(x + delta.x);
            return;
        }
    }
}

fn spawn_dragging_card(
    mut commands: Commands,
    card: Card,
    card_list_item: CardListItem,
    image: Handle<Image>,
    position: Vec2,
) {
    commands.spawn((
        DraggingCard {
            card_list_item,
            card,
        },
        Node {
            width: Val::Px(CARD_SIZE.x),
            height: Val::Px(CARD_SIZE.y),
            position_type: PositionType::Relative,
            top: Val::Px(position.y - CARD_SIZE.y / 2.),
            left: Val::Px(position.x - CARD_SIZE.x / 2.),
            ..default()
        },
        ImageNode::new(image),
    ));
}

fn handle_drop(
    mut click_ev: EventReader<MouseDropEvent>,
    mut commands: Commands,
    dragging_card_q: Query<(Entity, &DraggingCard)>,
    card_list_q: Query<(&GlobalTransform, &ComputedNode, &CardList)>,
    mut editing_deck: ResMut<EditingDeck>,
) {
    let Some(ev) = click_ev.read().next() else {
        return;
    };
    let Ok((entity, dragging_card)) = dragging_card_q.get_single() else {
        warn!("Should have one dragging card when drop");
        return;
    };
    let cursor_position = ev.position;
    if let Ok(drop_card_list) = find_card_belongs(&cursor_position, card_list_q) {
        match (dragging_card.card_list_item.clone(), drop_card_list) {
            (CardListItem::Deck, CardList::Selection) => {
                handle_remove_card_from_deck(&mut editing_deck, &dragging_card.card)
            }
            (CardListItem::Selection, CardList::Deck) => {
                handle_add_card_to_deck(&mut editing_deck, &dragging_card.card)
            }
            _ => {}
        }
    }
    commands.entity(entity).despawn();
}

fn handle_remove_card_from_deck(editing_deck: &mut ResMut<EditingDeck>, card: &Card) {
    let card_id = card.get_id();
    let Some(first_index) = editing_deck
        .deck
        .card_ids
        .iter()
        .position(|deck_card_id| **deck_card_id == card_id)
    else {
        warn!("Should have at least one card with same id when removing");
        return;
    };
    editing_deck.deck.card_ids.remove(first_index);
}

fn handle_add_card_to_deck(editing_deck: &mut ResMut<EditingDeck>, card: &Card) {
    editing_deck.deck.card_ids.push(card.get_id());
}

// Util
#[derive(Bundle, Clone)]
struct DragDropCard {
    belongs: CardListItem,
    interaction: MouseDragDropClick,
    card: Card,
}

fn convert_card_into_button_map(
    belongs: CardListItem,
    cards: &Vec<Card>,
    loaded_asset: &Res<LoadedAssetMap>,
) -> Vec<(DragDropCard, ListItem)> {
    cards
        .iter()
        .map(|card| {
            (
                DragDropCard {
                    belongs: belongs.clone(),
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

fn find_card_belongs(
    cursor_position: &Vec2,
    card_list_q: Query<(&GlobalTransform, &ComputedNode, &CardList)>,
) -> Result<CardList, String> {
    for (global_transform, node, card_list) in card_list_q.iter() {
        let center_position = global_transform.compute_transform().translation.truncate();
        if is_cusrsor_in_container(cursor_position, &center_position, &(node.size() / 2.)) {
            return Ok(card_list.clone());
        }
    }
    Err("The card is not in both of the container".to_string())
}
