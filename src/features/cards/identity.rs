use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use super::{
    data::{
        core_black_panther, core_captain_marvel, core_iron_man, core_she_hulk, core_spider_man,
    },
    deck_validator::DeckValidator,
    Card,
};

#[derive(Component, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, Debug)]
pub enum Identity {
    CoreSpiderMan,
    CoreCaptainMarvel,
    CoreSheHulk,
    CoreIronMan,
    CoreBlackPanther,
}

impl Identity {
    pub fn get_all() -> Vec<Self> {
        vec![
            Identity::CoreSpiderMan,
            Identity::CoreCaptainMarvel,
            Identity::CoreSheHulk,
            Identity::CoreIronMan,
            Identity::CoreBlackPanther,
        ]
    }

    pub fn get_all_cards() -> Vec<Card> {
        let mut cards = Vec::new();
        for identity in Identity::get_all() {
            cards.push(identity.get_cards());
        }
        cards.concat()
    }

    pub fn get_validator(&self) -> DeckValidator {
        match self {
            _ => DeckValidator::default(self.clone()),
        }
    }

    pub fn to_string(&self) -> String {
        let str = match *self {
            Identity::CoreSpiderMan => "Core - Spider Man",
            Identity::CoreCaptainMarvel => "Core - Captain Marvel",
            Identity::CoreSheHulk => "Core - She Hulk",
            Identity::CoreIronMan => "Core - Iron Man",
            Identity::CoreBlackPanther => "Core - Black Panther",
        };
        str.to_string()
    }

    pub fn get_key(&self) -> String {
        let key = match *self {
            Identity::CoreSpiderMan => "core_spider_man",
            Identity::CoreCaptainMarvel => "core_captain_marvel",
            Identity::CoreSheHulk => "core_she_hulk",
            Identity::CoreIronMan => "core_iron_man",
            Identity::CoreBlackPanther => "core_black_panther",
        };
        key.to_string()
    }

    pub fn get_title_image_path(&self) -> String {
        let prefix = "embedded://identity/";
        let postfix = ".png";
        let name = self.get_key();
        format!("{prefix}{name}{postfix}")
    }

    pub fn get_health(&self) -> u8 {
        for card in self.get_cards() {
            if let Card::AlterEgo(alter_ego_card) = card {
                return alter_ego_card.initial_hit_points;
            }
        }
        panic!("No alter ego card found for identity: {:?}", self);
    }

    pub fn get_identity_cards(&self) -> Vec<Card> {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_identity_cards(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_identity_cards(),
            Identity::CoreSheHulk => core_she_hulk::get_identity_cards(),
            Identity::CoreIronMan => core_iron_man::get_identity_cards(),
            Identity::CoreBlackPanther => core_black_panther::get_identity_cards(),
        }
    }

    pub fn get_cards(&self) -> Vec<Card> {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_all(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_all(),
            Identity::CoreSheHulk => core_she_hulk::get_all(),
            Identity::CoreIronMan => core_iron_man::get_all(),
            Identity::CoreBlackPanther => core_black_panther::get_all(),
        }
    }

    pub fn get_player_cards(&self) -> Vec<Card> {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_player_cards(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_player_cards(),
            Identity::CoreSheHulk => core_she_hulk::get_player_cards(),
            Identity::CoreIronMan => core_iron_man::get_player_cards(),
            Identity::CoreBlackPanther => core_black_panther::get_player_cards(),
        }
    }

    pub fn get_obligation(&self) -> Card {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_obligation(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_obligation(),
            Identity::CoreSheHulk => core_she_hulk::get_obligation(),
            Identity::CoreIronMan => core_iron_man::get_obligation(),
            Identity::CoreBlackPanther => core_black_panther::get_obligation(),
        }
    }

    pub fn get_nemesis_set(&self) -> Vec<Card> {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_nemesis_set(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_nemesis_set(),
            Identity::CoreSheHulk => core_she_hulk::get_nemesis_set(),
            Identity::CoreIronMan => core_iron_man::get_nemesis_set(),
            Identity::CoreBlackPanther => core_black_panther::get_nemesis_set(),
        }
    }
}
