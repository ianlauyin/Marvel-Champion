use bevy::math::Vec2;

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
