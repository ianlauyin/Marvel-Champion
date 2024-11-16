use bevy::prelude::*;

use crate::features::shared::{spawn_previous_button, CustomButton};

use super::state::{CollectionState, CollectionStateChangeEvent};

pub struct CollectionMenuPlugin;

impl Plugin for CollectionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionState::Menu), spawn_card_type_menu)
            .add_systems(
                Update,
                handle_button_reaction.run_if(in_state(CollectionState::Menu)),
            )
            .add_systems(OnExit(CollectionState::Menu), despawn_card_type_menu);
    }
}

#[derive(Component)]
struct CardTypeMenu;

#[derive(Component, Clone)]
enum CardTypeButton {
    Hero,
    Basic,
    Aggression,
    Leadership,
    Protection,
    Justice,
    Pool,
    Villain,
    Modular,
}

const BUTTON_MAP: [(CardTypeButton, &str, Option<Color>); 9] = [
    (CardTypeButton::Hero, "Hero", None),
    (CardTypeButton::Basic, "Basic", None),
    (
        CardTypeButton::Aggression,
        "Aggression",
        Some(Color::srgb(0.741, 0.192, 0.192)),
    ),
    (
        CardTypeButton::Leadership,
        "Leadership",
        Some(Color::srgb(0.125, 0.769, 0.882)),
    ),
    (
        CardTypeButton::Protection,
        "Protection",
        Some(Color::srgb(0.075, 0.773, 0.075)),
    ),
    (
        CardTypeButton::Justice,
        "Justice",
        Some(Color::srgb(0.871, 0.941, 0.086)),
    ),
    (
        CardTypeButton::Pool,
        "Pool",
        Some(Color::srgb(0.89, 0.149, 0.816)),
    ),
    (CardTypeButton::Villain, "Villain", None),
    (CardTypeButton::Modular, "Modular", None),
];

const BUTTON_SIZE: (Val, Val) = (Val::Px(300.), Val::Px(100.));

fn spawn_card_type_menu(mut commands: Commands) {
    commands
        .spawn((
            CardTypeMenu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(90.),
                    height: Val::Percent(90.),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
                ..default()
            },
        ))
        .with_children(|card_type_menu| {
            spawn_card_type_header(card_type_menu);
            spawn_card_type_button_group(card_type_menu);
        });
}

fn spawn_card_type_header(card_type_menu: &mut ChildBuilder) {
    card_type_menu
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(5.),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|card_type_header| {
            spawn_previous_button(card_type_header);
        });
}

fn spawn_card_type_button_group(card_type_menu: &mut ChildBuilder) {
    card_type_menu
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_self: AlignSelf::Center,
                display: Display::Grid,
                grid_template_columns: vec![RepeatedGridTrack::auto(3)],
                grid_template_rows: vec![RepeatedGridTrack::auto(3)],
                ..default()
            },
            ..default()
        })
        .with_children(|card_type_menu| {
            for (button_component, text, color) in BUTTON_MAP {
                card_type_menu
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|card_type_node| {
                        let mut button = CustomButton { text, ..default() };
                        if let Some(background_color) = color {
                            button.background_color = background_color
                        }
                        button.spawn(card_type_node).insert(button_component);
                    });
            }
        });
}

fn handle_button_reaction(
    commands: Commands,
    mut card_type_button_q: Query<(&mut BackgroundColor, &Interaction, &CardTypeButton)>,
    mut window_q: Query<&mut Window>,
) {
    let mut window = window_q.get_single_mut().unwrap();
    let mut is_hovered = false;

    for (mut background_color, interaction, card_type_button) in card_type_button_q.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                handle_button_click(commands, card_type_button.clone());
                window.cursor.icon = CursorIcon::default();
                return;
            }
            Interaction::Hovered => {
                background_color.0.set_alpha(0.5);
                is_hovered = true;
            }
            Interaction::None => {
                background_color.0.set_alpha(1.);
            }
        }
    }

    window.cursor.icon = if is_hovered {
        CursorIcon::Pointer
    } else {
        CursorIcon::default()
    };
}

fn handle_button_click(mut commands: Commands, card_type_button: CardTypeButton) {
    commands.trigger(CollectionStateChangeEvent({
        match card_type_button {
            CardTypeButton::Hero => CollectionState::Hero,
            CardTypeButton::Basic => CollectionState::Basic,
            CardTypeButton::Aggression => CollectionState::Aggression,
            CardTypeButton::Leadership => CollectionState::Leadership,
            CardTypeButton::Protection => CollectionState::Protection,
            CardTypeButton::Justice => CollectionState::Justice,
            CardTypeButton::Pool => CollectionState::Pool,
            CardTypeButton::Villain => CollectionState::Villain,
            CardTypeButton::Modular => CollectionState::Modular,
        }
    }))
}

fn despawn_card_type_menu(
    mut commands: Commands,
    card_type_menu_q: Query<Entity, With<CardTypeMenu>>,
) {
    if card_type_menu_q.is_empty() {
        warn!("Cannot find main menu when despawning");
        return;
    }
    for entity in card_type_menu_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
