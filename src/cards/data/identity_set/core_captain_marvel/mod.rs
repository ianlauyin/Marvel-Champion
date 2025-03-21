mod alpha_flight_station;
mod alter_ego;
mod captain_marvels_helmet;
mod cosmic_flight;
mod crisis_interdiction;
mod energy_absorption;
mod energy_channel;
mod hero;
mod nemesis_set;
mod obligation;
mod photonic_blast;
mod spider_woman;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        alpha_flight_station::get_info(),
        alter_ego::get_info(),
        captain_marvels_helmet::get_info(),
        cosmic_flight::get_info(),
        crisis_interdiction::get_info(),
        energy_absorption::get_info(),
        energy_channel::get_info(),
        hero::get_info(),
        obligation::get_info(),
        photonic_blast::get_info(),
        spider_woman::get_info(),
        nemesis_set::kree_manipulator::get_info(),
        nemesis_set::the_psyche_magnitron::get_info(),
        nemesis_set::yon_rogg::get_info(),
        nemesis_set::yon_roggs_treason::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        alpha_flight_station::get_card(),
        alter_ego::get_card(),
        captain_marvels_helmet::get_card(),
        cosmic_flight::get_card(),
        crisis_interdiction::get_card(),
        energy_absorption::get_card(),
        energy_channel::get_card(),
        hero::get_card(),
        obligation::get_card(),
        photonic_blast::get_card(),
        spider_woman::get_card(),
        nemesis_set::kree_manipulator::get_card(),
        nemesis_set::the_psyche_magnitron::get_card(),
        nemesis_set::yon_rogg::get_card(),
        nemesis_set::yon_roggs_treason::get_card(),
    ]
}
