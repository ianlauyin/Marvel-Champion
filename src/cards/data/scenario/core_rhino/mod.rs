mod armored_rhino_suit;
mod breakin_and_takin;
mod charge;
mod crowd_control;
mod enhanced_ivory_horn;
mod hard_to_keep_down;
mod hydra_mercenary;
mod im_tough;
mod rhino_i;
mod rhino_ii;
mod rhino_iii;
mod sandman;
mod shocker;
mod stampede;
mod the_break_in_1a;
mod the_break_in_1b;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        armored_rhino_suit::get_info(),
        breakin_and_takin::get_info(),
        charge::get_info(),
        crowd_control::get_info(),
        enhanced_ivory_horn::get_info(),
        hard_to_keep_down::get_info(),
        hydra_mercenary::get_info(),
        im_tough::get_info(),
        rhino_i::get_info(),
        rhino_ii::get_info(),
        rhino_iii::get_info(),
        sandman::get_info(),
        shocker::get_info(),
        stampede::get_info(),
        the_break_in_1a::get_info(),
        the_break_in_1b::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        armored_rhino_suit::get_card(),
        breakin_and_takin::get_card(),
        charge::get_card(),
        crowd_control::get_card(),
        enhanced_ivory_horn::get_card(),
        hard_to_keep_down::get_card(),
        hydra_mercenary::get_card(),
        im_tough::get_card(),
        rhino_i::get_card(),
        rhino_ii::get_card(),
        rhino_iii::get_card(),
        sandman::get_card(),
        shocker::get_card(),
        stampede::get_card(),
        the_break_in_1a::get_card(),
        the_break_in_1b::get_card(),
    ]
}
