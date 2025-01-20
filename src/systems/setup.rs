use crate::constants::WINDOW_RESOLUTION;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_embedded_assets::EmbeddedAssetPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::from(WINDOW_RESOLUTION),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((EmbeddedAssetPlugin::default(), MeshPickingPlugin));
    }
}
