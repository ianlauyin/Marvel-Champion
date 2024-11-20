use super::{core_spider_man, Card};

pub fn get_all_identity() -> Vec<Identity> {
    vec![Identity::CoreSpiderMan]
}

#[derive(Clone)]
pub enum Identity {
    CoreSpiderMan,
}

impl Identity {
    pub fn to_string(&self) -> String {
        match *self {
            Identity::CoreSpiderMan => "Core - Spider man".to_string(),
        }
    }
    pub fn get_title_image_path(&self) -> String {
        match *self {
            Identity::CoreSpiderMan => "embedded://identity/core_spider_man.png".to_string(),
        }
    }

    pub fn get_cards(&self, player_number: u8) -> Vec<Card> {
        match *self {
            Identity::CoreSpiderMan => core_spider_man::get_all(player_number),
        }
    }
}
