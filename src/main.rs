use std::f32::consts::PI;

use bevy::prelude::*;
use constants::CARD_SIZE;

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

// Only work for vertical card.
fn spawn_cards(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cards = crate::features::cards::core_spider_man::get_all_cards(1);
    let mut x = -64. * 3.;
    let mut y = 89.;
    for card in cards {
        let card_image_path = card.get_card_image_path().to_string();
        let card_back_image_path = card.get_card_back_image_path().to_string();
        commands
            .spawn((
                card.clone(),
                GlobalTransform::default(),
                InheritedVisibility::VISIBLE,
                Transform {
                    translation: Vec3::new(x, y, 800.),
                    ..default()
                },
            ))
            .with_children(|card_node| {
                card_node.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::from_size(CARD_SIZE)),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load(card_image_path)),
                        ..default()
                    }),
                    transform: Transform::from_translation(Vec3::new(0., 0., -0.5)),
                    ..default()
                });
                card_node.spawn(PbrBundle {
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
        x += 64.;
        if x > 64. * 3. {
            y -= 89.;
            x = -64. * 3.;
        }
    }
}

fn rotate_card(mut card_q: Query<&mut Transform, With<crate::features::cards::Card>>) {
    for mut transform in card_q.iter_mut() {
        transform.rotate(Quat::from_rotation_y(0.05));
    }
}
