pub mod basic;
pub mod identity_specific_cards;

use std::collections::HashMap;

use bevy::{
    app::{App, Plugin},
    prelude::Resource,
};
use identity_specific_cards::core_spider_man;

use super::{Card, Identity};

#[derive(Resource)]
pub struct CardDatas(pub HashMap<String, Card>);

impl CardDatas {
    pub fn init() -> Self {
        let mut hashmap = HashMap::new();
        let all_cards = [core_spider_man::get_all(), basic::core::get_all()].concat();

        for card in all_cards.iter() {
            hashmap.insert(card.get_card_id(), card.clone());
        }
        CardDatas(hashmap)
    }

    pub fn get_identity_cards(identity: Identity) -> Vec<Card> {
        match identity {
            Identity::CoreSpiderMan => core_spider_man::get_all(),
        }
    }
    pub fn get_basic_cards() -> Vec<Card> {
        basic::core::get_all()
    }
}

pub struct CardDataPlugin;

impl Plugin for CardDataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CardDatas::init());
    }
}
