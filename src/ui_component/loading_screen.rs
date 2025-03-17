use bevy::prelude::*;

use crate::constant::WINDOW_RESOLUTION;

#[derive(Component)]
pub struct LoadingScreen;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_loading_screen);
    }
}

fn spawn_loading_screen(trigger: Trigger<OnAdd, LoadingScreen>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .insert((
            Node {
                width: Val::Px(WINDOW_RESOLUTION.x),
                height: Val::Px(WINDOW_RESOLUTION.y),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor::from(Color::BLACK.with_alpha(0.8)),
        ))
        .with_child(Text::new("Loading..."));
}
