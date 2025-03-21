use bevy::{prelude::*, window::WindowResolution};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use constant::WINDOW_RESOLUTION;

mod cards;
mod component;
mod constant;
mod flow;
mod resource;
mod ui_component;
mod util;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::from(WINDOW_RESOLUTION),
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(EmbeddedAssetPlugin::default())
    .add_plugins((
        flow::FlowPlugin,
        ui_component::UiComponentPlugin,
        resource::ResourcePlugin,
        util::UtilPlugin,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}
