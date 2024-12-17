use bevy::prelude::*;

use crate::features::shared::CustomButton;

use super::{ListItem, ScrollingList};

pub struct CardListBuilder<B: Bundle + Clone> {
    pub button_map: Vec<(B, ListItem)>,
    pub card_size: (Val, Val),
    pub height: Val,
    pub columns: u16,
}

impl<B: Bundle + Clone> CardListBuilder<B> {
    pub fn spawn(&self, mut commands: Commands) -> Entity {
        commands
            .spawn(Node {
                height: self.height,
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
                            grid_template_columns: vec![RepeatedGridTrack::auto(self.columns)],
                            row_gap: Val::Px(10.),
                            column_gap: Val::Px(10.),
                            ..default()
                        },
                    ))
                    .with_children(|list_div| {
                        // Items
                        for (button_component, button_info) in self.button_map.clone() {
                            list_div
                                .spawn(Node {
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                })
                                .with_children(|card_type_node| {
                                    let ListItem { color, image, .. } = button_info;
                                    card_type_node.spawn((
                                        button_component,
                                        CustomButton {
                                            color,
                                            image,
                                            size: self.card_size,
                                            with_border: false,
                                            ..default()
                                        },
                                    ));
                                });
                        }
                    });
            })
            .id()
    }
}
