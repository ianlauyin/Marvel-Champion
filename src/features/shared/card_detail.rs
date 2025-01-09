use std::f32::consts::PI;

use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::FocusPolicy};

use crate::{
    constants::CARD_DETAIL_SIZE,
    features::{cards::Card, shared::CustomButton},
    utils::UiUtils,
};

pub struct CardDetailPlugin;

impl Plugin for CardDetailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (on_drag, on_escape))
            .add_systems(
                Update,
                handle_card_detail_button_click.run_if(input_just_pressed(MouseButton::Left)),
            )
            .add_observer(handle_card_detail_spawn);
    }
}

#[derive(Component, Clone)]
pub struct CardDetailButton(pub Card);

#[derive(Component)]
struct EscapeButton;

#[derive(Component)]
pub struct CardDetail(pub Card);

fn handle_card_detail_spawn(
    trigger: Trigger<OnAdd, CardDetail>,
    mut commands: Commands,
    card_detail_q: Query<&CardDetail>,
    asset_server: Res<AssetServer>,
    z_index_q: Query<&ZIndex>,
) {
    let card_detail = card_detail_q.get(trigger.entity()).unwrap();
    commands
        .entity(trigger.entity())
        .insert((
            Node {
                width: Val::Px(600.),
                height: Val::Px(600.),
                position_type: PositionType::Relative,
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                top: Val::Px(0.),
                left: Val::Px(0.),
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
            UiUtils::get_largest_z_index(z_index_q),
            Interaction::default(),
        ))
        .with_children(|container| {
            spawn_escape_button(container);
            spawn_content(
                container,
                asset_server.load(card_detail.0.get_image_path()),
                card_detail.0.is_ui_vertical(),
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
            button_container.spawn((
                EscapeButton,
                CustomButton {
                    text: String::from("X"),
                    text_color: Color::srgb(0.494, 0.494, 0.494),
                    size: (Val::Px(30.), Val::Px(30.)),
                    with_border: false,
                    color: Color::srgb(0.239, 0.239, 0.239),
                    border_radius: BorderRadius::all(Val::Percent(50.)),
                    ..default()
                },
            ));
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

fn handle_card_detail_button_click(
    mut commands: Commands,
    card_button_q: Query<(&Interaction, &CardDetailButton)>,
) {
    for (interaction, card) in card_button_q.iter() {
        if *interaction == Interaction::Pressed {
            commands.spawn(CardDetail(card.0.clone()));
            return;
        }
    }
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
                let (Some(delta), Val::Px(y), Val::Px(x)) = (cursor.delta, node.top, node.left)
                else {
                    return;
                };
                node.top = Val::Px(y + delta.y);
                node.left = Val::Px(x + delta.x);
            }
            return;
        }
    }
}
