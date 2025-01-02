use bevy::prelude::*;

use super::ScrollingList;

#[derive(Default)]
pub struct TextListItem {
    pub text: String,
    pub color: Color,
}

pub struct TextListBuilder<B: Component + Clone>(pub Vec<(B, TextListItem)>);

impl<B: Component + Clone> TextListBuilder<B> {
    pub fn spawn(&self, mut commands: Commands) -> Entity {
        // Scrolling List Container
        commands
            .spawn(Node {
                width: Val::Percent(100.),
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
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(30.)),
                            row_gap: Val::Px(5.),
                            ..default()
                        },
                    ))
                    .with_children(|container| {
                        for (item_component, item_info) in self.0.iter() {
                            let TextListItem { text, color } = item_info;
                            container
                                .spawn((
                                    item_component.clone(),
                                    Node {
                                        padding: UiRect::all(Val::Px(10.)),
                                        border: UiRect::all(Val::Px(1.)),
                                        ..default()
                                    },
                                    BackgroundColor::from(*color),
                                    BorderRadius::all(Val::Px(5.)),
                                    BorderColor::from(Color::NONE),
                                    Interaction::default(),
                                ))
                                .with_child(Text::new(text.clone()));
                        }
                    });
            })
            .id()
    }
}
