mod loading_screen;

pub use loading_screen::LoadingScreen;

use bevy::prelude::{App, Plugin};

pub struct UiComponentPlugin;

impl Plugin for UiComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(loading_screen::LoadingScreenPlugin);
    }
}
