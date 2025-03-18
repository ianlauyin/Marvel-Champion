mod component_util;
// mod decks_storage_util;
// mod deck_validator;
mod mouse_control_util;
mod ui_util;

pub use component_util::ComponentUtil;
pub use mouse_control_util::{MouseControl, MouseControlEvent};
pub use ui_util::UiUtils;

use bevy::prelude::*;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(mouse_control_util::MouseControlUtilPlugin);
    }
}
