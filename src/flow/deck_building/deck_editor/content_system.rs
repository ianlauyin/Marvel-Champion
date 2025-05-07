use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::{
    component::Card,
    constant::CARD_SIZE_SMALL,
    flow::deck_building::{resource::DeckBuildingResource, state::DeckBuildingState},
    node_ui::{CardDetail, CardNode, MouseControl, MouseControlEvent},
    resource::AssetLoader,
    util::UiUtils,
};

use super::content::{CardFrom, DeckContent};

pub struct DeckEditorContentSystemPlugin;

impl Plugin for DeckEditorContentSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_mouse_event.run_if(in_state(DeckBuildingState::DeckEditor)),
        );
    }
}

fn handle_mouse_event(
    mut click_ev: EventReader<MouseControlEvent>,
    mut commands: Commands,
    card_info_q: Query<(&Card<'static>, &GlobalTransform, &CardFrom), With<MouseControl>>,
    mut dragging_card_q: Query<
        (Entity, &DraggingCard, &Card<'static>, &mut Node),
        Without<MouseControl>,
    >,
    deck_component_q: Query<(&RelativeCursorPosition, &DeckContent)>,
    z_index_q: Query<&ZIndex>,
    asset_loader: Res<AssetLoader>,
    mut res: ResMut<DeckBuildingResource>,
) {
    for ev in click_ev.read() {
        match ev {
            MouseControlEvent::ShortClick(entity) => {
                if let Ok((card_info, _, _)) = card_info_q.get(*entity) {
                    commands.spawn(CardDetail::new(card_info.get_key(), card_info.is_vertical));
                } else {
                    warn!("card_info not found {:?}", entity);
                }
            }
            MouseControlEvent::StartDrag(entity) => {
                if let Ok((card_info, card_transform, card_from)) = card_info_q.get(*entity) {
                    handle_start_drag(
                        commands.reborrow(),
                        card_info,
                        card_transform,
                        card_from,
                        &asset_loader,
                        &z_index_q,
                    );
                } else {
                    warn!("card_info not found {:?}", entity);
                }
            }
            MouseControlEvent::Drag(delta_position_op) => {
                if let Some(delta_position) = delta_position_op {
                    if let Ok((_, _, _, dragging_card_node)) = dragging_card_q.single_mut() {
                        handle_drag(dragging_card_node, &delta_position);
                    } else {
                        warn!("dragging_card_transform not found");
                    }
                }
            }
            MouseControlEvent::Drop => {
                if let Ok((entity, dragging_card, card_info, _)) = dragging_card_q.single() {
                    handle_drop(
                        commands.reborrow(),
                        entity,
                        dragging_card,
                        card_info,
                        &deck_component_q,
                        &mut res,
                    );
                } else {
                    warn!("dragging_card not found");
                }
            }
        }
    }
}

#[derive(Component)]
#[require(Card)]
struct DraggingCard(CardFrom);

fn handle_start_drag(
    mut commands: Commands,
    card_info: &Card<'static>,
    global_transform: &GlobalTransform,
    card_from: &CardFrom,
    asset_loader: &Res<AssetLoader>,
    z_index_q: &Query<&ZIndex>,
) {
    let image = asset_loader.get(&card_info.get_key());
    let transform = global_transform.translation();
    commands.spawn((
        DraggingCard(card_from.clone()),
        card_info.clone(),
        CardNode::small(image.clone()),
        Node {
            top: Val::Px(transform.y - CARD_SIZE_SMALL.y / 2.),
            left: Val::Px(transform.x - CARD_SIZE_SMALL.x / 2.),
            ..default()
        },
        UiUtils::get_largest_z_index(z_index_q),
    ));
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
    card_info: &Card<'static>,
    deck_component_q: &Query<(&RelativeCursorPosition, &DeckContent)>,
    res: &mut ResMut<DeckBuildingResource>,
) {
    for (relative_cursor_position, deck_content) in deck_component_q.iter() {
        if relative_cursor_position.mouse_over() {
            match deck_content {
                DeckContent::DeckScrollingList if dragging_card.0 == CardFrom::Collection => {
                    res.add_card(card_info.id);
                }
                DeckContent::CollectionScrollingList if dragging_card.0 == CardFrom::Deck => {
                    res.remove_card(card_info.id);
                }
                _ => {}
            }
            break;
        }
    }

    commands.entity(dragging_card_entity).despawn();
}
