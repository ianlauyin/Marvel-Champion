use crate::features::{
    cards::{Card, CardDatas, Identity},
    game::game_selector::SelectedPlayers,
};
use bevy::prelude::*;
use rand::seq::SliceRandom;

use super::{
    components::{CardState, DeckCard, Player},
    FirstPlayer, PlayerNumber,
};

pub struct PlayerOperation {
    player_tag: usize,
}

impl PlayerOperation {
    pub fn init_game(
        mut commands: Commands,
        selected_players: &Res<SelectedPlayers>,
        card_datas: Res<CardDatas>,
    ) {
        commands.insert_resource(PlayerNumber(selected_players.0.len()));
        commands.insert_resource(FirstPlayer::new());
        for (index, player) in selected_players.0.iter().enumerate() {
            let deck_cards = card_datas.from_ids(&player.deck.card_ids);
            new_player(commands.reborrow(), index + 1, &player.identity, deck_cards);
        }
    }

    pub fn flip_form(&self, mut identity_cards_q: Query<(&Player, &Card, &mut CardState)>) {
        for (player, card, mut card_state) in identity_cards_q.iter_mut() {
            if player.is(self.player_tag) && matches!(card, Card::Hero(_) | Card::AlterEgo(_)) {
                card_state.toggle();
            }
        }
    }
}

fn new_player(
    mut commands: Commands,
    player_tag: usize,
    identity: &Identity,
    deck_cards: Vec<Card>,
) {
    let player = Player::new(player_tag, identity.get_health());
    init_identity_cards(commands.reborrow(), &player, identity);
    init_deck(commands, &player, deck_cards);
}

fn init_identity_cards(mut commands: Commands, player: &Player, identity: &Identity) {
    commands.spawn((player.clone(), identity.get_alter_ego(), CardState::InPlay));
    for card in identity.get_hero() {
        commands.spawn((player.clone(), card.clone(), CardState::OutPlay));
    }
}

fn init_deck(mut commands: Commands, player: &Player, mut deck_cards: Vec<Card>) {
    deck_cards.shuffle(&mut rand::thread_rng());
    for (index, card) in deck_cards.iter().enumerate() {
        commands.spawn((
            player.clone(),
            card.clone(),
            CardState::OutPlay,
            DeckCard::new(index),
        ));
    }
}
