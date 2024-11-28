use super::{
    data::{core_captain_marvel, core_iron_man, core_she_hulk, core_spider_man},
    Card,
};

#[derive(Clone)]
pub enum Identity {
    CoreSpiderMan,
    CoreCaptainMarvel,
    CoreSheHulk,
    CoreIronMan,
}

impl Identity {
    pub fn get_all() -> Vec<Self> {
        vec![
            Identity::CoreSpiderMan,
            Identity::CoreCaptainMarvel,
            Identity::CoreSheHulk,
            Identity::CoreIronMan,
        ]
    }

    pub fn get_all_cards() -> Vec<Card> {
        let mut cards = Vec::new();
        for identity in Identity::get_all() {
            cards.push(identity.get_cards());
        }
        cards.concat()
    }

    pub fn to_string(&self) -> String {
        let str = match *self {
            Identity::CoreSpiderMan => "Core - Spider Man",
            Identity::CoreCaptainMarvel => "Core - Captain Marvel",
            Identity::CoreSheHulk => "Core - She Hulk",
            Identity::CoreIronMan => "Core - Iron Man",
        };
        str.to_string()
    }
    pub fn get_title_image_path(&self) -> String {
        let prefix = "embedded://identity/";
        let postfix = ".png";
        let name = match *self {
            Identity::CoreSpiderMan => "core_spider_man",
            Identity::CoreCaptainMarvel => "core_captain_marvel",
            Identity::CoreSheHulk => "core_she_hulk",
            Identity::CoreIronMan => "core_iron_man",
        };
        format!("{prefix}{name}{postfix}")
    }

    pub fn get_cards(&self) -> Vec<Card> {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_all(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_all(),
            Identity::CoreSheHulk => core_she_hulk::get_all(),
            Identity::CoreIronMan => core_iron_man::get_all(),
        }
    }

    pub fn get_player_cards(&self) -> Vec<Card> {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_player_cards(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_player_cards(),
            Identity::CoreSheHulk => core_she_hulk::get_player_cards(),
            Identity::CoreIronMan => core_iron_man::get_player_cards(),
        }
    }

    pub fn get_obligation(&self) -> Card {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_obligation(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_obligation(),
            Identity::CoreSheHulk => core_she_hulk::get_obligation(),
            Identity::CoreIronMan => core_iron_man::get_obligation(),
        }
    }

    pub fn get_nemesis_set(&self) -> Vec<Card> {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_nemesis_set(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_nemesis_set(),
            Identity::CoreSheHulk => core_she_hulk::get_nemesis_set(),
            Identity::CoreIronMan => core_iron_man::get_nemesis_set(),
        }
    }
}
