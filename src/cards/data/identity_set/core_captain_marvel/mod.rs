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

use crate::component::Card;

pub fn get_identity_cards<'a>() -> Vec<Card<'a>> {
    vec![alter_ego::get_card(), hero::get_card()]
}

pub fn get_deck_cards<'a>() -> Vec<Card<'a>> {
    vec![
        alpha_flight_station::get_card(),
        captain_marvels_helmet::get_card(),
        cosmic_flight::get_card(),
        crisis_interdiction::get_card(),
        energy_absorption::get_card(),
        energy_channel::get_card(),
        photonic_blast::get_card(),
        spider_woman::get_card(),
    ]
}

pub fn get_obligation_card<'a>() -> Card<'a> {
    obligation::get_card()
}

pub fn get_out_of_play_cards<'a>() -> Vec<Card<'a>> {
    vec![
        nemesis_set::kree_manipulator::get_card(),
        nemesis_set::the_psyche_magnitron::get_card(),
        nemesis_set::yon_rogg::get_card(),
        nemesis_set::yon_roggs_treason::get_card(),
    ]
}
