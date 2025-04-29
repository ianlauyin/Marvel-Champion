mod deck;
mod deck_validator;
mod decks_storage;
mod util;

pub use deck::Deck;
pub use deck_validator::DeckValidator;
pub use decks_storage::DecksStorageUtil;
pub use util::DeckUtil;

use bevy::app::{App, Plugin};

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(decks_storage::DecksStoragePlugin);
    }
}
