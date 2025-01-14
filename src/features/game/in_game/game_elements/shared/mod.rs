mod card_3d;

use bevy::app::{App, Plugin};
pub use card_3d::Card3d;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(card_3d::Card3dPlugin);
    }
}
