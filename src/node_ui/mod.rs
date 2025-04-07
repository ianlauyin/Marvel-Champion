mod card;
mod card_detail;
mod card_detail_button;
mod container_header;
mod custom_button;
mod loading_screen;
mod main_container;
mod mouse_control_util;
mod scrolling_list;

pub use card::{Card, CARD_SIZE_SMALL};
pub use card_detail::CardDetail;
pub use card_detail_button::CardDetailButton;
pub use container_header::{ContainerHeader, ContainerHeaderEvent};
pub use custom_button::CustomButton;
pub use loading_screen::LoadingScreen;
pub use main_container::MainContainer;
pub use mouse_control_util::{MouseControl, MouseControlEvent};
pub use scrolling_list::ScrollingList;

use bevy::prelude::{App, Plugin};

pub struct NodeUiPlugin;

impl Plugin for NodeUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            container_header::ContainerHeaderPlugin,
            main_container::MainContainerPlugin,
            loading_screen::LoadingScreenPlugin,
            custom_button::MenuButtonPlugin,
            scrolling_list::ScrollingListPlugin,
            card::CardPlugin,
            card_detail_button::CardDetailButtonPlugin,
            card_detail::CardDetailPlugin,
            mouse_control_util::MouseControlUtilPlugin,
        ));
    }
}
