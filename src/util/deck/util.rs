use bevy::utils::HashMap;

use crate::{
    cards::{Aspect, IdentitySet, SetTrait},
    component::card::CardBasic,
};

pub struct DeckUtil;

impl DeckUtil {
    pub fn get_cards_pair(
        identity_set: IdentitySet,
    ) -> (Vec<CardBasic<'static>>, Vec<CardBasic<'static>>) {
        let mut identity_cards = vec![];
        let mut other_cards = vec![];
        let identity_card_ids = identity_set.get_identity_card_ids();
        let non_player_card_ids = identity_set.get_non_player_cards_ids();
        for card in identity_set.get_card_infos().iter() {
            if identity_card_ids.contains(&card.id) {
                identity_cards.push(card.clone());
            } else if !non_player_card_ids.contains(&card.id) {
                for _ in 0..card.card_amount_max {
                    other_cards.push(card.clone());
                }
            }
        }
        (identity_cards, other_cards)
    }

    pub fn get_current_aspect(aspect_cards: &Vec<CardBasic<'static>>) -> Option<Aspect> {
        for card in aspect_cards {
            if let Some(aspect) = card.belongs.get_aspect() {
                if aspect != Aspect::Basic {
                    return Some(aspect);
                }
            }
        }
        None
    }

    pub fn get_available_cards(deck_card_ids: &Vec<String>) -> Vec<CardBasic<'static>> {
        let mut available_cards = vec![];
        let mut deck_card_map: HashMap<String, u8> = HashMap::new();
        for deck_card in deck_card_ids {
            let count = deck_card_map.entry(deck_card.clone()).or_insert(0);
            *count += 1;
        }

        for aspect in Aspect::get_all() {
            for card in aspect.get_card_infos() {
                if let Some(count) = deck_card_map.get(card.id) {
                    if *count < card.card_amount_max {
                        available_cards.push(card);
                    }
                } else {
                    available_cards.push(card);
                }
            }
        }

        available_cards
    }
}
