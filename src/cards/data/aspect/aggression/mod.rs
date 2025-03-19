mod chase_them_down;
mod combat_training;
mod hulk;
mod relentless_assault;
mod tac_team;
mod the_power_of_aggression;
mod tigra;
mod uppercut;

use crate::component::card::CardBasic;
use bevy::ecs::system::Commands;

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        chase_them_down::get_info(),
        combat_training::get_info(),
        hulk::get_info(),
        relentless_assault::get_info(),
        tac_team::get_info(),
        the_power_of_aggression::get_info(),
        tigra::get_info(),
        uppercut::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands))> {
    vec![
        chase_them_down::get_card(),
        combat_training::get_card(),
        hulk::get_card(),
        relentless_assault::get_card(),
        tac_team::get_card(),
        the_power_of_aggression::get_card(),
        tigra::get_card(),
        uppercut::get_card(),
    ]
}
