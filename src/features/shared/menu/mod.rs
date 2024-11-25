mod card_list;
mod list;
mod menu_frame;
mod scrolling_list;

use bevy::prelude::{App, Plugin};
pub use card_list::spawn_card_list;
pub use list::spawn_list;
pub use menu_frame::{DisplayMethod, ListItem, MenuBuilder};
pub use scrolling_list::ScrollingList;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(scrolling_list::ScrollingListPlugin);
    }
}
