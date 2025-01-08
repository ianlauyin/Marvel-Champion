use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

pub struct CameraPlugin;

#[derive(Resource, Default)]
pub struct GamePlayCamera {
    is_play_mode: bool,
    rotate_count: Option<usize>,
}

const TOTAL_COUNTER: usize = 50;

impl GamePlayCamera {
    pub fn toggle(&mut self) {
        self.is_play_mode = !self.is_play_mode;
        self.rotate_count = Some(TOTAL_COUNTER);
    }

    pub fn move_tick(&mut self, camera_transfom: &mut Transform) {
        if let Some(rotate_count) = self.rotate_count {
            let angle = if self.is_play_mode { 0.01 } else { -0.01 };
            camera_transfom.rotate_around(Vec3::new(0., 100., -100.), Quat::from_rotation_x(angle));
            let look_at_point_y = if self.is_play_mode {
                100 / TOTAL_COUNTER * (TOTAL_COUNTER - rotate_count)
            } else {
                100 / TOTAL_COUNTER * rotate_count
            } as f32;
            camera_transfom.look_at(Vec3::new(0., -look_at_point_y, 0.), Vec3::Y);
            if rotate_count == 0 {
                self.rotate_count = None;
            } else {
                self.rotate_count = Some(rotate_count - 1);
            }
        }
    }
}

const ORIGINAL_DESITINATION: Vec3 = Vec3::new(0., 0., 1200.);
const ORIGINAL_LOOKING_AT: Vec3 = Vec3::ZERO;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GamePlayCamera>()
            .add_systems(PreStartup, add_camara)
            .add_systems(Update, handle_camera_rotate);
    }
}

fn add_camara(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform {
            translation: ORIGINAL_DESITINATION,
            ..default()
        }
        .looking_at(ORIGINAL_LOOKING_AT, Vec3::Y),
    ));
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

fn handle_camera_rotate(
    mut game_play_camera: ResMut<GamePlayCamera>,
    mut camera_q: Query<&mut Transform, With<Camera3d>>,
) {
    let Ok(mut transform) = camera_q.get_single_mut() else {
        return;
    };
    game_play_camera.move_tick(&mut transform);
}
