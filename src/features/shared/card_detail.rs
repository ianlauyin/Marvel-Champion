use std::f32::consts::PI;

use bevy::{prelude::*, ui::FocusPolicy};

use crate::{
    constants::CARD_DETAIL_SIZE,
    features::{cards::Card, shared::ButtonBuilder},
};

pub struct CardDetailPlugin;

impl Plugin for CardDetailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (on_drag, on_escape));
    }
}

#[derive(Component)]
struct CardDetail;

#[derive(Component)]
struct EscapeButton;

pub fn spawn_card_detail(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    card: Card,
    position: Vec2,
    z_index: ZIndex,
) {
    commands
        .spawn((
            Node {
                width: Val::Px(600.),
                height: Val::Px(600.),
                position_type: PositionType::Relative,
                bottom: Val::Px(position.y),
                left: Val::Px(position.x),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            FocusPolicy::Block,
            BorderColor::from(Color::WHITE),
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
            z_index,
            Interaction::default(),
            CardDetail,
        ))
        .with_children(|container| {
            spawn_escape_button(container);
            let vertical = match card {
                Card::MainSchemeA(_) | Card::MainSchemeB(_) | Card::SideScheme(_) => false,
                _ => true,
            };
            spawn_content(
                container,
                asset_server.load(card.get_card_image_path()),
                vertical,
            );
        });
}

fn spawn_escape_button(children_builder: &mut ChildBuilder) {
    children_builder
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            right: Val::Px(5.),
            ..default()
        })
        .with_children(|button_container| {
            ButtonBuilder {
                text: String::from("X"),
                text_color: Color::srgb(0.494, 0.494, 0.494),
                size: (Val::Px(30.), Val::Px(30.)),
                with_border: false,
                color: Color::srgb(0.239, 0.239, 0.239),
                border_radius: BorderRadius::all(Val::Percent(50.)),
                ..default()
            }
            .spawn(button_container)
            .insert(EscapeButton);
        });
}

fn spawn_content(container: &mut ChildBuilder, card_image: Handle<Image>, vertical: bool) {
    container.spawn((
        Node {
            width: Val::Px(CARD_DETAIL_SIZE.x),
            height: Val::Px(CARD_DETAIL_SIZE.y),

            ..default()
        },
        Transform::from_rotation(Quat::from_axis_angle(
            Vec3::Z,
            if vertical { 0. } else { PI / 2. },
        )),
        BorderRadius::all(Val::Px(20.)),
        ImageNode::new(card_image),
    ));
}

fn on_escape(
    mut commands: Commands,
    escape_button_q: Query<(&Interaction, Entity), With<EscapeButton>>,
    parent_q: Query<&Parent>,
) {
    for (interaction, escape_button) in escape_button_q.iter() {
        if *interaction == Interaction::Pressed {
            let card_detail = parent_q.iter_ancestors(escape_button).last().unwrap();
            commands.entity(card_detail).despawn_recursive();
            return;
        }
    }
}

fn on_drag(
    mut cursor_ev: EventReader<CursorMoved>,
    mut card_detail_q: Query<(&Interaction, &mut Node), With<CardDetail>>,
) {
    for (inteaction, mut node) in &mut card_detail_q {
        if *inteaction == Interaction::Pressed {
            for cursor in cursor_ev.read() {
                let (Some(delta), Val::Px(y), Val::Px(x)) = (cursor.delta, node.bottom, node.left)
                else {
                    return;
                };
                node.bottom = Val::Px(y - delta.y);
                node.left = Val::Px(x + delta.x);
            }
            return;
        }
    }
}
