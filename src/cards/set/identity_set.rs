use bevy::color::Color;

use crate::cards::data::identity_set;
use crate::component::Card;
use crate::util::DeckValidator;

use super::set_trait::SetTrait;

#[derive(Clone, PartialEq)]
pub enum IdentitySet {
    CoreSpiderMan,
    CoreCaptainMarvel,
    CoreSheHulk,
    CoreIronMan,
    CoreBlackPanther,
}

impl IdentitySet {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::CoreSpiderMan,
            Self::CoreCaptainMarvel,
            Self::CoreSheHulk,
            Self::CoreIronMan,
            Self::CoreBlackPanther,
        ]
    }

    pub fn get_identity_cards<'a>(&self) -> Vec<Card<'a>> {
        match *self {
            Self::CoreSpiderMan => identity_set::core_spider_man::get_identity_cards(),
            Self::CoreCaptainMarvel => identity_set::core_captain_marvel::get_identity_cards(),
            Self::CoreSheHulk => identity_set::core_she_hulk::get_identity_cards(),
            Self::CoreIronMan => identity_set::core_iron_man::get_identity_cards(),
            Self::CoreBlackPanther => identity_set::core_black_panther::get_identity_cards(),
        }
    }

    pub fn get_deck_cards<'a>(&self) -> Vec<Card<'a>> {
        let original_cards = match *self {
            Self::CoreSpiderMan => identity_set::core_spider_man::get_deck_cards(),
            Self::CoreCaptainMarvel => identity_set::core_captain_marvel::get_deck_cards(),
            Self::CoreSheHulk => identity_set::core_she_hulk::get_deck_cards(),
            Self::CoreIronMan => identity_set::core_iron_man::get_deck_cards(),
            Self::CoreBlackPanther => identity_set::core_black_panther::get_deck_cards(),
        };
        self.multiply_cards(&original_cards)
    }

    pub fn get_obligation_card<'a>(&self) -> Vec<Card<'a>> {
        let original_card = match *self {
            Self::CoreSpiderMan => identity_set::core_spider_man::get_obligation_card(),
            Self::CoreCaptainMarvel => identity_set::core_captain_marvel::get_obligation_card(),
            Self::CoreSheHulk => identity_set::core_she_hulk::get_obligation_card(),
            Self::CoreIronMan => identity_set::core_iron_man::get_obligation_card(),
            Self::CoreBlackPanther => identity_set::core_black_panther::get_obligation_card(),
        };
        self.multiply_cards(&vec![original_card])
    }

    pub fn get_out_of_play_cards<'a>(&self) -> Vec<Card<'a>> {
        let original_cards = match *self {
            Self::CoreSpiderMan => identity_set::core_spider_man::get_out_of_play_cards(),
            Self::CoreCaptainMarvel => identity_set::core_captain_marvel::get_out_of_play_cards(),
            Self::CoreSheHulk => identity_set::core_she_hulk::get_out_of_play_cards(),
            Self::CoreIronMan => identity_set::core_iron_man::get_out_of_play_cards(),
            Self::CoreBlackPanther => identity_set::core_black_panther::get_out_of_play_cards(),
        };
        self.multiply_cards(&original_cards)
    }

    pub fn get_deck_validator(&self) -> DeckValidator {
        match *self {
            _ => DeckValidator::default(),
        }
    }

    fn multiply_cards<'a>(&self, cards: &Vec<Card<'a>>) -> Vec<Card<'a>> {
        let mut multiplied_cards = vec![];
        for card in cards {
            for _ in 0..card.card_amount_max {
                multiplied_cards.push(card.clone());
            }
        }
        multiplied_cards
    }
}

impl SetTrait for IdentitySet {
    fn get_boxed_all() -> Vec<Box<dyn SetTrait>> {
        Self::get_all()
            .into_iter()
            .map(|set| Box::new(set) as Box<dyn SetTrait>)
            .collect()
    }

    fn to_str(&self) -> &str {
        match *self {
            Self::CoreSpiderMan => "Core - Spider Man",
            Self::CoreCaptainMarvel => "Core - Captain Marvel",
            Self::CoreSheHulk => "Core - She Hulk",
            Self::CoreIronMan => "Core - Iron Man",
            Self::CoreBlackPanther => "Core - Black Panther",
        }
    }

    fn get_key(&self) -> &str {
        match *self {
            Self::CoreSpiderMan => "core_spider_man",
            Self::CoreCaptainMarvel => "core_captain_marvel",
            Self::CoreSheHulk => "core_she_hulk",
            Self::CoreIronMan => "core_iron_man",
            Self::CoreBlackPanther => "core_black_panther",
        }
    }

    fn get_cards<'a>(&self) -> Vec<Card<'a>> {
        self.get_identity_cards()
            .into_iter()
            .chain(self.get_deck_cards())
            .chain(self.get_out_of_play_cards())
            .chain(self.get_obligation_card())
            .collect()
    }

    fn get_thumbnail_key(&self) -> Option<String> {
        Some(format!("identity_set/{}", self.get_key()))
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
