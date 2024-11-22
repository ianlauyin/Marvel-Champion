use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    bevy_inspector::hierarchy::SelectedEntities,
    egui,
    quick::WorldInspectorPlugin,
    DefaultInspectorConfigPlugin,
};

pub struct DevtoolPlugin;

impl Plugin for DevtoolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new());
    }
}
