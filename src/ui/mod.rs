use bevy::app::{App, Plugin};

mod camera;
mod game_mat;
mod loading_screen;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            game_mat::GameMatPlugin,
            camera::CameraPlugin,
            loading_screen::LoadingScreenPlugin,
        ));
    }
}
