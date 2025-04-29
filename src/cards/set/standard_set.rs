use bevy::color::Color;

use crate::cards::data::standard_set;
use crate::component::Card;

use super::set_trait::SetTrait;

#[derive(Clone)]
pub enum StandardSet {
    Standard,
}

impl StandardSet {
    pub fn get_all() -> Vec<Self> {
        vec![Self::Standard]
    }
}

impl SetTrait for StandardSet {
    fn get_boxed_all() -> Vec<Box<dyn SetTrait>> {
        Self::get_all()
            .into_iter()
            .map(|set| Box::new(set) as Box<dyn SetTrait>)
            .collect()
    }

    fn to_str(&self) -> &str {
        match *self {
            Self::Standard => "Standard",
        }
    }

    fn get_key(&self) -> &str {
        match *self {
            Self::Standard => "standard",
        }
    }

    fn get_cards<'a>(&self) -> Vec<Card<'a>> {
        match *self {
            Self::Standard => standard_set::standard::get_cards(),
        }
    }

    fn get_thumbnail_key(&self) -> Option<String> {
        Some(format!("standard_set/{}", self.get_key()))
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
