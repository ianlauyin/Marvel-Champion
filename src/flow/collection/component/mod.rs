mod card_list;
mod menu_button;
mod sub_menu;
mod sub_menu_button;

pub use menu_button::CollectionMenuButton;
pub use sub_menu::SubMenu;
pub use sub_menu_button::SubMenuButton;

use bevy::app::{App, Plugin};
pub struct CollectionComponentsPlugin;

impl Plugin for CollectionComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((sub_menu::SubMenuPlugin, card_list::CardListPlugin));
    }
}
