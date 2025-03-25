use bevy::ecs::{entity::Entity, system::Commands};

use crate::cards::data::expert_set;
use crate::component::card::CardBasic;

use super::set_trait::SetTrait;

#[derive(Clone)]
pub enum ExpertSet {
    Expert,
}

impl ExpertSet {
    pub fn get_all() -> Vec<Self> {
        vec![Self::Expert]
    }
}

impl SetTrait for ExpertSet {
    fn get_boxed_all() -> Vec<Box<dyn SetTrait>> {
        Self::get_all()
            .into_iter()
            .map(|set| Box::new(set) as Box<dyn SetTrait>)
            .collect()
    }

    fn to_str(&self) -> &str {
        match *self {
            Self::Expert => "Expert",
        }
    }

    fn get_key(&self) -> &str {
        match *self {
            Self::Expert => "expert",
        }
    }

    fn get_card_infos(&self) -> Vec<CardBasic> {
        match *self {
            Self::Expert => expert_set::expert::get_infos(),
        }
    }

    fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
        match *self {
            Self::Expert => expert_set::expert::get_cards(),
        }
    }

    fn get_thumbnail_key(&self) -> Option<String> {
        Some(format!("expert_set/{}", self.get_key()))
    }
}
