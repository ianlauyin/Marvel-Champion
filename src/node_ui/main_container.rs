use bevy::{prelude::*, ui::FocusPolicy};

use crate::util::UiUtils;

#[derive(Component)]
pub struct MainContainer;

impl MainContainer {
    pub fn default(z_index_q: &Query<&ZIndex>) -> impl Bundle {
        Self::bundle(JustifyContent::Start, z_index_q)
    }

    pub fn space_around(z_index_q: &Query<&ZIndex>) -> impl Bundle {
        Self::bundle(JustifyContent::SpaceAround, z_index_q)
    }

    fn bundle(justify_content: JustifyContent, z_index_q: &Query<&ZIndex>) -> impl Bundle {
        (
            Node {
                width: Val::Percent(90.),
                height: Val::Percent(90.),
                padding: UiRect::all(Val::Px(10.)),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                display: Display::Flex,
                column_gap: Val::Px(20.),
                flex_direction: FlexDirection::Column,
                justify_content: justify_content,
                align_items: AlignItems::Center,
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor::from(Color::BLACK.with_alpha(0.9999)),
            UiUtils::get_largest_z_index(&z_index_q),
            FocusPolicy::Block,
        )
    }
}
