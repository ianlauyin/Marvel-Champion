use bevy::ecs::{entity::Entity, system::Commands};

use crate::cards::data::expert_set;
use crate::component::card::CardBasic;

#[derive(Clone)]
pub enum ExpertSet {
    Expert,
}

impl ExpertSet {
    pub fn get_all() -> Vec<Self> {
        vec![Self::Expert]
    }

    pub fn to_string(&self) -> String {
        let str = match *self {
            Self::Expert => "Expert",
        };
        str.to_string()
    }

    pub fn get_key(&self) -> String {
        match *self {
            Self::Expert => "expert".to_string(),
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic> {
        match *self {
            Self::Expert => expert_set::expert::get_infos(),
        }
    }

    pub fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
        match *self {
            Self::Expert => expert_set::expert::get_cards(),
        }
    }
}
