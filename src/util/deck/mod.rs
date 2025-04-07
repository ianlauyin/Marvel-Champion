mod decks_storage;
mod util;

use std::collections::HashMap;

pub use decks_storage::DecksStorageUtil;
pub use util::DeckUtil;

use bevy::{
    log::warn,
    prelude::{App, Plugin},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Deck {
    id: String,
    name: String,
    card_id_map: HashMap<String, u8>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "New Deck".to_string(),
            card_id_map: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_card_ids(&self) -> Vec<String> {
        let mut card_ids = vec![];
        for (card_id, amount) in self.card_id_map.iter() {
            for _ in 0..*amount {
                card_ids.push(card_id.to_string());
            }
        }
        card_ids
    }

    pub fn push_card_id(&mut self, card_id: &str) {
        let count = self.card_id_map.entry(card_id.to_string()).or_insert(0);
        *count += 1;
    }

    pub fn remove_card_id(&mut self, card_id: &str) {
        if let Some(count) = self.card_id_map.get_mut(card_id) {
            *count -= 1;
            if *count == 0 {
                self.card_id_map.remove(card_id);
            }
        } else {
            warn!("Card id not found: {}", card_id);
        }
    }
}

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(decks_storage::DecksStoragePlugin);
    }
}
