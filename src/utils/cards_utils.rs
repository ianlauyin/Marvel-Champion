use crate::features::cards::Card;

pub fn get_card_amount(deck_cards: &Vec<Card>) -> u8 {
    deck_cards
        .iter()
        .filter(|card| !matches!(card, Card::Hero(_) | Card::AlterEgo(_)))
        .count() as u8
}
