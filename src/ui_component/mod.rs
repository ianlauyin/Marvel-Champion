mod loading_screen;
mod node_moving;

pub use loading_screen::LoadingScreen;
pub use node_moving::{NodeMove, NodeMoveRemoveEvent};

use bevy::prelude::{App, Plugin};

pub struct UiComponentPlugin;

impl Plugin for UiComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            loading_screen::LoadingScreenPlugin,
            node_moving::NodeMovingPlugin,
        ));
    }
}
