use bevy::ecs::world::World;

use crate::component::card::CardBasic;

pub enum BasicSet {
    Standard,
    Expert,
}

impl BasicSet {
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

    pub fn get_cards(&self) -> Vec<(CardBasic, fn(&mut World))> {
        todo!()
    }
}
