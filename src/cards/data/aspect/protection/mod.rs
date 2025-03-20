mod armored_vest;
mod black_widow;
mod counter_punch;
mod get_behind_me;
mod indomitable;
mod luke_cage;
mod med_team;
mod the_power_of_protection;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        armored_vest::get_info(),
        black_widow::get_info(),
        counter_punch::get_info(),
        get_behind_me::get_info(),
        indomitable::get_info(),
        luke_cage::get_info(),
        med_team::get_info(),
        the_power_of_protection::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        armored_vest::get_card(),
        black_widow::get_card(),
        counter_punch::get_card(),
        get_behind_me::get_card(),
        indomitable::get_card(),
        luke_cage::get_card(),
        med_team::get_card(),
        the_power_of_protection::get_card(),
    ]
}
