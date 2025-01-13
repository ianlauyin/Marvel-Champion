use bevy::prelude::*;

use crate::features::cards::Card;

use super::{card_state::CardState, player::Player};

#[derive(Component, Clone)]
pub struct Deck {
    player: Player,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(player_tag: usize, cards: Vec<Card>, commands: Commands) -> Self {
        let deck = Self {
            player: Player::new(player_tag),
            cards,
        };
        deck.init(commands);
        deck
    }

    fn init(&self, mut commands: Commands) {
        commands
            .spawn((self.clone(), self.player.clone()))
            .with_children(|deck| {
                for card in self.cards.clone() {
                    deck.spawn((CardState::InPlay, card));
                }
            });
    }
}
