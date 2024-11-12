use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, add_camara);
    }
}

fn get_original_transform() -> Transform {
    Transform::from_xyz(0., 0., 1200.).looking_at(Vec3::ZERO, Vec3::Y)
}

fn get_gameplay_transform() -> Transform {
    Transform::from_xyz(0., -450., 1200.).looking_at(Vec3::new(0., -80., 0.), Vec3::Y)
}

fn add_camara(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: get_original_transform(),
        ..default()
    });
    commands.insert_resource(AmbientLight {
        brightness: 1.,
        color: Color::WHITE,
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 3000.,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 1200.)),
        ..default()
    });
}
