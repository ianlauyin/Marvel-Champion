use bevy::prelude::*;

mod constants;
mod features;
mod systems;
mod ui;

fn main() {
    App::new()
        .add_plugins(systems::SystemPlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(features::FeaturePlugin)
        .run();
}
