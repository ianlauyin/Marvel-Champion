use crate::system::{AppState, LoadAsset, StateLoading};
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct GameMatPlugin;

impl Plugin for GameMatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_game_mat)
            .add_systems(OnExit(AppState::LoadingAsset), spawn_game_mat);
    }
}

const GAME_MAT_SIZE: Vec3 = Vec3::new(1500., 1000., 50.);
const GAME_MAT_PATH: &str = "embedded://game_mat.png";

fn load_game_mat(mut load_asset: ResMut<LoadAsset>, asset_server: Res<AssetServer>) {
    load_asset
        .loading_image_handles
        .push(asset_server.load(GAME_MAT_PATH));
    load_asset.state_loading = Some(StateLoading::AppState);
}

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
            base_color_texture: asset_server.get_handle(GAME_MAT_PATH),
            perceptual_roughness: 1.,
            ..default()
        }),
        ..default()
    });
}
