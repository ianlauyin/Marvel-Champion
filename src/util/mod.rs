mod deck;
mod devtool;
mod system_util;
mod ui_util;

pub use deck::{Deck, DeckUtil, DecksStorageUtil};
pub use system_util::SystemUtil;
pub use ui_util::UiUtils;

pub struct UtilPlugin;

impl bevy::app::Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((devtool::DevtoolPlugin, deck::DeckPlugin));
    }
}
