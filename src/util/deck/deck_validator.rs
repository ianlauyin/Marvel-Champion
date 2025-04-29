use std::collections::HashMap;

use crate::component::Card;

use super::DeckUtil;

pub struct DeckValidator {
    validators: Vec<fn(&Vec<Card<'static>>) -> Result<(), String>>,
}

impl DeckValidator {
    pub fn default() -> Self {
        Self {
            validators: vec![
                aspects_rules_validator,
                cards_amount_limit_validator,
                deck_cards_amount_validator,
            ],
        }
    }

    pub fn validate(&mut self, deck_cards: &Vec<Card<'static>>) -> Result<(), String> {
        self.validators
            .iter_mut()
            .try_for_each(|validator| validator(&deck_cards))
    }
}

pub fn aspects_rules_validator(deck_cards: &Vec<Card<'static>>) -> Result<(), String> {
    let card_aspects = DeckUtil::get_current_aspects(deck_cards);
    if card_aspects.len() > 1 {
        return Err("Cannot have more than one card aspect".to_string());
    }
    Ok(())
}

fn deck_cards_amount_validator(deck_cards: &Vec<Card<'static>>) -> Result<(), String> {
    let card_amounts = deck_cards.len() + 15;
    match card_amounts {
        0..=39 => Err("Cannot have cards less than 40".to_string()),
        40..=50 => Ok(()),
        51.. => Err("Cannot have cards more than 50".to_string()),
    }
}

fn cards_amount_limit_validator(deck_cards: &Vec<Card<'static>>) -> Result<(), String> {
    let mut hash_map: HashMap<String, u8> = HashMap::new();
    for card in deck_cards.iter() {
        if let Some(card_amount_limit) = hash_map.get_mut(&card.id.to_string()) {
            if *card_amount_limit == 0 {
                let error_message =
                    format!("{} can only have {} cards", card.name, card.card_amount_max);
                return Err(error_message);
            }
            *card_amount_limit -= 1;
        } else {
            hash_map.insert(card.id.to_string(), card.card_amount_max - 1);
        };
    }
    Ok(())
}
