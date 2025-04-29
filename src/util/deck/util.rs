use bevy::utils::{HashMap, HashSet};

use crate::{
    cards::{Aspect, IdentitySet, SetTrait},
    component::Card,
};

pub struct DeckUtil;

impl DeckUtil {
    pub fn get_cards_pair(identity_set: IdentitySet) -> (Vec<Card<'static>>, Vec<Card<'static>>) {
        let mut identity_cards = vec![];
        let mut other_cards = vec![];
        let identity_card_ids = identity_set.get_identity_card_ids();
        let non_player_card_ids = identity_set.get_non_player_cards_ids();
        for card in identity_set.get_cards().iter() {
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

    pub fn get_current_aspects(aspect_cards: &Vec<Card<'static>>) -> Vec<Aspect> {
        let mut aspects = HashSet::new();
        for card in aspect_cards {
            if let Some(aspect) = card.belongs.get_aspect() {
                if aspect != Aspect::Basic {
                    aspects.insert(aspect);
                }
            }
        }
        aspects.into_iter().collect()
    }

    pub fn get_available_cards(
        deck_card_ids: &Vec<String>,
        current_aspect: &Vec<Aspect>,
    ) -> Vec<Card<'static>> {
        let mut available_cards = vec![];
        let mut deck_card_map: HashMap<String, u8> = HashMap::new();
        for deck_card_id in deck_card_ids {
            let count = deck_card_map.entry(deck_card_id.clone()).or_insert(0);
            *count += 1;
        }

        let aspect_cards: Vec<Card<'static>> = {
            if current_aspect.is_empty() {
                Aspect::get_all()
                    .iter()
                    .map(|aspect| aspect.get_cards())
                    .flatten()
                    .collect()
            } else {
                current_aspect
                    .iter()
                    .flat_map(|aspect| aspect.get_cards())
                    .chain(Aspect::Basic.get_cards())
                    .collect()
            }
        };

        for card in aspect_cards {
            if let Some(count) = deck_card_map.get_mut(card.id) {
                if *count < card.card_amount_max {
                    available_cards.push(card);
                }
            } else {
                available_cards.push(card);
            }
        }

        available_cards
    }
}
