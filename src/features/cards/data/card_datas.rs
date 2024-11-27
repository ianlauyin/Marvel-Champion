use std::collections::HashMap;

use bevy::{
    app::{App, Plugin},
    prelude::Resource,
};

use crate::features::cards::{Card, Identity};

use super::{aggression, basic, identity_specific_cards::core_spider_man, justice, leadership};

#[derive(Resource)]
pub struct CardDatas(pub HashMap<String, Card>);

impl CardDatas {
    pub fn init() -> Self {
        let mut hashmap = HashMap::new();
        let identity_cards = [core_spider_man::get_all()].concat();
        let aspect_cards = [
            CardDatas::get_basic_cards(),
            CardDatas::get_aggression_cards(),
            CardDatas::get_justice_cards(),
            CardDatas::get_leadership_cards(),
        ]
        .concat();
        let all_cards = [identity_cards, aspect_cards].concat();

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

    pub fn get_identity_player_cards(identity: Identity) -> Vec<Card> {
        match identity {
            Identity::CoreSpiderMan => core_spider_man::get_player_cards(),
        }
    }

    pub fn get_obligation(identity: Identity) -> Card {
        match identity {
            Identity::CoreSpiderMan => core_spider_man::get_obligation(),
        }
    }

    pub fn get_basic_cards() -> Vec<Card> {
        basic::get_all()
    }

    pub fn get_aggression_cards() -> Vec<Card> {
        aggression::get_all()
    }
    pub fn get_justice_cards() -> Vec<Card> {
        justice::get_all()
    }
    pub fn get_leadership_cards() -> Vec<Card> {
        leadership::get_all()
    }
}

pub struct CardDataPlugin;

impl Plugin for CardDataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CardDatas::init());
    }
}
