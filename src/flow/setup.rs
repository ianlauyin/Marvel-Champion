use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_plugins(WorldInspectorPlugin::new());
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d::default());
        });
    }
}
