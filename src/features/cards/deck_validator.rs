use std::collections::HashMap;

use crate::utils::CardUtils;

use super::{Card, CardAspect, Identity};

pub struct DeckValidator {
    identity: Identity,
    build_validators: Vec<fn(&Vec<Card>) -> Result<(), String>>,
    play_validators: Vec<fn(&Vec<Card>) -> Result<(), String>>,
}

impl DeckValidator {
    pub fn default(identity: Identity) -> Self {
        Self {
            identity,
            build_validators: vec![aspects_rules_validator, cards_amount_limit_validator],
            play_validators: vec![deck_cards_amount_validator],
        }
    }

    pub fn validate_build(&mut self, deck_cards: &Vec<Card>) -> Result<(), String> {
        if let Err(message) = self.validate_identity_cards(&deck_cards) {
            return Err(message);
        }
        self.build_validators
            .iter_mut()
            .try_for_each(|validator| validator(&deck_cards))
    }

    pub fn validate(&mut self, deck_cards: &Vec<Card>) -> Result<(), String> {
        [self.play_validators.clone(), self.build_validators.clone()]
            .concat()
            .iter_mut()
            .try_for_each(|validator| validator(&deck_cards))
    }

    fn validate_identity_cards(&self, deck_cards: &Vec<Card>) -> Result<(), String> {
        let mut hash_map: HashMap<String, u8> = HashMap::new();

        for comparing_card in [
            self.identity.get_identity_cards(),
            self.identity.get_player_cards(),
        ]
        .concat()
        {
            if let Some(amount) = hash_map.get_mut(&comparing_card.get_id()) {
                *amount += 1;
            } else {
                hash_map.insert(comparing_card.get_id(), 1);
            };
        }

        for card in deck_cards {
            if let Ok(CardAspect::IdentitySpecific(identity)) = card.get_aspect() {
                if identity != self.identity {
                    return Err("Cannot include identity cards that is your identity.".to_string());
                }
                let card_id = card.get_id();
                if let Some(amount) = hash_map.get_mut(&card_id) {
                    *amount -= 1;
                    if *amount == 0 {
                        hash_map.remove(&card_id);
                    }
                } else {
                    return Err("Cannot include more than original identity cards".to_string());
                }
            }
        }
        if !hash_map.is_empty() {
            return Err("Cannot exclude original identity cards".to_string());
        }
        Ok(())
    }
}

pub fn aspects_rules_validator(deck_cards: &Vec<Card>) -> Result<(), String> {
    let card_aspects = CardUtils::get_deck_aspect(deck_cards);
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

fn deck_cards_amount_validator(deck_cards: &Vec<Card>) -> Result<(), String> {
    let card_amounts = CardUtils::get_card_amount(deck_cards);
    match card_amounts {
        0..=39 => Err("Cannot have cards less than 40".to_string()),
        40..=50 => Ok(()),
        51.. => Err("Cannot have cards more than 50".to_string()),
    }
}

fn cards_amount_limit_validator(deck_cards: &Vec<Card>) -> Result<(), String> {
    let mut hash_map: HashMap<String, u8> = HashMap::new();
    for card in deck_cards.iter() {
        if let Ok(card_amount_max) = card.get_card_amount_max() {
            if let Some(card_amount_limit) = hash_map.get_mut(&card.get_id()) {
                if *card_amount_limit == 0 {
                    let error_message = format!(
                        "{} can only have {} cards",
                        card.get_name(),
                        card_amount_max
                    );
                    return Err(error_message);
                }
                *card_amount_limit -= 1;
            } else {
                hash_map.insert(card.get_id(), card_amount_max - 1);
            };
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::features::cards::{
        deck_validator::{
            aspects_rules_validator, cards_amount_limit_validator, deck_cards_amount_validator,
            DeckValidator,
        },
        Card, CardDatas, Identity,
    };

    fn get_valid_deck() -> Vec<Card> {
        [
            Identity::CoreSpiderMan.get_player_cards(),
            CardDatas::get_basic_cards()[..11].to_vec(),
            CardDatas::get_basic_cards()[2..5].to_vec(),
            CardDatas::get_basic_cards()[2..5].to_vec(),
            CardDatas::get_basic_cards()[11..].to_vec(),
            CardDatas::get_basic_cards()[11..].to_vec(),
            CardDatas::get_aggression_cards()[0..8].to_vec(),
        ]
        .concat()
    }

    #[test]
    fn test_validate_identity_cards() {
        let valid_deck = get_valid_deck();
        let validator = DeckValidator::default(Identity::CoreSpiderMan);
        assert_eq!(validator.validate_identity_cards(&valid_deck), Ok(()));
    }

    #[test]
    fn test_aspects_rules_validator() {
        let valid_deck = get_valid_deck();
        assert!(aspects_rules_validator(&valid_deck).is_ok());

        let invalid_deck = [valid_deck, CardDatas::get_protection_cards()[0..1].to_vec()].concat();
        assert_eq!(
            aspects_rules_validator(&invalid_deck),
            Err("Cannot have more than one card aspect".to_string())
        );
    }

    #[test]
    fn test_deck_cards_amount_validator() {
        let valid_deck = get_valid_deck();
        assert!(deck_cards_amount_validator(&valid_deck).is_ok());

        let undersized_deck = valid_deck[0..30].to_vec();
        assert_eq!(
            deck_cards_amount_validator(&undersized_deck),
            Err("Cannot have cards less than 40".to_string())
        );

        let oversized_deck = [valid_deck.clone(), valid_deck].concat();
        assert_eq!(
            deck_cards_amount_validator(&oversized_deck),
            Err("Cannot have cards more than 50".to_string())
        )
    }

    #[test]
    fn test_cards_amount_limit_validator() {
        let valid_deck = get_valid_deck();
        assert!(cards_amount_limit_validator(&valid_deck).is_ok());

        let test_card = CardDatas::get_basic_cards()[0].clone();
        let invalid_deck_card_amount = test_card.get_card_amount_max().unwrap();
        let invalid_deck: Vec<Card> = (0..=invalid_deck_card_amount)
            .map(|_| test_card.clone())
            .collect();

        assert_eq!(
            cards_amount_limit_validator(&invalid_deck),
            Err(format!(
                "{} can only have {} cards",
                test_card.get_name(),
                invalid_deck_card_amount
            ))
        )
    }
}
