use bevy::ecs::world::World;

use crate::component::card::CardBasic;

pub enum IdentitySet {
    CoreSpiderMan,
    CoreCaptainMarvel,
    CoreSheHulk,
    CoreIronMan,
    CoreBlackPanther,
}

impl IdentitySet {
    pub fn to_string(&self) -> String {
        let str = match *self {
            Self::CoreSpiderMan => "Core - Spider Man",
            Self::CoreCaptainMarvel => "Core - Captain Marvel",
            Self::CoreSheHulk => "Core - She Hulk",
            Self::CoreIronMan => "Core - Iron Man",
            Self::CoreBlackPanther => "Core - Black Panther",
        };
        str.to_string()
    }

    pub fn get_key(&self) -> String {
        match *self {
            Self::CoreSpiderMan => "core_spider_man".to_string(),
            Self::CoreCaptainMarvel => "core_captain_marvel".to_string(),
            Self::CoreSheHulk => "core_she_hulk".to_string(),
            Self::CoreIronMan => "core_iron_man".to_string(),
            Self::CoreBlackPanther => "core_black_panther".to_string(),
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic> {
        todo!()
    }

    pub fn get_cards(&self) -> Vec<(CardBasic, fn(&mut World))> {
        todo!()
    }
}
