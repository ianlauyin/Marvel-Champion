use bevy::platform::collections::{HashMap, HashSet};

use crate::{
    cards::{Aspect, SetTrait},
    component::Card,
};

pub struct DeckUtil;

impl DeckUtil {
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
