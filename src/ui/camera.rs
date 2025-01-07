use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

pub struct CameraPlugin;

#[derive(Resource, Default)]
pub struct GamePlayCamera {
    is_play_mode: bool,
}

const TOTAL_COUNTER: usize = 10;

impl GamePlayCamera {
    pub fn toggle(&mut self) {
        self.is_play_mode = !self.is_play_mode;
    }

    pub fn get_transform(&self) -> Transform {
        Transform {
            translation: if self.is_play_mode {
                PLAY_MODE_DESITINATION
            } else {
                ORIGINAL_DESITINATION
            },
            ..default()
        }
        .looking_at(
            if self.is_play_mode {
                PLAY_MODE_LOOKING_AT
            } else {
                ORIGINAL_LOOKING_AT
            },
            Vec3::Y,
        )
    }
}

const ORIGINAL_DESITINATION: Vec3 = Vec3::new(0., 0., 1200.);
const ORIGINAL_LOOKING_AT: Vec3 = Vec3::ZERO;

const PLAY_MODE_DESITINATION: Vec3 = Vec3::new(0., -500., 1100.);
const PLAY_MODE_LOOKING_AT: Vec3 = Vec3::new(0., -85., 0.);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GamePlayCamera>()
            .add_systems(PreStartup, add_camara)
            .add_systems(Update, handle_camera_movement);
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

fn handle_camera_movement(
    game_play_camera: ResMut<GamePlayCamera>,
    mut camera_q: Query<&mut Transform, With<Camera3d>>,
) {
    let Ok(mut transform) = camera_q.get_single_mut() else {
        return;
    };

    let target_transform = game_play_camera.get_transform();
    if target_transform != *transform {
        *transform = target_transform;
    }
}
