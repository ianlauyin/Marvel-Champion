use bevy::prelude::*;

mod constants;
mod feature;
mod system;
mod ui;

fn main() {
    App::new()
        .add_plugins(ui::UI_PLUGINS)
        .add_plugins(feature::FEATURE_PLUGINS)
        .add_plugins(system::SYSTEM_PLUGINS)
        .run();
}
