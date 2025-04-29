use bevy::{prelude::*, ui::FocusPolicy};

use crate::constant::WINDOW_RESOLUTION;

#[derive(Component)]
pub struct LoadingScreen;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_loading_screen_added);
    }
}

fn on_loading_screen_added(trigger: Trigger<OnAdd, LoadingScreen>, mut commands: Commands) {
    commands.entity(trigger.target()).insert((
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
    ));
}
