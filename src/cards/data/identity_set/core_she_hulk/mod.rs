mod alter_ego;
mod focused_rage;
mod gamma_slam;
mod ground_stomp;
mod hellcat;
mod hero;
mod legal_practice;
mod nemesis_set;
mod obligation;
mod one_two_punch;
mod split_personality;
mod superhuman_law_division;
mod superhuman_strength;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        alter_ego::get_info(),
        focused_rage::get_info(),
        gamma_slam::get_info(),
        ground_stomp::get_info(),
        hellcat::get_info(),
        hero::get_info(),
        legal_practice::get_info(),
        obligation::get_info(),
        one_two_punch::get_info(),
        split_personality::get_info(),
        superhuman_law_division::get_info(),
        superhuman_strength::get_info(),
        nemesis_set::genetically_enhanced::get_info(),
        nemesis_set::personal_challenge::get_info(),
        nemesis_set::titania::get_info(),
        nemesis_set::titanias_fury::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        alter_ego::get_card(),
        focused_rage::get_card(),
        gamma_slam::get_card(),
        ground_stomp::get_card(),
        hellcat::get_card(),
        hero::get_card(),
        legal_practice::get_card(),
        obligation::get_card(),
        one_two_punch::get_card(),
        split_personality::get_card(),
        superhuman_law_division::get_card(),
        superhuman_strength::get_card(),
        nemesis_set::genetically_enhanced::get_card(),
        nemesis_set::personal_challenge::get_card(),
        nemesis_set::titania::get_card(),
        nemesis_set::titanias_fury::get_card(),
    ]
}
