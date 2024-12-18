use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    ui::RelativeCursorPosition,
};

pub struct ScrollingListPlugin;

impl Plugin for ScrollingListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_scroll);
    }
}

#[derive(Component, Default)]
#[require(RelativeCursorPosition)]
pub struct ScrollingList {
    pub disable: bool,
    position: f32,
}

fn on_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(
        &mut ScrollingList,
        &RelativeCursorPosition,
        &mut Node,
        &ComputedNode,
        &Parent,
    )>,
    query_node: Query<&ComputedNode>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, relative_cursor_position, mut node, computed_node, parent) in
            &mut query_list
        {
            if scrolling_list.disable {
                continue;
            }
            if !relative_cursor_position.mouse_over() {
                continue;
            }
            let container_node = query_node.get(parent.get()).unwrap();

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
