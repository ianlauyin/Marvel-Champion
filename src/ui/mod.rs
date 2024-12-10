use bevy::app::{App, Plugin};

mod camera;
mod loading_screen;
mod node_moving;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, node_moving::NodeMovingPlugin));
    }
}

pub use loading_screen::LoadingScreenPlugin;
pub use node_moving::{NodeMove, NodeMoveRemoveEvent};
