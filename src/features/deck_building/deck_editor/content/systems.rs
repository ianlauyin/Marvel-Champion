use bevy::prelude::*;

use crate::{
    constants::CARD_SIZE,
    features::{
        cards::{Card, CardDatas},
        deck_building::{
            deck_editor::EditingDeck, deck_list::EditIdentity, state::DeckBuildingState,
        },
        shared::{CardDetail, Popup, ScrollingList},
    },
    systems::{
        listen_mouse_click, MouseDragDropClick, MouseDragEvent, MouseDropEvent, MousePlugin,
        MouseShortClickEvent,
    },
    ui::{NodeMove, NodeMoveRemoveEvent},
    utils::global_transform_to_node_vec2,
};

use super::{
    ui::{CardListItem, ContentContainer},
    utils::find_card_belongs,
};

#[derive(Resource)]
enum EditDeckIntent {
    Add(Card),
    Remove(Card),
}

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckBuilding;

pub struct DeckEditorContentSystemPlugin;

impl Plugin for DeckEditorContentSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MousePlugin {
            current_state: CURRENT_STATE,
        })
        .add_systems(
            Update,
            (
                handle_click,
                handle_drop,
                handle_return_card_cleanup,
                handle_edit_card_intent,
                handle_drag.after(listen_mouse_click),
            )
                .run_if(in_state(CURRENT_STATE)),
        );
    }
}

fn handle_click(
    mut click_ev: EventReader<MouseShortClickEvent>,
    mut commands: Commands,
    card_q: Query<&Card, With<MouseDragDropClick>>,
) {
    for ev in click_ev.read() {
        if let Ok(card) = card_q.get(ev.0) {
            commands.spawn(CardDetail(card.clone()));
            return;
        }
    }
}

#[derive(Component)]
struct DraggingCard {
    card_list_item: CardListItem,
    card: Card,
    initial_position: Vec2,
}

fn handle_drag(
    mut click_ev: EventReader<MouseDragEvent>,
    mut commands: Commands,
    card_q: Query<(&Card, &CardListItem, &GlobalTransform), With<MouseDragDropClick>>,
    mut drag_card_q: Query<&mut Node, With<DraggingCard>>,
    asset_server: Res<AssetServer>,
    window_q: Query<&Window>,
    mut scrolling_list_q: Query<&mut ScrollingList>,
) {
    let window = window_q.get_single().unwrap();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    for ev in click_ev.read() {
        if let Ok((card, card_list_item, global_transform)) = card_q.get(ev.entity) {
            if drag_card_q.is_empty() {
                let card_position = global_transform_to_node_vec2(global_transform);
                let image = asset_server.load(card.get_image_path());
                spawn_dragging_card(
                    commands.reborrow(),
                    card.clone(),
                    card_list_item.clone(),
                    image,
                    cursor_position,
                    card_position,
                );
                for mut scrolling_list in scrolling_list_q.iter_mut() {
                    scrolling_list.disable = true
                }
                continue;
            }
            let Ok(mut drag_card_node) = drag_card_q.get_single_mut() else {
                warn!("Should not have more than one dragging card.");
                continue;
            };
            let (Some(delta), Val::Px(y), Val::Px(x)) =
                (ev.delta_position, drag_card_node.top, drag_card_node.left)
            else {
                continue;
            };
            drag_card_node.top = Val::Px(y + delta.y);
            drag_card_node.left = Val::Px(x + delta.x);
        }
    }
}

fn spawn_dragging_card(
    mut commands: Commands,
    card: Card,
    card_list_item: CardListItem,
    image: Handle<Image>,
    spawn_position: Vec2,
    card_position: Vec2,
) {
    let top_f32 = spawn_position.y - CARD_SIZE.y / 2.;
    let left_f32 = spawn_position.x - CARD_SIZE.x / 2.;
    commands.spawn((
        DraggingCard {
            initial_position: card_position,
            card_list_item,
            card,
        },
        Node {
            width: Val::Px(CARD_SIZE.x),
            height: Val::Px(CARD_SIZE.y),
            position_type: PositionType::Relative,
            top: Val::Px(top_f32),
            left: Val::Px(left_f32),
            ..default()
        },
        ImageNode::new(image),
        BorderRadius::all(Val::Percent(5.)),
    ));
}

fn handle_drop(
    mut click_ev: EventReader<MouseDropEvent>,
    mut commands: Commands,
    dragging_card_q: Query<(Entity, &DraggingCard)>,
    card_list_q: Query<(&GlobalTransform, &ComputedNode, &ContentContainer)>,
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
            (CardListItem::Deck, ContentContainer::Selection) => {
                commands.insert_resource(EditDeckIntent::Remove(dragging_card.card.clone()));
                commands.entity(entity).despawn();
            }
            (CardListItem::Selection, ContentContainer::Deck) => {
                commands.insert_resource(EditDeckIntent::Add(dragging_card.card.clone()));
                commands.entity(entity).despawn();
            }
            _ => handle_return_card(
                commands.entity(entity),
                cursor_position,
                dragging_card.initial_position,
            ),
        }
    } else {
        handle_return_card(
            commands.entity(entity),
            cursor_position,
            dragging_card.initial_position,
        );
    }
}

fn handle_edit_card_intent(
    mut commands: Commands,
    mut editing_deck: ResMut<EditingDeck>,
    editing_card_intent_op: Option<Res<EditDeckIntent>>,
    edit_identity: Res<EditIdentity>,
    card_datas: Res<CardDatas>,
) {
    let Some(editing_card_intent) = editing_card_intent_op else {
        return;
    };
    let mut new_deck_card_ids = editing_deck.deck.card_ids.clone();
    match editing_card_intent.into_inner() {
        EditDeckIntent::Add(card) => {
            new_deck_card_ids.push(card.get_id());
        }
        EditDeckIntent::Remove(card) => {
            let Some(first_index) = new_deck_card_ids
                .iter()
                .position(|deck_card_id| **deck_card_id == card.get_id())
            else {
                warn!("Should have at least one card with same id when removing");
                return;
            };
            new_deck_card_ids.remove(first_index);
        }
    }
    let mut validator = edit_identity.0.get_validator();
    let new_deck = card_datas.from_ids(&new_deck_card_ids);
    match validator.validate_build(&new_deck) {
        Ok(()) => editing_deck.deck.card_ids = new_deck_card_ids,
        Err(message) => {
            commands.spawn(Popup::new(message));
        }
    }
    commands.remove_resource::<EditDeckIntent>();
}

fn handle_return_card(
    mut entity_commands: EntityCommands,
    current_position: Vec2,
    target_position: Vec2,
) {
    entity_commands
        .remove::<DraggingCard>()
        .insert(NodeMove::new(current_position, target_position));
}

fn handle_return_card_cleanup(
    mut commands: Commands,
    mut node_remove_event: EventReader<NodeMoveRemoveEvent>,
    node_move_q: Query<&NodeMove>,
    dragging_card_q: Query<&DraggingCard>,
    mut scrolling_list_q: Query<&mut ScrollingList>,
) {
    for ev in node_remove_event.read() {
        commands.entity(ev.0).despawn();
    }
    if node_move_q.is_empty() && dragging_card_q.is_empty() {
        for mut scrolling_list in scrolling_list_q.iter_mut() {
            scrolling_list.disable = false
        }
    }
}
