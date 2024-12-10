use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::utils::is_cusrsor_in_container;

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
    mut query_list: Query<(&mut ScrollingList, &mut Node, &ComputedNode, &Parent)>,
    window_q: Query<&Window>,
    query_node: Query<(&ComputedNode, &GlobalTransform)>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let Some(cursor_position) = window_q.get_single().unwrap().cursor_position() else {
            return;
        };
        for (mut scrolling_list, mut node, computed_node, parent) in &mut query_list {
            let (container_node, global_transform) = query_node.get(parent.get()).unwrap();
            let container_transform = global_transform.compute_transform();

            if !is_cusrsor_in_container(
                &cursor_position,
                &container_transform.translation.truncate(),
                &(container_node.size() / 2.),
            ) {
                continue;
            }

            let items_height = computed_node.size().y;
            let max_scroll = (items_height - container_node.size().y).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            node.top = Val::Px(scrolling_list.position);
        }
    }
}
