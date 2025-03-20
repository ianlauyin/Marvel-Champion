use bevy::ecs::{entity::Entity, system::Commands};

use crate::component::card::CardBasic;

#[derive(Clone)]
pub enum IdentitySet {
    CoreSpiderMan,
    CoreCaptainMarvel,
    CoreSheHulk,
    CoreIronMan,
    CoreBlackPanther,
}

impl IdentitySet {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::CoreSpiderMan,
            Self::CoreCaptainMarvel,
            Self::CoreSheHulk,
            Self::CoreIronMan,
            Self::CoreBlackPanther,
        ]
    }

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

    pub fn get_nemesis_id(&self) -> String {
        match *self {
            Self::CoreBlackPanther => "core_157".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn get_nemesis_scheme_id(&self) -> String {
        match *self {
            Self::CoreBlackPanther => "core_156".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn get_nemesis_cards_ids(&self) -> Vec<String> {
        match *self {
            Self::CoreBlackPanther => vec!["core_158".to_string(), "core_159".to_string()],
            _ => vec![],
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic> {
        todo!()
    }

    pub fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
        todo!()
    }
}
