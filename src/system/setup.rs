use crate::constants::WINDOW_RESOLUTION;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_embedded_assets::EmbeddedAssetPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EmbeddedAssetPlugin::default())
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::from(WINDOW_RESOLUTION),
                    ..default()
                }),
                ..default()
            }))
            .insert_resource(ClearColor(Color::srgb(0.294, 0.675, 0.408)));
    }
}
