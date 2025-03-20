mod master_of_mayhem;
mod melter;
mod radioactive_man;
mod the_master_of_evil;
mod tiger_shark;
mod whirlwind;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        master_of_mayhem::get_info(),
        melter::get_info(),
        radioactive_man::get_info(),
        the_master_of_evil::get_info(),
        tiger_shark::get_info(),
        whirlwind::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        master_of_mayhem::get_card(),
        melter::get_card(),
        radioactive_man::get_card(),
        the_master_of_evil::get_card(),
        tiger_shark::get_card(),
        whirlwind::get_card(),
    ]
}
