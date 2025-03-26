mod card_list;
mod menu_button;
mod sub_menu;

pub use menu_button::CollectionMenuButton;
pub use sub_menu::SubMenu;

use bevy::app::{App, Plugin};
pub struct CollectionComponentsPlugin;

impl Plugin for CollectionComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sub_menu::SubMenuPlugin);
    }
}
