mod concussion_blasters;
mod concussive_blast;
mod under_attack;
mod vibranium_armor;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        concussion_blasters::get_info(),
        concussive_blast::get_info(),
        under_attack::get_info(),
        vibranium_armor::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        concussion_blasters::get_card(),
        concussive_blast::get_card(),
        under_attack::get_card(),
        vibranium_armor::get_card(),
    ]
}
