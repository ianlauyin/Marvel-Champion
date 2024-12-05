use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, add_camara);
    }
}

fn get_original_transform() -> Transform {
    Transform::from_xyz(0., 0., 1200.).looking_at(Vec3::ZERO, Vec3::Y)
}

fn add_camara(mut commands: Commands) {
    commands.spawn((Camera3d::default(), get_original_transform()));
    commands.spawn((
        DirectionalLight {
            illuminance: 2000.,
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            maximum_distance: 3000.,
            ..default()
        }
        .build(),
    ));
}
