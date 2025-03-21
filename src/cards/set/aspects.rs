use bevy::ecs::{entity::Entity, system::Commands};

use super::super::data::aspect;
use crate::component::card::CardBasic;

#[derive(Clone)]
pub enum Aspect {
    Basic,
    Justice,
    Aggression,
    Protection,
    Leadership,
    Pool,
}

impl Aspect {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::Basic,
            Self::Justice,
            Self::Aggression,
            Self::Protection,
            Self::Leadership,
            Self::Pool,
        ]
    }

    pub fn get_all_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
        let mut cards = Vec::new();
        for aspect in Self::get_all() {
            for card in aspect.get_cards() {
                cards.push(card);
            }
        }
        cards
    }

    pub fn to_str(&self) -> &str {
        match *self {
            Self::Basic => "Basic",
            Self::Justice => "Justice",
            Self::Aggression => "Aggression",
            Self::Protection => "Protection",
            Self::Leadership => "Leadership",
            Self::Pool => "Pool",
        }
    }

    pub fn get_key(&self) -> &str {
        match *self {
            Self::Basic => "basic",
            Self::Justice => "justice",
            Self::Aggression => "aggression",
            Self::Protection => "protection",
            Self::Leadership => "leadership",
            Self::Pool => "pool",
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic<'static>> {
        match self {
            Self::Aggression => aspect::aggression::get_infos(),
            Self::Basic => aspect::basic::get_infos(),
            Self::Justice => aspect::justice::get_infos(),
            Self::Leadership => aspect::leadership::get_infos(),
            Self::Protection => aspect::protection::get_infos(),
            Self::Pool => aspect::pool::get_infos(),
        }
    }

    pub fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
        match self {
            Self::Aggression => aspect::aggression::get_cards(),
            Self::Basic => aspect::basic::get_cards(),
            Self::Justice => aspect::justice::get_cards(),
            Self::Leadership => aspect::leadership::get_cards(),
            Self::Protection => aspect::protection::get_cards(),
            Self::Pool => aspect::pool::get_cards(),
        }
    }
}
