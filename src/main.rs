use bevy::prelude::*;

mod constants;
mod feature;
mod system;
mod ui;

fn main() {
    App::new()
        .add_plugins(system::SystemPlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(feature::FeaturePlugin)
        .run();
}
