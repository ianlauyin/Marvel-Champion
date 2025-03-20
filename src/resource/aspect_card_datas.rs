use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Resource},
    },
    utils::HashMap,
};

use crate::{cards::Aspect, component::card::CardBasic};

#[derive(Resource)]
pub struct AspectCardDatas<'a>(HashMap<String, (CardBasic<'a>, fn(Commands) -> Entity)>);

impl<'a> AspectCardDatas<'a> {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        for card in Aspect::get_all_cards() {
            map.insert(card.0.get_key(), card);
        }
        Self(map)
    }
}
