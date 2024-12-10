use bevy::prelude::*;

use crate::features::shared::ButtonBuilder;

use super::super::{ListItem, ScrollingList};

pub fn spawn_list<B: Component>(mut commands: Commands, button_map: Vec<(B, ListItem)>) -> Entity {
    // Scrolling List Container
    commands
        .spawn(Node {
            height: Val::Percent(90.),
            align_self: AlignSelf::Stretch,
            flex_direction: FlexDirection::Column,
            overflow: Overflow::clip_y(),
            ..default()
        })
        .with_children(|list_container| {
            let row_gap = Val::Px(match button_map.len() {
                0..10 => 80.,
                10..13 => 50.,
                _ => 30.,
            });
            // Moving Panel
            list_container
                .spawn((
                    ScrollingList::default(),
                    Node {
                        width: Val::Percent(100.),
                        display: Display::Grid,
                        padding: UiRect::all(Val::Px(30.)),
                        grid_template_columns: vec![RepeatedGridTrack::auto(3)],
                        row_gap,
                        ..default()
                    },
                ))
                .with_children(|container| {
                    for (button_component, button_info) in button_map {
                        let ListItem { text, color, image } = button_info;
                        let button = ButtonBuilder {
                            text,
                            color,
                            image,
                            ..default()
                        };
                        button.spawn(container).insert(button_component);
                    }
                });
        })
        .id()
}
