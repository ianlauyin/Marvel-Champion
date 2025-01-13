use crate::{
    features::cards::{Card, Identity},
    systems::Deck,
};
use bevy::prelude::*;

pub struct PlayerInfo {
    player_tag: usize,
    identity: Identity,
    hand: Vec<Card>,
    deck: Vec<Card>,
    discard_piles: Vec<Card>,
}

impl PlayerInfo {
    pub fn new(
        commands: Commands,
        player_tag: usize,
        identity: &Identity,
        deck: Vec<Card>,
    ) -> Self {
        let player_info = Self {
            player_tag,
            identity: identity.clone(),
            deck,
            hand: vec![],
            discard_piles: vec![],
        };
        player_info.init(commands);
        player_info
    }

    pub fn get() {}

    fn init(&self, mut commands: Commands) {
        
    }
}
