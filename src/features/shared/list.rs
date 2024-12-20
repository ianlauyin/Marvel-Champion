use bevy::prelude::*;

use crate::features::shared::CustomButton;

use super::ScrollingList;

#[derive(Default)]
pub struct ListItem {
    pub text: String,
    pub color: Color,
    pub image: ImageNode,
}

pub struct ListBuilder<B: Component + Clone>(pub Vec<(B, ListItem)>);

impl<B: Component + Clone> ListBuilder<B> {
    pub fn spawn(&self, mut commands: Commands) -> Entity {
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
                let row_gap = Val::Px(match self.0.len() {
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
                        for (item_component, item_info) in self.0.iter() {
                            let ListItem { text, color, image } = item_info;
                            container.spawn((
                                item_component.clone(),
                                CustomButton {
                                    text: text.to_string(),
                                    color: *color,
                                    image: image.clone(),
                                    ..default()
                                },
                            ));
                        }
                    });
            })
            .id()
    }
}
