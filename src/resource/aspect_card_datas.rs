use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Resource},
    },
    log::warn,
    utils::HashMap,
};

use crate::{cards::Aspect, component::card::CardBasic};

#[derive(Resource)]
pub struct AspectCardDatas(HashMap<String, (CardBasic<'static>, fn(Commands) -> Entity)>);

impl AspectCardDatas {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        for card in Aspect::get_all_cards() {
            map.insert(card.0.id.to_string(), card);
        }
        Self(map)
    }

    pub fn get_batch_info_by_id(&self, ids: &Vec<String>) -> Vec<CardBasic<'static>> {
        let mut infos = vec![];
        for id in ids {
            if let Some(info) = self.get_info_by_id(id) {
                infos.push(info.clone());
            } else {
                warn!("card not found: {}", id);
            }
        }
        infos
    }

    fn get_info_by_id(&self, id: &str) -> Option<&CardBasic<'static>> {
        self.0.get(id).map(|(card, _)| card)
    }
}
