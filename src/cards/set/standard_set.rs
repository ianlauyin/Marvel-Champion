use bevy::ecs::{entity::Entity, system::Commands};

use crate::cards::data::standard_set;
use crate::component::card::CardBasic;

#[derive(Clone)]
pub enum StandardSet {
    Standard,
}

impl StandardSet {
    pub fn get_all() -> Vec<Self> {
        vec![Self::Standard]
    }

    pub fn to_string(&self) -> String {
        let str = match *self {
            Self::Standard => "Standard",
        };
        str.to_string()
    }

    pub fn get_key(&self) -> String {
        match *self {
            Self::Standard => "standard".to_string(),
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic> {
        match *self {
            Self::Standard => standard_set::standard::get_infos(),
        }
    }

    pub fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
        match *self {
            Self::Standard => standard_set::standard::get_cards(),
        }
    }
}
