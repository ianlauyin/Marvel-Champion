mod bomb_scare;
mod explosion;
mod false_alarm;
mod hydra_bomber;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        bomb_scare::get_info(),
        explosion::get_info(),
        false_alarm::get_info(),
        hydra_bomber::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        bomb_scare::get_card(),
        explosion::get_card(),
        false_alarm::get_card(),
        hydra_bomber::get_card(),
    ]
}
