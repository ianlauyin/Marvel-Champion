use bevy::prelude::*;

pub struct UiUtils;

impl UiUtils {
    pub fn get_largest_z_index(z_index_q: Query<&ZIndex>) -> ZIndex {
        let current_largest_z_index_i32 = z_index_q.iter().map(|z_index| z_index.0).max().unwrap();
        ZIndex(current_largest_z_index_i32 + 1)
    }
}
