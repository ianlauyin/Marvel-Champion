use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    constants::CARD_SIZE,
    features::{cards::Card, shared::CardDetail},
    utils::UiUtils,
};

pub struct Card3dPlugin;

impl Plugin for Card3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_added)
            .add_observer(handle_3d_card_on_click);
    }
}

#[derive(Component)]
pub struct Card3d {
    face_up: bool,
    exhaust: bool,
    card: Card,
    position: Vec3,
}

impl Card3d {
    pub fn face_down(card: Card, position: Vec3) -> Self {
        Self {
            face_up: false,
            exhaust: false,
            card,
            position,
        }
    }

    pub fn face_up(card: Card, position: Vec3) -> Self {
        Self {
            face_up: true,
            exhaust: false,
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
    let is_horizontal_card = matches!(
        card,
        Card::MainSchemeA(_) | Card::MainSchemeB(_) | Card::SideScheme(_)
    );
    let rotation_z_radian = UiUtils::angle_to_radian(if is_horizontal_card { -90. } else { 0. });

    let mut transform = Transform::from_translation(position);
    transform.rotate_y(rotation_y_radian);
    transform.rotate_z(rotation_z_radian);

    commands
        .entity(entity)
        .insert((card.clone(), InheritedVisibility::VISIBLE, transform))
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

fn handle_3d_card_on_click(
    ev: Trigger<Pointer<Click>>,
    mut commands: Commands,
    card3d_q: Query<&Card3d>,
) {
    if let Ok(card3d) = card3d_q.get(ev.entity()) {
        if card3d.face_up {
            commands.spawn(CardDetail(card3d.card.clone()));
        }
    }
}
