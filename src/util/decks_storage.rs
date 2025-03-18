use bevy::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

use crate::features::cards::Identity;

pub struct DecksStorage<'a> {
    identity: Identity,
    pkv: ResMut<'a, PkvStore>,
}

impl<'a> DecksStorage<'a> {
    pub fn new(identity: &Identity, pkv: ResMut<'a, PkvStore>) -> Self {
        Self {
            identity: identity.clone(),
            pkv,
        }
    }

    pub fn get_decks(&mut self) -> Vec<StorageDeck> {
        if let Ok(decks) = self.pkv.get::<Vec<StorageDeck>>(self.identity.get_key()) {
            decks
        } else {
            self.init_identity();
            vec![]
        }
    }

    pub fn save_deck(&mut self, deck: StorageDeck, index: usize) {
        let mut decks = self.get_decks();
        decks[index] = deck;
        self.pkv
            .set(self.identity.get_key(), &decks)
            .expect("Failed to add deck.");
    }

    pub fn add_deck(&mut self, deck: StorageDeck) {
        let mut decks = self.get_decks();
        decks.push(deck);
        self.pkv
            .set(self.identity.get_key(), &decks)
            .expect("Failed to add deck.");
    }

    pub fn remove_deck(&mut self, index: usize) {
        let mut decks = self.get_decks();
        decks.remove(index);
        self.pkv
            .set(self.identity.get_key(), &decks)
            .expect("Failed to add deck.");
    }

    fn init_identity(&mut self) {
        self.pkv
            .set(self.identity.get_key(), &Vec::<StorageDeck>::new())
            .expect("Failed to init decks.");
    }
}

#[derive(Resource, Serialize, Deserialize)]
struct Decks(Vec<StorageDeck>);

#[derive(Serialize, Deserialize, Clone)]
pub struct StorageDeck {
    pub name: String,
    pub card_ids: Vec<String>,
}

pub struct DecksStoragePlugin;

impl Plugin for DecksStoragePlugin {
    fn build(&self, app: &mut App) {
        let (name, storage) = if cfg!(debug_assertions) {
            ("IanLau_Debug", "MarvelChampion_Debug") // Debug mode strings
        } else {
            ("IanLau", "MarvelChampion") // Release mode strings
        };
        app.insert_resource(PkvStore::new(name, storage));
    }
}
