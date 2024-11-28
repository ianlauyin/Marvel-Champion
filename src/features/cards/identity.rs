use super::{
    data::{core_captain_marvel, core_she_hulk, core_spider_man},
    Card,
};

#[derive(Clone)]
pub enum Identity {
    CoreSpiderMan,
    CoreCaptainMarvel,
    CoreSheHulk,
}

impl Identity {
    pub fn get_all() -> Vec<Self> {
        vec![
            Identity::CoreSpiderMan,
            Identity::CoreCaptainMarvel,
            Identity::CoreSheHulk,
        ]
    }

    pub fn get_all_cards() -> Vec<Card> {
        [
            Identity::CoreSpiderMan.get_cards(),
            Identity::CoreCaptainMarvel.get_cards(),
        ]
        .concat()
    }

    pub fn to_string(&self) -> String {
        match *self {
            Identity::CoreSpiderMan => "Core - Spider man".to_string(),
            Identity::CoreCaptainMarvel => "Core - Captain Marvel".to_string(),
            Identity::CoreSheHulk => "Core - She Hulk".to_string(),
        }
    }
    pub fn get_title_image_path(&self) -> String {
        match *self {
            Identity::CoreSpiderMan => "embedded://identity/core_spider_man.png".to_string(),
            Identity::CoreCaptainMarvel => {
                "embedded://identity/core_captain_marvel.png".to_string()
            }
            Identity::CoreSheHulk => "embedded://identity/core_she_hulk.png".to_string(),
        }
    }

    pub fn get_cards(&self) -> Vec<Card> {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_all(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_all(),
            Identity::CoreSheHulk => core_she_hulk::get_all(),
        }
    }

    pub fn get_player_cards(&self) -> Vec<Card> {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_player_cards(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_player_cards(),
            Identity::CoreSheHulk => core_she_hulk::get_player_cards(),
        }
    }

    pub fn get_obligation(&self) -> Card {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_obligation(),
            Identity::CoreCaptainMarvel => core_captain_marvel::get_obligation(),
            Identity::CoreSheHulk => core_she_hulk::get_obligation(),
        }
    }
}
