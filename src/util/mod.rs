mod component_util;

pub use component_util::ComponentUtil;

use bevy::prelude::*;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {}
}
