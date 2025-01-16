use crate::{constants::GAME_MAT_ASSET, systems::AppState, utils::UiUtils};
use bevy::prelude::*;

pub struct GameMatPlugin;

impl Plugin for GameMatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::LoadingAsset), spawn_game_mat);
    }
}

const GAME_MAT_SIZE: Vec3 = Vec3::new(800., 800., 50.);

fn spawn_game_mat(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(GAME_MAT_SIZE))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: asset_server.get_handle(GAME_MAT_ASSET.path),
            perceptual_roughness: 1.,
            ..default()
        })),
        Transform {
            translation: Vec3::new(0., 0., -25.),
            rotation: Quat::from_rotation_y(UiUtils::angle_to_radian(180.)),
            ..default()
        },
    ));
}
