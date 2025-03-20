mod biomechanical_upgrades;
mod modok;
mod the_doomsday_chair;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        biomechanical_upgrades::get_info(),
        modok::get_info(),
        the_doomsday_chair::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        biomechanical_upgrades::get_card(),
        modok::get_card(),
        the_doomsday_chair::get_card(),
    ]
}
