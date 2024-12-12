use crate::utils::get_deck_aspect;

use super::{Card, CardAspect};

pub struct DeckValidator {
    validators: Vec<fn(&Vec<Card>) -> Result<(), String>>,
}

impl DeckValidator {
    pub fn validate(&mut self, deck_cards: Vec<Card>) -> Result<(), String> {
        self.validators
            .iter_mut()
            .try_for_each(|validator| validator(&deck_cards))
    }

    pub fn default() -> Self {
        Self {
            validators: vec![aspects_rules_validator],
        }
    }
}

fn aspects_rules_validator(deck_cards: &Vec<Card>) -> Result<(), String> {
    let card_aspects = get_deck_aspect(deck_cards);
    let filterd_card_aspects: Vec<&CardAspect> = card_aspects
        .iter()
        .filter(|aspect| match aspect {
            CardAspect::IdentitySpecific(_) | CardAspect::Basic => false,
            _ => true,
        })
        .collect();
    if filterd_card_aspects.len() > 1 {
        return Err("Cannot have more than one card aspect".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::features::cards::{deck_validator::aspects_rules_validator, CardDatas, Identity};

    #[test]
    fn test_aspects_rules_validator() {
        let base_deck_cards = [
            Identity::CoreSpiderMan.get_cards(),
            CardDatas::get_basic_cards()[0..1].to_vec(),
        ]
        .concat();

        let valid_deck = [
            base_deck_cards.clone(),
            CardDatas::get_aggression_cards()[0..1].to_vec(),
        ]
        .concat();

        assert_eq!(aspects_rules_validator(&valid_deck), Ok(()));

        let invalid_deck = [
            base_deck_cards.clone(),
            CardDatas::get_aggression_cards()[0..1].to_vec(),
            CardDatas::get_protection_cards()[0..1].to_vec(),
        ]
        .concat();

        assert_eq!(
            aspects_rules_validator(&invalid_deck),
            Err("Cannot have more than one card aspect".to_string())
        );
    }
}
