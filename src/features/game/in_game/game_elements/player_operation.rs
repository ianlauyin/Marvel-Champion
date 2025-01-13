use crate::features::cards::{Card, Identity};
use bevy::prelude::*;

use super::components::{CardState, Deck, Player};

pub struct PlayerOperation;

impl PlayerOperation {
    pub fn new_player(
        mut commands: Commands,
        player_tag: usize,
        identity: &Identity,
        deck_cards: Vec<Card>,
    ) {
        let player = Player::new(player_tag, identity);
        Deck::new(&player, deck_cards.clone(), commands.reborrow());
        commands.spawn((player.clone(), identity.get_alter_ego(), CardState::InPlay));
        for card in identity.get_hero() {
            commands.spawn((player.clone(), card.clone(), CardState::OutPlay));
        }
    }
}
