use bevy::{prelude::*, window::WindowResolution};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_simple_text_input::TextInputPlugin;
use constant::WINDOW_RESOLUTION;

mod cards;
mod component;
mod constant;
mod flow;
mod node_ui;
mod resource;
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
        .add_plugins(TextInputPlugin)
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins((
            flow::FlowPlugin,
            node_ui::NodeUiPlugin,
            resource::ResourcePlugin,
            util::UtilPlugin,
        ))
        .run();
}
