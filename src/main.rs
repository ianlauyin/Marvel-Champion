use bevy::prelude::*;

mod constants;
mod state;
mod ui;

fn main() {
    let ui_plugins = (ui::CameraPlugin, ui::SetupPlugin);
    let state_plugins = state::AssetStatePlugin;

    App::new()
        .add_plugins(ui_plugins)
        .add_plugins(state_plugins)
        .add_systems(Startup, testing)
        .run();
}

fn testing(mut commands: Commands) {}
