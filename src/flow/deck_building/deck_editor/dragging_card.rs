use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::{
    flow::deck_building::{resource::DeckBuildingResource, state::DeckBuildingState},
    node_ui::{CardNode, MouseControlEvent},
    util::UiUtils,
};

use super::card_list_container::{CardFrom, CardList};

#[derive(Component)]
pub struct DraggingCard {
    pub(super) card_id: String,
    pub(super) card_image: Handle<Image>,
    pub(super) card_from: CardFrom,
    pub(super) position: (Val, Val),
}

pub struct DeckEditorDraggingCardPlugin;

impl Plugin for DeckEditorDraggingCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_mouse_event.run_if(in_state(DeckBuildingState::DeckEditor)),
        )
        .add_observer(on_added);
    }
}

fn on_added(
    trigger: Trigger<OnAdd, DraggingCard>,
    mut commands: Commands,
    dragging_card_q: Query<&DraggingCard>,
    z_index_q: Query<&ZIndex>,
) {
    let dragging_card = dragging_card_q.get(trigger.target()).unwrap();
    commands.entity(trigger.target()).insert((
        CardNode::small(dragging_card.card_image.clone()),
        Node {
            top: dragging_card.position.0,
            left: dragging_card.position.1,
            ..default()
        },
        UiUtils::get_largest_z_index(&z_index_q),
    ));
}

fn handle_mouse_event(
    mut mouse_event: EventReader<MouseControlEvent>,
    mut commands: Commands,
    mut dragging_card_q: Query<(Entity, &mut Node, &DraggingCard)>,
    card_list_q: Query<(&RelativeCursorPosition, &CardList)>,
    mut res: ResMut<DeckBuildingResource>,
) {
    for ev in mouse_event.read() {
        match ev {
            MouseControlEvent::Drag(delta_position_op) => {
                if let Some(delta_position) = delta_position_op {
                    if let Ok((_, dragging_card_node, _)) = dragging_card_q.single_mut() {
                        handle_drag(dragging_card_node, &delta_position);
                    } else {
                        warn!("dragging_card_transform not found");
                    }
                }
            }
            MouseControlEvent::Drop => {
                if let Ok((entity, _, dragging_card)) = dragging_card_q.single() {
                    handle_drop(
                        commands.reborrow(),
                        entity,
                        dragging_card,
                        &card_list_q,
                        &mut res,
                    );
                } else {
                    warn!("dragging_card not found");
                }
            }
            _ => {}
        }
    }
}

fn handle_drag(mut dragging_card_node: Mut<Node>, delta_position: &Vec2) {
    if let (Val::Px(y), Val::Px(x)) = (dragging_card_node.top, dragging_card_node.left) {
        dragging_card_node.top = Val::Px(y + delta_position.y);
        dragging_card_node.left = Val::Px(x + delta_position.x);
    }
}

fn handle_drop(
    mut commands: Commands,
    dragging_card_entity: Entity,
    dragging_card: &DraggingCard,
    card_list_q: &Query<(&RelativeCursorPosition, &CardList)>,
    res: &mut ResMut<DeckBuildingResource>,
) {
    for (relative_cursor_position, card_list) in card_list_q.iter() {
        if relative_cursor_position.mouse_over() {
            match card_list {
                CardList::Deck if dragging_card.card_from == CardFrom::Collection => {
                    res.add_card(&dragging_card.card_id);
                }
                CardList::Collection if dragging_card.card_from == CardFrom::Deck => {
                    res.remove_card(&dragging_card.card_id);
                }
                _ => {}
            }
        }
    }

    commands.entity(dragging_card_entity).despawn();
}
