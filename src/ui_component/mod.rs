mod card;
mod card_detail;
mod container_header;
mod custom_button;
mod loading_screen;
mod main_container;
mod node_moving;
mod scrolling_list;

pub use card::Card;
pub use card_detail::CardDetailButton;
pub use container_header::{ContainerHeader, ContainerHeaderEvent};
pub use custom_button::CustomButton;
pub use loading_screen::LoadingScreen;
pub use main_container::MainContainer;
pub use node_moving::{NodeMove, NodeMoveRemoveEvent};
pub use scrolling_list::ScrollingList;

use bevy::prelude::{App, Plugin};

pub struct UiComponentPlugin;

impl Plugin for UiComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            container_header::ContainerHeaderPlugin,
            main_container::MainContainerPlugin,
            loading_screen::LoadingScreenPlugin,
            node_moving::NodeMovingPlugin,
            custom_button::MenuButtonPlugin,
            scrolling_list::ScrollingListPlugin,
            card::CardPlugin,
            card_detail::CardDetailPlugin,
        ));
    }
}
