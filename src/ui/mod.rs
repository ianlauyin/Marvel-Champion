use bevy::app::{App, Plugin};

mod camera;
mod loading_screen;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, loading_screen::LoadingScreenPlugin));
    }
}
