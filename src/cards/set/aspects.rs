use bevy::color::Color;

use super::{super::data::aspect, set_trait::SetTrait};
use crate::component::Card;

#[derive(Clone, PartialEq, Eq, Hash)]
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

    pub fn get_all_cards<'a>() -> Vec<Card<'a>> {
        let mut cards = Vec::new();
        for aspect in Self::get_all() {
            for card in aspect.get_cards() {
                cards.push(card);
            }
        }
        cards
    }
}

impl SetTrait for Aspect {
    fn get_boxed_all() -> Vec<Box<dyn SetTrait>> {
        Self::get_all()
            .into_iter()
            .map(|set| Box::new(set) as Box<dyn SetTrait>)
            .collect()
    }

    fn to_str(&self) -> &str {
        match *self {
            Self::Basic => "Basic",
            Self::Justice => "Justice",
            Self::Aggression => "Aggression",
            Self::Protection => "Protection",
            Self::Leadership => "Leadership",
            Self::Pool => "Pool",
        }
    }

    fn get_key(&self) -> &str {
        match *self {
            Self::Basic => "basic",
            Self::Justice => "justice",
            Self::Aggression => "aggression",
            Self::Protection => "protection",
            Self::Leadership => "leadership",
            Self::Pool => "pool",
        }
    }

    fn get_cards<'a>(&self) -> Vec<Card<'a>> {
        match self {
            Self::Aggression => aspect::aggression::get_cards(),
            Self::Basic => aspect::basic::get_cards(),
            Self::Justice => aspect::justice::get_cards(),
            Self::Leadership => aspect::leadership::get_cards(),
            Self::Protection => aspect::protection::get_cards(),
            Self::Pool => aspect::pool::get_cards(),
        }
    }

    fn get_thumbnail_key(&self) -> Option<String> {
        None
    }

    fn get_color(&self) -> Option<Color> {
        match *self {
            Self::Basic => None,
            Self::Justice => Some(Color::srgb(0.871, 0.941, 0.086)),
            Self::Aggression => Some(Color::srgb(0.741, 0.192, 0.192)),
            Self::Protection => Some(Color::srgb(0.075, 0.773, 0.075)),
            Self::Leadership => Some(Color::srgb(0.125, 0.769, 0.882)),
            Self::Pool => Some(Color::srgb(0.89, 0.149, 0.816)),
        }
    }
}
