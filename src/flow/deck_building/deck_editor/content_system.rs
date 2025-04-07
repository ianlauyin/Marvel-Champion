use bevy::prelude::*;

use crate::{
    component::card::CardBasic,
    flow::{deck_building::resource::DeckBuildingResource, state::AppState},
    resource::AssetLoader,
    ui_component::{Card, CardDetail, CardSprite, CARD_SIZE_SMALL},
    util::{MouseControl, MouseControlEvent, UiUtils},
};

pub struct DeckEditorContentSystemPlugin;

impl Plugin for DeckEditorContentSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_mouse_event).run_if(in_state(AppState::DeckBuilding)),
        );
    }
}

fn handle_mouse_event(
    mut click_ev: EventReader<MouseControlEvent>,
    mut commands: Commands,
    card_info_q: Query<(&CardBasic<'static>, &GlobalTransform), With<MouseControl>>,
    mut dragging_card_q: Query<(Entity, &mut Node), (With<DraggingCard>, Without<MouseControl>)>,
    mut res: ResMut<DeckBuildingResource>,
    asset_loader: Res<AssetLoader>,
    z_index_q: Query<&ZIndex>,
) {
    for ev in click_ev.read() {
        match ev {
            MouseControlEvent::ShortClick(entity) => {
                if let Ok((card_info, _)) = card_info_q.get(*entity) {
                    commands.spawn(CardDetail::new(card_info.get_key(), card_info.is_vertical));
                } else {
                    warn!("card_info not found {:?}", entity);
                }
            }
            MouseControlEvent::StartDrag(entity) => {
                if let Ok((card_info, card_transform)) = card_info_q.get(*entity) {
                    handle_start_drag(
                        commands.reborrow(),
                        card_info,
                        card_transform,
                        &asset_loader,
                        &z_index_q,
                    );
                } else {
                    warn!("card_info not found {:?}", entity);
                }
            }
            MouseControlEvent::Drag(delta_position_op) => {
                if let Some(delta_position) = delta_position_op {
                    if let Ok((_, dragging_card_node)) = dragging_card_q.get_single_mut() {
                        handle_drag(dragging_card_node, &delta_position);
                    } else {
                        warn!("dragging_card_transform not found");
                    }
                }
            }
            MouseControlEvent::Drop => {
                if let Ok((dragging_card, _)) = dragging_card_q.get_single() {
                    handle_drop(commands.reborrow(), dragging_card);
                } else {
                    warn!("dragging_card not found");
                }
            }
        }
    }
}

#[derive(Component)]
#[require(CardBasic)]
struct DraggingCard;

fn handle_start_drag(
    mut commands: Commands,
    card_info: &CardBasic<'static>,
    global_transform: &GlobalTransform,
    asset_loader: &Res<AssetLoader>,
    z_index_q: &Query<&ZIndex>,
) {
    let image = asset_loader.get(&card_info.get_key());
    let transform = global_transform.translation();
    commands.spawn((
        DraggingCard,
        card_info.clone(),
        Card::small(image.clone()),
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

fn handle_drop(mut commands: Commands, dragging_card: Entity) {
    commands.entity(dragging_card).despawn_recursive();
}
