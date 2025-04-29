mod bomb_scare;
mod explosion;
mod false_alarm;
mod hydra_bomber;

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        bomb_scare::get_card(),
        explosion::get_card(),
        false_alarm::get_card(),
        hydra_bomber::get_card(),
    ]
}
