use bevy::prelude::*;

use crate::features::shared::ButtonBuilder;

use super::ListItem;

pub fn spawn_list<B: Component>(menu: &mut ChildBuilder, button_map: Vec<(B, ListItem)>) {
    menu.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(70.),
            width: Val::Percent(100.),
            align_self: AlignSelf::Center,
            display: Display::Grid,
            grid_template_columns: vec![RepeatedGridTrack::auto(3)],
            row_gap: Val::Px(40.),
            ..default()
        },
        ..default()
    })
    .with_children(|container| {
        for (button_component, button_info) in button_map {
            container
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
                    let ListItem { text, color, image } = button_info;
                    let button = ButtonBuilder {
                        text,
                        color,
                        image,
                        ..default()
                    };
                    button.spawn(card_type_node).insert(button_component);
                });
        }
    });
}
