use bevy::ecs::{entity::Entity, system::Commands};

use crate::cards::data::identity_set;
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

    pub fn get_nemesis_scheme_id(&self) -> String {
        match *self {
            Self::CoreBlackPanther => "core_156".to_string(),
            Self::CoreCaptainMarvel => "core_176".to_string(),
            Self::CoreIronMan => "core_171".to_string(),
            Self::CoreSheHulk => "core_161".to_string(),
            Self::CoreSpiderMan => "core_166".to_string(),
        }
    }

    pub fn get_nemesis_id(&self) -> String {
        match *self {
            Self::CoreBlackPanther => "core_157".to_string(),
            Self::CoreCaptainMarvel => "core_177".to_string(),
            Self::CoreIronMan => "core_172".to_string(),
            Self::CoreSheHulk => "core_162".to_string(),
            Self::CoreSpiderMan => "core_167".to_string(),
        }
    }

    pub fn get_nemesis_cards_ids(&self) -> Vec<String> {
        match *self {
            Self::CoreBlackPanther => vec!["core_158".to_string(), "core_159".to_string()],
            Self::CoreCaptainMarvel => vec!["core_178".to_string(), "core_179".to_string()],
            Self::CoreIronMan => vec!["core_173".to_string(), "core_174".to_string()],
            Self::CoreSheHulk => vec!["core_163".to_string(), "core_164".to_string()],
            Self::CoreSpiderMan => vec!["core_168".to_string(), "core_169".to_string()],
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic> {
        match *self {
            Self::CoreSpiderMan => identity_set::core_spider_man::get_infos(),
            Self::CoreCaptainMarvel => identity_set::core_captain_marvel::get_infos(),
            Self::CoreSheHulk => identity_set::core_she_hulk::get_infos(),
            Self::CoreIronMan => identity_set::core_iron_man::get_infos(),
            Self::CoreBlackPanther => identity_set::core_black_panther::get_infos(),
        }
    }

    pub fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
        match *self {
            Self::CoreSpiderMan => identity_set::core_spider_man::get_cards(),
            Self::CoreCaptainMarvel => identity_set::core_captain_marvel::get_cards(),
            Self::CoreSheHulk => identity_set::core_she_hulk::get_cards(),
            Self::CoreIronMan => identity_set::core_iron_man::get_cards(),
            Self::CoreBlackPanther => identity_set::core_black_panther::get_cards(),
        }
    }
}
