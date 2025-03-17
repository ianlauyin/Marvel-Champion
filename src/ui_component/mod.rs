mod loading_screen;
mod main_container;
mod menu_button;
mod node_moving;

pub use loading_screen::LoadingScreen;
pub use main_container::MainContainer;
pub use menu_button::MenuButton;
pub use node_moving::{NodeMove, NodeMoveRemoveEvent};

use bevy::prelude::{App, Plugin};

pub struct UiComponentPlugin;

impl Plugin for UiComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            main_container::MainContainerPlugin,
            loading_screen::LoadingScreenPlugin,
            node_moving::NodeMovingPlugin,
            menu_button::MenuButtonPlugin,
        ));
    }
}
