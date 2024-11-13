use std::f32::consts::PI;

use bevy::prelude::*;
use constants::CARD_SIZE;
use systems::AppState;

mod constants;
mod features;
mod systems;
mod ui;

fn main() {
    App::new()
        .add_plugins(systems::SystemPlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(features::FeaturePlugin)
        .run();
}

fn spawn_card(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let hero_card = crate::features::cards::core_spider_man::get_hero();
    let card_image_path = hero_card.card_image_path.clone();
    let card_back_image_path = hero_card.card_back_image_path.clone();
    commands
        .spawn((
            hero_card,
            GlobalTransform::default(),
            InheritedVisibility::VISIBLE,
            Transform {
                translation: Vec3::new(0., 0., 800.),
                ..default()
            },
        ))
        .with_children(|card| {
            card.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::from_size(CARD_SIZE)),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load(card_image_path)),
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(0., 0., -0.5)),
                ..default()
            });
            card.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::from_size(CARD_SIZE)),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load(card_back_image_path)),
                    ..default()
                }),
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.5),
                    rotation: Quat::from_rotation_y(PI),
                    ..default()
                },
                ..default()
            });
        });
}

fn rotate_card(
    mut card_q: Query<&mut Transform, With<crate::features::cards::builders::HeroCard>>,
) {
    let Ok(mut transform) = card_q.get_single_mut() else {
        return;
    };
    transform.rotate(Quat::from_rotation_y(0.05));
}
