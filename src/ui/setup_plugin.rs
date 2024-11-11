use crate::constants::WINDOW_RESOLUTION;
use bevy::prelude::*;
use bevy::window::WindowResolution;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::from(WINDOW_RESOLUTION),
                ..default()
            }),
            ..default()
        }));
    }
}
