use bevy::{ecs::resource::Resource, log::warn, platform::collections::HashMap};

use crate::{cards::Aspect, component::Card};

#[derive(Resource)]
pub struct AspectCardDatas(HashMap<String, Card<'static>>);

impl AspectCardDatas {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        for card in Aspect::get_all_cards() {
            map.insert(card.id.to_string(), card);
        }
        Self(map)
    }

    pub fn get_batch_card_by_id(&self, ids: &Vec<String>) -> Vec<Card<'static>> {
        let mut infos = vec![];
        for id in ids {
            if let Some(info) = self.get_card_by_id(id) {
                infos.push(info.clone());
            } else {
                warn!("card not found: {}", id);
            }
        }
        infos
    }

    fn get_card_by_id(&self, id: &str) -> Option<&Card<'static>> {
        self.0.get(id)
    }
}
