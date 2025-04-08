use bevy::{ecs::system::Resource, utils::HashMap};

use crate::{cards::IdentitySet, flow::game::component::PlayerTag};

#[derive(Resource, Default)]
pub struct PlayersInfo {
    infos: HashMap<PlayerTag, PlayerInfo>,
}

impl PlayersInfo {
    pub fn save_player(&mut self, identity: &IdentitySet, deck_card_ids: Vec<String>) {
        if let Some((_, player_info)) = self.get_by_identity_mut(identity) {
            player_info.deck_card_ids = deck_card_ids;
            return;
        }
        for player_number in PlayerTag::get_all_asc() {
            if !self.infos.contains_key(&player_number) {
                self.infos.insert(
                    player_number,
                    PlayerInfo {
                        identity: identity.clone(),
                        deck_card_ids,
                    },
                );
                return;
            }
        }
    }

    pub fn remove_player(&mut self, identity: &IdentitySet) {
        if let Some((player_tag, _)) = self.get_by_identity(identity) {
            self.infos.remove(&player_tag.clone());
            self.rearrange_players();
        }
    }

    pub fn contains_identity(&self, identity: &IdentitySet) -> bool {
        self.infos
            .iter()
            .any(|player_info| player_info.1.identity == *identity)
    }

    pub fn get_by_identity(&self, identity: &IdentitySet) -> Option<(&PlayerTag, &PlayerInfo)> {
        self.infos
            .iter()
            .find(|player_info| player_info.1.identity == *identity)
    }

    fn get_by_identity_mut(
        &mut self,
        identity: &IdentitySet,
    ) -> Option<(&PlayerTag, &mut PlayerInfo)> {
        self.infos
            .iter_mut()
            .find(|player_info| player_info.1.identity == *identity)
    }

    fn rearrange_players(&mut self) {
        let mut new_map: HashMap<PlayerTag, PlayerInfo> = HashMap::new();
        let mut player_numbers_iter = PlayerTag::get_all_asc().into_iter();
        for player_number in PlayerTag::get_all_asc() {
            if let Some(player_info) = self.infos.get(&player_number) {
                new_map.insert(player_numbers_iter.next().unwrap(), player_info.clone());
            }
        }
        self.infos = new_map;
    }
}

#[derive(Clone, PartialEq)]
pub struct PlayerInfo {
    identity: IdentitySet,
    deck_card_ids: Vec<String>,
}
