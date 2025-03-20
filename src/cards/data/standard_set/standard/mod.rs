mod advance;
mod assault;
mod caught_off_guard;
mod gang_up;
mod shadow_of_the_past;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        advance::get_info(),
        assault::get_info(),
        caught_off_guard::get_info(),
        gang_up::get_info(),
        shadow_of_the_past::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        advance::get_card(),
        assault::get_card(),
        caught_off_guard::get_card(),
        gang_up::get_card(),
        shadow_of_the_past::get_card(),
    ]
}
