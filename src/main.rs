use bevy::prelude::*;
use constants::CARD_SIZE;

mod constants;
mod state;
mod ui;

fn main() {
    let ui_plugins = (ui::CameraPlugin, ui::SetupPlugin, ui::BackgroundPlugin);
    let state_plugins = state::AssetStatePlugin;

    App::new()
        .add_plugins(ui_plugins)
        .add_plugins(state_plugins)
        .add_systems(Startup, spawn_testing_card)
        .run();
}

#[derive(Component)]
struct Card;

fn spawn_testing_card(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::from_size(CARD_SIZE)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("cards/card_backs/hero_card_back.png")),
                ..default()
            }),
            ..default()
        })
        .insert(Card);
}
