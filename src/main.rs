use std::f32::consts::PI;

use bevy::{ecs::system::SystemId, prelude::*};
use constants::CARD_SIZE;
use features::cards::{Card, CardAbility, CardDatas, Identity};
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
        // .add_systems(OnEnter(AppState::DeckBuilding), spawn_cards)
        // .add_systems(Update, rotate_card.run_if(in_state(AppState::DeckBuilding)))
        // .add_systems(
        //     OnEnter(AppState::MainMenu),
        //     (spawn_component, test_component_effect).chain(),
        // )
        .run();
}

#[derive(Bundle)]
struct CardBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}

// Only work for vertical card.
fn spawn_cards(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cards = Identity::CoreSpiderMan.get_cards();
    let mut x = -64. * 3.;
    let mut y = 89.;
    for card in cards {
        let card_image_path = card.get_image_path().to_string();
        let card_back_image_path = card.get_back_image_path().to_string();
        commands
            .spawn((
                card,
                GlobalTransform::default(),
                InheritedVisibility::VISIBLE,
                Transform {
                    translation: Vec3::new(x, y, 800.),
                    ..default()
                },
            ))
            .with_children(|card_node| {
                card_node.spawn(CardBundle {
                    mesh: Mesh3d(meshes.add(Cuboid::from_size(CARD_SIZE))),
                    material: MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load(card_image_path)),
                        perceptual_roughness: 0.8,
                        metallic: 0.5,
                        ..default()
                    })),
                    transform: Transform::from_translation(Vec3::new(0., 0., -0.5)),
                });
                card_node.spawn(CardBundle {
                    mesh: Mesh3d(meshes.add(Cuboid::from_size(CARD_SIZE))),
                    material: MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load(card_back_image_path)),
                        perceptual_roughness: 0.8,
                        metallic: 0.5,
                        ..default()
                    })),
                    transform: Transform {
                        translation: Vec3::new(0., 0., 0.5),
                        rotation: Quat::from_rotation_y(PI),
                        ..default()
                    },
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

#[derive(Component)]
struct TestEffect(SystemId);

fn spawn_component(mut commands: Commands, card_datas: Res<CardDatas>) {
    let Some(oppoenent_card) = card_datas.0.get("core_166") else {
        println!("card not found");
        return;
    };
    let Card::SideScheme(side_scheme) = oppoenent_card else {
        return;
    };
    let CardAbility::WhenRevealed(effect) = side_scheme.abilities[0] else {
        return;
    };
    let system_id = commands.register_system(effect);
    commands.spawn(TestEffect(system_id));
}

fn test_component_effect(mut commands: Commands, test_effect_q: Query<&TestEffect>) {
    if test_effect_q.is_empty() {
        warn!("No TestEffect")
    }
    for test_effect in test_effect_q.iter() {
        commands.run_system(test_effect.0);
    }
}
