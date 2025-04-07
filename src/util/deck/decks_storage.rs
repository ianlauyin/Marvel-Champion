use crate::cards::{IdentitySet, SetTrait};
use bevy::prelude::*;
use bevy_pkv::PkvStore;

use super::Deck;
use crate::resource::AspectCardDatas;

pub struct DecksStorageUtil<'a> {
    identity: IdentitySet,
    pkv: ResMut<'a, PkvStore>,
}

impl<'a> DecksStorageUtil<'a> {
    pub fn init(identity: &IdentitySet, pkv: ResMut<'a, PkvStore>) -> Self {
        Self {
            identity: identity.clone(),
            pkv,
        }
    }

    pub fn get_decks(&mut self) -> Vec<Deck> {
        if let Ok(decks) = self.pkv.get::<Vec<Deck>>(self.identity.get_key()) {
            decks
        } else {
            self.init_identity();
            vec![]
        }
    }

    pub fn save_deck(
        &mut self,
        deck: Deck,
        aspect_cards_data: Res<AspectCardDatas>,
    ) -> Result<(), String> {
        let aspect_cards = aspect_cards_data.get_batch_info_by_id(&deck.get_card_ids());
        let mut validator = self.identity.get_deck_validator();
        if let Err(message) = validator.validate(&aspect_cards) {
            return Err(message);
        }

        let mut decks = self.get_decks();
        if let Some(index) = decks
            .iter()
            .position(|existing_deck| existing_deck.id == deck.id)
        {
            // Existing Deck
            decks[index] = deck;
        } else {
            // New Deck
            decks.push(deck);
        }
        self.pkv
            .set(self.identity.get_key(), &decks)
            .map_err(|e| format!("Failed to add deck: {}", e))?;
        Ok(())
    }

    pub fn remove_deck(&mut self, id: &str) {
        let mut decks = self.get_decks();
        let Some(index) = decks.iter().position(|deck| deck.id == id) else {
            warn!("Deck not found id: {}", id);
            return;
        };
        decks.remove(index);
        self.pkv
            .set(self.identity.get_key(), &decks)
            .expect("Failed to remove deck.");
    }

    fn init_identity(&mut self) {
        self.pkv
            .set(self.identity.get_key(), &Vec::<Deck>::new())
            .expect("Failed to init decks.");
    }
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
