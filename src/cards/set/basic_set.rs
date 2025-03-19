use bevy::ecs::system::Commands;

use crate::component::card::CardBasic;

#[derive(Clone)]
pub enum BasicSet {
    Standard,
    Expert,
}

impl BasicSet {
    pub fn get_all() -> Vec<Self> {
        vec![Self::Standard, Self::Expert]
    }

    pub fn to_string(&self) -> String {
        let str = match *self {
            Self::Standard => "Standard",
            Self::Expert => "Expert",
        };
        str.to_string()
    }

    pub fn get_key(&self) -> String {
        match *self {
            Self::Standard => "standard".to_string(),
            Self::Expert => "expert".to_string(),
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic> {
        todo!()
    }

    pub fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands))> {
        todo!()
    }
}
