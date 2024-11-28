use crate::{constants::GAME_MAT_ASSET, systems::AppState};
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct GameMatPlugin;

impl Plugin for GameMatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::LoadingAsset), spawn_game_mat);
    }
}

const GAME_MAT_SIZE: Vec3 = Vec3::new(1000., 1000., 50.);

fn spawn_game_mat(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(GAME_MAT_SIZE)),
        transform: Transform {
            translation: Vec3::new(0., 0., -25.),
            rotation: Quat::from_axis_angle(Vec3::new(0., 1., 0.), PI),
            ..default()
        },
        material: materials.add(StandardMaterial {
            base_color_texture: asset_server.get_handle(GAME_MAT_ASSET.path),
            perceptual_roughness: 1.,
            ..default()
        }),
        ..default()
    });
}
