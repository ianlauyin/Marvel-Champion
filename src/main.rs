use bevy::prelude::*;

mod constants;
mod system;
mod ui;

fn main() {
    App::new()
        .add_plugins(ui::UI_PLUGINS)
        .add_plugins(system::SYSTEM_PLUGINS)
        .run();
}
