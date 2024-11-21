use bevy::prelude::*;

use crate::features::{cards::Card, shared::ButtonBuilder};

use super::{ListItem, ScrollingList};

pub fn spawn_card_list<B: Component>(menu: &mut ChildBuilder, button_map: Vec<(B, ListItem)>) {
    menu.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(100.),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            padding: UiRect::horizontal(Val::Px(10.)),
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
                        grid_template_columns: vec![RepeatedGridTrack::auto(8)],
                        row_gap: Val::Px(10.),
                        column_gap: Val::Px(10.),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|list_div| {
                for (button_component, button_info) in button_map {
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
                            let ListItem { text, color, image } = button_info;
                            let button = ButtonBuilder {
                                text,
                                color,
                                image,
                                size: (Val::Px(128.), Val::Px(178.)),
                                with_border: false,
                            };
                            button.spawn(card_type_node).insert(button_component);
                        });
                }
            });
    });
}
