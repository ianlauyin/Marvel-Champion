use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, add_camara);
    }
}

fn add_camara(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., -450., 800.)
            .looking_at(Vec3::new(0., -80., 0.), Vec3::Y),

        ..default()
    });
    commands.insert_resource(AmbientLight {
        brightness: 1.,
        color: Color::WHITE,
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 800.)),
        ..default()
    });
}
