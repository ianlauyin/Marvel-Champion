use bevy::prelude::*;

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

const BUTTON_MAP: [(&str, CardTypeButton, Color); 9] = [
    (
        "Hero",
        CardTypeButton::Hero,
        Color::srgb(0.235, 0.235, 0.235),
    ),
    (
        "Basic",
        CardTypeButton::Basic,
        Color::srgb(0.235, 0.235, 0.235),
    ),
    (
        "Aggression",
        CardTypeButton::Aggression,
        Color::srgb(0.741, 0.192, 0.192),
    ),
    (
        "Leadership",
        CardTypeButton::Leadership,
        Color::srgb(0.125, 0.769, 0.882),
    ),
    (
        "Protection",
        CardTypeButton::Protection,
        Color::srgb(0.075, 0.773, 0.075),
    ),
    (
        "Justice",
        CardTypeButton::Justice,
        Color::srgb(0.871, 0.941, 0.086),
    ),
    (
        "Pool",
        CardTypeButton::Pool,
        Color::srgb(0.89, 0.149, 0.816),
    ),
    (
        "Villain",
        CardTypeButton::Villain,
        Color::srgb(0.235, 0.235, 0.235),
    ),
    (
        "Modular",
        CardTypeButton::Modular,
        Color::srgb(0.235, 0.235, 0.235),
    ),
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
                    display: Display::Grid,
                    grid_template_columns: vec![RepeatedGridTrack::auto(3)],
                    grid_template_rows: vec![RepeatedGridTrack::auto(3)],
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
                ..default()
            },
        ))
        .with_children(|card_type_menu| {
            for (text, button_component, color) in BUTTON_MAP {
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
                        card_type_node
                            .spawn((
                                button_component,
                                ButtonBundle {
                                    style: Style {
                                        width: BUTTON_SIZE.0,
                                        height: BUTTON_SIZE.1,
                                        border: UiRect::all(Val::Px(2.)),
                                        display: Display::Flex,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::srgb(0.725, 0.725, 0.725)),
                                    border_radius: BorderRadius::all(Val::Px(10.)),
                                    background_color: BackgroundColor::from(color),
                                    ..default()
                                },
                            ))
                            .with_children(|button_div| {
                                button_div
                                    .spawn(TextBundle::from_section(text, TextStyle::default()));
                            });
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
