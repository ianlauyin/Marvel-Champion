use crate::features::cards::{Card, Identity};

pub struct GamePlayer {
    player_order: usize,
    health: u8,
    is_hero_form: bool,
    in_play_cards: Vec<Card>,
    out_of_play_cards: Vec<Card>,
    hand_cards: Vec<Card>,
    deck_cards: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl GamePlayer {
    pub fn new(player_order: usize, identity: &Identity, deck: Vec<Card>) -> Self {
        Self {
            player_order,
            health: identity.get_health(),
            is_hero_form: false,
            discard_pile: vec![],
            in_play_cards: vec![],
            out_of_play_cards: vec![],
            deck_cards: deck,
            hand_cards: vec![],
        }
    }

    pub fn is_first_player(&self) -> bool {
        self.player_order == 0
    }
}
