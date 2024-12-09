use bevy::prelude::*;

pub fn is_cusrsor_in_container(
    cursor_position: Vec2,
    container_center: Vec2,
    container_half_size: Vec2,
) -> bool {
    let container_left_bottom_bound = container_center - container_half_size;
    let container_right_top_bound = container_center + container_half_size;

    cursor_position.x >= container_left_bottom_bound.x
        && cursor_position.x <= container_right_top_bound.x
        && cursor_position.y >= container_left_bottom_bound.y
        && cursor_position.y <= container_right_top_bound.y
}

pub fn get_largest_z_index(z_index_q: Query<&ZIndex>) -> ZIndex {
    let current_largest_z_index_i32 = z_index_q.iter().map(|z_index| z_index.0).max().unwrap();
    ZIndex(current_largest_z_index_i32 + 1)
}
