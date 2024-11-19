use bevy::prelude::*;

use crate::constants::WINDOW_RESOLUTION;
use crate::systems::clean_up;
pub struct LoadingScreenPlugin<S: States> {
    pub loading_state: S,
}

impl<S: States> Plugin for LoadingScreenPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.loading_state.clone()), spawn_loading_screen)
            .add_systems(
                OnExit(self.loading_state.clone()),
                clean_up::<LoadingScreen>,
            );
    }
}

#[derive(Component)]
struct LoadingScreen;

fn spawn_loading_screen(mut commands: Commands) {
    commands
        .spawn((
            LoadingScreen,
            NodeBundle {
                style: Style {
                    width: Val::Px(WINDOW_RESOLUTION.x),
                    height: Val::Px(WINDOW_RESOLUTION.y),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK.with_alpha(0.8)),
                ..default()
            },
        ))
        .with_children(|background_node| {
            background_node.spawn(TextBundle::from_section("Loading...", TextStyle::default()));
        });
}
