use bevy::prelude::*;
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
    }
}

const BACKGROUND_SIZE: Vec3 = Vec3::new(1000., 800., 50.);

fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(BACKGROUND_SIZE)),
        transform: Transform::from_translation(Vec3::new(0., 0., -25.)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0., 0.522, 0.157),
            ..default()
        }),
        ..default()
    });
}
