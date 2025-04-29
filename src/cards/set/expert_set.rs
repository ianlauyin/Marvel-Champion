use bevy::color::Color;

use crate::cards::data::expert_set;
use crate::component::Card;

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

    fn get_cards<'a>(&self) -> Vec<Card<'a>> {
        match *self {
            Self::Expert => expert_set::expert::get_cards(),
        }
    }

    fn get_thumbnail_key(&self) -> Option<String> {
        Some(format!("expert_set/{}", self.get_key()))
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
