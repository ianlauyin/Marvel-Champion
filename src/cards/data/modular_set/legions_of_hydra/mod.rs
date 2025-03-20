mod hydra_soldier;
mod legions_of_hydra;
mod madame_hydra;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        hydra_soldier::get_info(),
        legions_of_hydra::get_info(),
        madame_hydra::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        hydra_soldier::get_card(),
        legions_of_hydra::get_card(),
        madame_hydra::get_card(),
    ]
}
