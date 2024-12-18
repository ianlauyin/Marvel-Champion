use bevy::prelude::*;

pub fn get_largest_z_index(z_index_q: Query<&ZIndex>) -> ZIndex {
    let current_largest_z_index_i32 = z_index_q.iter().map(|z_index| z_index.0).max().unwrap();
    ZIndex(current_largest_z_index_i32 + 1)
}

pub fn global_transform_to_node_vec2(global_transform: &GlobalTransform) -> Vec2 {
    let transform = global_transform.compute_transform().translation.truncate();
    Vec2::new(transform.x, transform.y)
}
