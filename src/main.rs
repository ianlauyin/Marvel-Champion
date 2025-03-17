use bevy::{prelude::*, window::WindowResolution};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use constant::WINDOW_RESOLUTION;

mod component;
mod constant;
mod flow;
mod resource;
mod ui_component;
mod util;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
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
            component::ComponentPlugin,
            ui_component::UiComponentPlugin,
            resource::ResourcePlugin,
            util::UtilPlugin,
        ))
        .run();
}
