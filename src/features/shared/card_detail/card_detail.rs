use bevy::{prelude::*, ui::FocusPolicy};

use crate::features::{cards::Card, shared::ButtonBuilder};

pub struct CardDetailFramePlugin;

impl Plugin for CardDetailFramePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_escape);
    }
}

#[derive(Component)]
struct EscapeButton;

pub fn spawn_card_detail(mut commands: Commands, card: Card, z_index: ZIndex) {
    commands
        .spawn(NodeBundle {
            focus_policy: FocusPolicy::Block,
            style: Style {
                width: Val::Percent(90.),
                height: Val::Percent(90.),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            border_radius: BorderRadius::all(Val::Px(10.)),
            z_index,
            background_color: BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
            ..default()
        })
        .with_children(|container| {
            spawn_escape_button(container);
        });
}

fn spawn_escape_button(children_builder: &mut ChildBuilder) {
    children_builder
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.),
                right: Val::Px(5.),
                ..default()
            },
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
