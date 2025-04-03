use bevy::color::Color;
use bevy::ecs::{entity::Entity, system::Commands};

use crate::cards::data::identity_set;
use crate::component::card::CardBasic;

use super::set_trait::SetTrait;

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

    pub fn get_identity_card_ids(&self) -> Vec<&str> {
        match *self {
            Self::CoreBlackPanther => vec!["core_040a", "core_040b"],
            Self::CoreCaptainMarvel => vec!["core_010a", "core_010b"],
            Self::CoreIronMan => vec!["core_029a", "core_029b"],
            Self::CoreSheHulk => vec!["core_019a", "core_019b"],
            Self::CoreSpiderMan => vec!["core_001a", "core_001b"],
        }
    }

    pub fn get_nemesis_scheme_id(&self) -> &str {
        match *self {
            Self::CoreBlackPanther => "core_156",
            Self::CoreCaptainMarvel => "core_176",
            Self::CoreIronMan => "core_171",
            Self::CoreSheHulk => "core_161",
            Self::CoreSpiderMan => "core_166",
        }
    }

    pub fn get_nemesis_id(&self) -> &str {
        match *self {
            Self::CoreBlackPanther => "core_157",
            Self::CoreCaptainMarvel => "core_177",
            Self::CoreIronMan => "core_172",
            Self::CoreSheHulk => "core_162",
            Self::CoreSpiderMan => "core_167",
        }
    }

    pub fn get_nemesis_cards_ids(&self) -> Vec<&str> {
        match *self {
            Self::CoreBlackPanther => vec!["core_158", "core_159"],
            Self::CoreCaptainMarvel => vec!["core_178", "core_179"],
            Self::CoreIronMan => vec!["core_173", "core_174"],
            Self::CoreSheHulk => vec!["core_163", "core_164"],
            Self::CoreSpiderMan => vec!["core_168", "core_169"],
        }
    }
}

impl SetTrait for IdentitySet {
    fn get_boxed_all() -> Vec<Box<dyn SetTrait>> {
        Self::get_all()
            .into_iter()
            .map(|set| Box::new(set) as Box<dyn SetTrait>)
            .collect()
    }

    fn to_str(&self) -> &str {
        match *self {
            Self::CoreSpiderMan => "Core - Spider Man",
            Self::CoreCaptainMarvel => "Core - Captain Marvel",
            Self::CoreSheHulk => "Core - She Hulk",
            Self::CoreIronMan => "Core - Iron Man",
            Self::CoreBlackPanther => "Core - Black Panther",
        }
    }

    fn get_key(&self) -> &str {
        match *self {
            Self::CoreSpiderMan => "core_spider_man",
            Self::CoreCaptainMarvel => "core_captain_marvel",
            Self::CoreSheHulk => "core_she_hulk",
            Self::CoreIronMan => "core_iron_man",
            Self::CoreBlackPanther => "core_black_panther",
        }
    }

    fn get_card_infos(&self) -> Vec<CardBasic<'static>> {
        match *self {
            Self::CoreSpiderMan => identity_set::core_spider_man::get_infos(),
            Self::CoreCaptainMarvel => identity_set::core_captain_marvel::get_infos(),
            Self::CoreSheHulk => identity_set::core_she_hulk::get_infos(),
            Self::CoreIronMan => identity_set::core_iron_man::get_infos(),
            Self::CoreBlackPanther => identity_set::core_black_panther::get_infos(),
        }
    }

    fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
        match *self {
            Self::CoreSpiderMan => identity_set::core_spider_man::get_cards(),
            Self::CoreCaptainMarvel => identity_set::core_captain_marvel::get_cards(),
            Self::CoreSheHulk => identity_set::core_she_hulk::get_cards(),
            Self::CoreIronMan => identity_set::core_iron_man::get_cards(),
            Self::CoreBlackPanther => identity_set::core_black_panther::get_cards(),
        }
    }

    fn get_thumbnail_key(&self) -> Option<String> {
        Some(format!("identity_set/{}", self.get_key()))
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
