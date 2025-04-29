use bevy::{prelude::*, ui::FocusPolicy};

use crate::constant::WINDOW_RESOLUTION;

#[derive(Component)]
pub struct LoadingScreen;

impl LoadingScreen {
    pub fn new() -> impl Bundle {
        (
            Self,
            Node {
                width: Val::Px(WINDOW_RESOLUTION.x),
                height: Val::Px(WINDOW_RESOLUTION.y),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor::from(Color::BLACK.with_alpha(0.8)),
            FocusPolicy::Block,
            children![Text::new("Loading...")],
        )
    }
}
