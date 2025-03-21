mod alter_ego;
mod arc_reactor;
mod hero;
mod mark_v_armor;
mod mark_v_helmet;
mod nemesis_set;
mod obligation;
mod pepper_potts;
mod powered_gauntlets;
mod repulsor_blast;
mod rocket_boots;
mod stark_tower;
mod supersonic_punch;
mod war_machine;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        alter_ego::get_info(),
        arc_reactor::get_info(),
        hero::get_info(),
        mark_v_armor::get_info(),
        mark_v_helmet::get_info(),
        obligation::get_info(),
        pepper_potts::get_info(),
        powered_gauntlets::get_info(),
        repulsor_blast::get_info(),
        rocket_boots::get_info(),
        stark_tower::get_info(),
        supersonic_punch::get_info(),
        war_machine::get_info(),
        nemesis_set::electric_whip_attack::get_info(),
        nemesis_set::electromagnetic_backlash::get_info(),
        nemesis_set::imminent_overload::get_info(),
        nemesis_set::whiplash::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        alter_ego::get_card(),
        arc_reactor::get_card(),
        hero::get_card(),
        mark_v_armor::get_card(),
        mark_v_helmet::get_card(),
        obligation::get_card(),
        pepper_potts::get_card(),
        powered_gauntlets::get_card(),
        repulsor_blast::get_card(),
        rocket_boots::get_card(),
        stark_tower::get_card(),
        supersonic_punch::get_card(),
        war_machine::get_card(),
        nemesis_set::electric_whip_attack::get_card(),
        nemesis_set::electromagnetic_backlash::get_card(),
        nemesis_set::imminent_overload::get_card(),
        nemesis_set::whiplash::get_card(),
    ]
}
