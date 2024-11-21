use bevy::{prelude::*, state::state::FreelyMutableState};

use crate::features::shared::previous_button::PreviousButtonBuilder;

#[derive(Default)]
pub struct ListItem {
    pub text: String,
    pub color: Color,
    pub image: UiImage,
}

/// Reminder: Add handle_previous_interaction(current_state) in system
pub fn spawn_menu<T: Component, S: States + FreelyMutableState, B: Component>(
    mut commands: Commands,
    component: T,
    previous_state: S,
    list_items: Vec<(B, ListItem)>,
    spawn_content: fn(&mut ChildBuilder, Vec<(B, ListItem)>),
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
            spawn_content(menu, list_items);
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
