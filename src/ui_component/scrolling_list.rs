use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
};

#[derive(Component)]
pub struct ScrollingList {
    node: Node,
}

impl ScrollingList {
    pub fn grid(column: u16, spacing: f32) -> Self {
        Self {
            node: Node {
                align_self: AlignSelf::Stretch,
                width: Val::Percent(100.),
                overflow: Overflow::scroll_y(),
                display: Display::Grid,
                row_gap: Val::Px(spacing),
                grid_template_columns: vec![RepeatedGridTrack::auto(column)],
                ..default()
            },
        }
    }
}

pub struct ScrollingListPlugin;

impl Plugin for ScrollingListPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_scrolling_list_added)
            .add_systems(Update, (on_scrolling_list_children_changed, on_scroll));
    }
}

fn on_scrolling_list_added(
    trigger: Trigger<OnAdd, ScrollingList>,
    mut commands: Commands,
    scrolling_list_q: Query<&ScrollingList>,
) {
    let scrolling_list = scrolling_list_q.get(trigger.entity()).unwrap();
    commands
        .entity(trigger.entity())
        .insert(scrolling_list.node.clone());
}

fn on_scrolling_list_children_changed(
    mut commands: Commands,
    scrolling_list_children_q: Query<&Children, (With<ScrollingList>, Changed<Children>)>,
    children_q: Query<&Children>,
) {
    for children in scrolling_list_children_q.iter() {
        add_picking_behavior(commands.reborrow(), children, &children_q);
    }
}

fn add_picking_behavior(
    mut commands: Commands,
    children: &Children,
    children_q: &Query<&Children>,
) {
    for child in children.iter() {
        commands.entity(*child).insert(PickingBehavior {
            is_hoverable: true,
            should_block_lower: false,
        });

        if let Ok(grandchildren) = children_q.get(*child) {
            add_picking_behavior(commands.reborrow(), grandchildren, children_q);
        }
    }
}

const LINE_HEIGHT: f32 = 21.;
fn on_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (dx, dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * LINE_HEIGHT,
                mouse_wheel_event.y * LINE_HEIGHT,
            ),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        for (_, pointer_map) in hover_map.iter() {
            for (entity, _) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}
