use std::collections::HashSet;

use crate::features::cards::{Card, CardAspect};

pub struct CardUtils;

impl CardUtils {
    pub fn get_card_amount(deck_cards: &Vec<Card>) -> u8 {
        deck_cards
            .iter()
            .filter(|card| card.count_in_deck_amount())
            .count() as u8
    }

    pub fn get_deck_aspect(deck_cards: &Vec<Card>) -> Vec<CardAspect> {
        let mut aspects = HashSet::new();
        for card in deck_cards.iter() {
            if let Ok(aspect) = card.get_aspect() {
                aspects.insert(aspect);
            }
        }
        aspects.into_iter().collect()
    }
}
