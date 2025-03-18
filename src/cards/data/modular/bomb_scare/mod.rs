use crate::features::cards::Card;
mod bomb_scare;
mod explosion;
mod false_alarm;
mod hydra_bomber;

pub fn get_all() -> Vec<Card> {
    vec![
        bomb_scare::get_bomb_scare(),
        hydra_bomber::get_hydra_bomber(),
        hydra_bomber::get_hydra_bomber(),
        explosion::get_explosion(),
        false_alarm::get_false_alarm(),
        false_alarm::get_false_alarm(),
    ]
}
