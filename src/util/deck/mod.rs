mod decks_storage;

pub use decks_storage::DecksStorageUtil;

use bevy::prelude::{App, Plugin};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Deck {
    id: String,
    name: String,
    card_ids: Vec<String>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            card_ids: vec![],
        }
    }
}

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(decks_storage::DecksStoragePlugin);
    }
}
