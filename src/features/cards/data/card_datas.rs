use std::collections::HashMap;

use bevy::{
    app::{App, Plugin},
    prelude::Resource,
};

use crate::features::cards::{Card, Identity, ModularSet, VillainSet};

use super::{aggression, basic, justice, leadership, pool, protection};

#[derive(Resource)]
pub struct CardDatas(pub HashMap<String, Card>);

impl CardDatas {
    pub fn init() -> Self {
        let mut hashmap = HashMap::new();
        let all_cards = [
            Identity::get_all_cards(),
            VillainSet::get_all_cards(),
            ModularSet::get_all_cards(),
            CardDatas::get_aspect_cards(),
        ]
        .concat();

        for card in all_cards.iter() {
            hashmap.insert(card.get_id(), card.clone());
        }
        CardDatas(hashmap)
    }

    pub fn get_aspect_cards() -> Vec<Card> {
        [
            CardDatas::get_basic_cards(),
            CardDatas::get_aggression_cards(),
            CardDatas::get_justice_cards(),
            CardDatas::get_leadership_cards(),
            CardDatas::get_protection_cards(),
            CardDatas::get_pool_cards(),
        ]
        .concat()
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
    pub fn get_protection_cards() -> Vec<Card> {
        protection::get_all()
    }
    pub fn get_pool_cards() -> Vec<Card> {
        pool::get_all()
    }

    pub fn get(&self, card_id: &str) -> Card {
        let Some(card) = self.0.get(card_id) else {
            panic!("Cannot get card with id: {}", card_id);
        };
        card.clone()
    }
}

pub struct CardDataPlugin;

impl Plugin for CardDataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CardDatas::init());
    }
}
