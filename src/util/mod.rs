mod deck;
mod devtool;
mod mouse_control_util;
mod system_util;
mod ui_util;

pub use deck::{Deck, DeckUtil, DecksStorageUtil};
pub use mouse_control_util::{MouseControl, MouseControlEvent};
pub use system_util::SystemUtil;
pub use ui_util::UiUtils;

pub struct UtilPlugin;

impl bevy::app::Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            mouse_control_util::MouseControlUtilPlugin,
            devtool::DevtoolPlugin,
            deck::DeckPlugin,
        ));
    }
}
