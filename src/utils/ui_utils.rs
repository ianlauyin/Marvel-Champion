use std::f32::consts::PI;

use bevy::{prelude::Query, ui::ZIndex};

pub struct UiUtils;

impl UiUtils {
    pub fn angle_to_radian(angle: f32) -> f32 {
        angle * PI / 180.
    }

    pub fn get_largest_z_index(z_index_q: Query<&ZIndex>) -> ZIndex {
        let current_largest_z_index_i32 =
            z_index_q.iter().map(|z_index| z_index.0).max().unwrap_or(0);
        ZIndex(current_largest_z_index_i32 + 1)
    }
}
