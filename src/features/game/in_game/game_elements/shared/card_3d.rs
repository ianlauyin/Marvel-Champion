use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{constants::CARD_SIZE, features::cards::Card, utils::UiUtils};

pub struct Card3dPlugin;

impl Plugin for Card3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_added);
    }
}

#[derive(Component)]
pub struct Card3d {
    face_up: bool,
    card: Card,
    position: Vec3,
}

impl Card3d {
    pub fn face_down(card: Card, position: Vec3) -> Self {
        Self {
            face_up: false,
            card,
            position,
        }
    }

    pub fn face_up(card: Card, position: Vec3) -> Self {
        Self {
            face_up: true,
            card,
            position,
        }
    }
}

fn on_added(
    on_added: Trigger<OnAdd, Card3d>,
    mut commands: Commands,
    card_3d_q: Query<&Card3d>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let entity = on_added.entity();
    let card_3d = card_3d_q.get(entity).unwrap();
    let card = &card_3d.card;
    let position = card_3d
        .position
        .with_z(card_3d.position.z + CARD_SIZE.z / 2.);
    let rotation_y_radian = UiUtils::angle_to_radian(if card_3d.face_up { 180. } else { 0. });
    commands
        .entity(entity)
        .insert((
            card.clone(),
            GlobalTransform::default(),
            InheritedVisibility::VISIBLE,
            Transform {
                translation: position,
                rotation: Quat::from_rotation_y(rotation_y_radian),
                ..default()
            },
        ))
        .with_children(|card_node| {
            card_node.spawn((
                Mesh3d(meshes.add(Cuboid::from_size(CARD_SIZE))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load(card.get_image_path())),
                    perceptual_roughness: 0.8,
                    metallic: 0.5,
                    ..default()
                })),
                Transform::from_translation(Vec3::new(0., 0., -0.5)),
            ));
            card_node.spawn((
                Mesh3d(meshes.add(Cuboid::from_size(CARD_SIZE))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load(card.get_back_image_path())),
                    perceptual_roughness: 0.8,
                    metallic: 0.5,
                    ..default()
                })),
                Transform {
                    translation: Vec3::new(0., 0., 0.5),
                    rotation: Quat::from_rotation_y(PI),
                    ..default()
                },
            ));
        });
}
