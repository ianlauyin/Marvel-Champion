use std::collections::HashMap;

use bevy::{prelude::*, transform};

use crate::{
    constants::CARD_SIZE,
    features::{
        cards::{Card, CardAspect, CardDatas},
        deck_building::state::DeckBuildingState,
        shared::{spawn_card_detail, spawn_card_list, ListItem},
    },
    systems::{
        LoadedAssetMap, MouseDragDropClick, MouseDragEvent, MouseDropEvent, MousePlugin,
        MouseShortClickEvent,
    },
    utils::{get_card_amount, get_largest_z_index, is_cusrsor_in_container},
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
    commands: Commands,
    card_q: Query<&Card, With<MouseDragDropClick>>,
    z_index_q: Query<&ZIndex>,
    asset_server: Res<AssetServer>,
) {
    for ev in click_ev.read() {
        if let Ok(card) = card_q.get(ev.0) {
            spawn_card_detail(
                commands,
                asset_server,
                card.clone(),
                Vec2::ZERO,
                get_largest_z_index(z_index_q),
            );
            return;
        }
    }
}

#[derive(Component)]
struct DraggingCard(CardList);

fn handle_drag(
    mut click_ev: EventReader<MouseDragEvent>,
    commands: Commands,
    card_q: Query<&Card, With<MouseDragDropClick>>,
    mut drag_card_q: Query<&mut Node, With<DraggingCard>>,
    asset_server: Res<AssetServer>,
) {
    for ev in click_ev.read() {
        if let Ok(card) = card_q.get(ev.entity) {
            if drag_card_q.is_empty() {
                let image = asset_server.load(card.get_image_path());
                let card_list = find_card_in_card_list();
                spawn_dragging_card(commands, card_list, image, ev.position);
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
    card_list: CardList,
    image: Handle<Image>,
    position: Vec2,
) {
    commands.spawn((
        DraggingCard(card_list),
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
    dragging_card_q: Query<Entity, With<DraggingCard>>,
) {
    for _ in click_ev.read() {
        let Ok(card) = dragging_card_q.get_single() else {
            warn!("Should have one dragging card when drop");
            return;
        };
        commands.entity(card).despawn();
    }
}

// Util
#[derive(Bundle)]
struct DragDropCard {
    interaction: MouseDragDropClick,
    card: Card,
}

fn convert_card_into_button_map(
    cards: &Vec<Card>,
    loaded_asset: &Res<LoadedAssetMap>,
) -> Vec<(DragDropCard, ListItem)> {
    cards
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

fn find_card_in_card_list() -> CardList {
    CardList::Deck
}
