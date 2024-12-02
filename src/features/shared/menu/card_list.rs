use bevy::prelude::*;

use crate::features::shared::ButtonBuilder;

use super::{ListItem, ScrollingList};

pub fn spawn_card_list<B: Component>(menu: &mut ChildBuilder, button_map: Vec<(B, ListItem)>) {
    // Scrolling List Container
    menu.spawn(Node {
        height: Val::Percent(90.),
        align_self: AlignSelf::Stretch,
        flex_direction: FlexDirection::Column,
        overflow: Overflow::clip_y(),
        ..default()
    })
    .with_children(|list_container| {
        // Moving Panel
        list_container
            .spawn((
                ScrollingList::default(),
                Node {
                    width: Val::Percent(100.),
                    display: Display::Grid,
                    padding: UiRect::all(Val::Px(30.)),
                    grid_template_columns: vec![RepeatedGridTrack::auto(8)],
                    row_gap: Val::Px(10.),
                    column_gap: Val::Px(10.),
                    ..default()
                },
            ))
            .with_children(|list_div| {
                // Items
                for (button_component, button_info) in button_map {
                    list_div
                        .spawn(Node {
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
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
                                ..default()
                            };
                            button.spawn(card_type_node).insert(button_component);
                        });
                }
            });
    });
}
