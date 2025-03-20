mod get_ready;
mod hawkeye;
mod inspired;
mod lead_from_the_front;
mod make_the_call;
mod maria_hill;
mod the_power_of_leadership;
mod the_triskelion;
mod vision;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        get_ready::get_info(),
        hawkeye::get_info(),
        inspired::get_info(),
        lead_from_the_front::get_info(),
        make_the_call::get_info(),
        maria_hill::get_info(),
        the_power_of_leadership::get_info(),
        the_triskelion::get_info(),
        vision::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        get_ready::get_card(),
        hawkeye::get_card(),
        inspired::get_card(),
        lead_from_the_front::get_card(),
        make_the_call::get_card(),
        maria_hill::get_card(),
        the_power_of_leadership::get_card(),
        the_triskelion::get_card(),
        vision::get_card(),
    ]
}
