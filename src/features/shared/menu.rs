use bevy::{prelude::*, state::state::FreelyMutableState};

use super::{ButtonBuilder, PreviousButtonBuilder, ScrollingList};

pub fn spawn_menu<T: Component, S: States + FreelyMutableState, B: Component>(
    mut commands: Commands,
    component: T,
    previous_state: S,
    button_map: Vec<(B, &str, Option<Color>)>,
) {
    commands
        .spawn((
            component,
            NodeBundle {
                style: Style {
                    width: Val::Percent(90.),
                    height: Val::Percent(90.),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
                ..default()
            },
        ))
        .with_children(|menu| {
            spawn_header(menu, previous_state);
            spawn_list(menu, button_map);
        });
}

fn spawn_header<S: States + FreelyMutableState>(menu: &mut ChildBuilder, previous_state: S) {
    menu.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(10.),
            padding: UiRect {
                left: Val::Px(10.),
                top: Val::Px(10.),
                bottom: Val::Px(10.),
                ..default()
            },
            ..default()
        },
        ..default()
    })
    .with_children(|header| {
        PreviousButtonBuilder(previous_state).spawn(header);
    });
}

fn spawn_list<B: Component>(menu: &mut ChildBuilder, button_map: Vec<(B, &str, Option<Color>)>) {
    menu.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(100.),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            overflow: Overflow::clip_y(),
            ..default()
        },
        ..default()
    })
    .with_children(|list_container| {
        list_container
            .spawn((
                ScrollingList::default(),
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        align_self: AlignSelf::Center,
                        display: Display::Grid,
                        grid_template_columns: vec![RepeatedGridTrack::auto(1)],
                        row_gap: Val::Px(40.),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|list_div| {
                for (button_component, text, color) in button_map {
                    list_div
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
                            let mut button = ButtonBuilder { text, ..default() };
                            if let Some(background_color) = color {
                                button.background_color = background_color
                            }
                            button.spawn(card_type_node).insert(button_component);
                        });
                }
            });
    });
}
