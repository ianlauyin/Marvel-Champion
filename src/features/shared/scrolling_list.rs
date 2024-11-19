use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::constants::WINDOW_RESOLUTION;
pub struct ScrollingListPlugin;

impl Plugin for ScrollingListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_scroll);
    }
}

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

fn on_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    window_q: Query<&Window>,
    query_node: Query<(&Node, &Transform)>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let Some(cursor_position) = window_q.get_single().unwrap().cursor_position() else {
            return;
        };
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let (container_node, container_transform) = query_node.get(parent.get()).unwrap();
            if !is_cusrsor_in_container(
                cursor_position,
                WINDOW_RESOLUTION / 2.,
                container_transform.translation.truncate(),
                container_node.size() / 2.,
            ) {
                return;
            }

            let items_height = list_node.size().y;
            let max_scroll = (items_height - container_node.size().y).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

fn is_cusrsor_in_container(
    cursor_position: Vec2,
    window_half_size: Vec2,
    container_center: Vec2,
    container_half_size: Vec2,
) -> bool {
    let position = cursor_position - window_half_size;

    let container_left_bottom_bound = container_center - container_half_size;
    let container_right_top_bound = container_center + container_half_size;

    position.x >= container_left_bottom_bound.x
        && position.x <= container_right_top_bound.x
        && position.y >= container_left_bottom_bound.y
        && position.y <= container_right_top_bound.y
}
