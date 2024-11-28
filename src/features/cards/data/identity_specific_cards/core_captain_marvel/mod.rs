use crate::features::cards::Card;

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

pub fn get_player_cards() -> Vec<Card> {
    vec![
        hero::get_hero(),
        alter_ego::get_alter_ego(),
        spider_woman::get_spider_woman(),
        crisis_interdiction::get_crisis_interdiction(),
        photonic_blast::get_photonic_blast(),
        energy_absorption::get_energy_absorption(),
        alpha_flight_station::get_alpha_flight_station(),
        captain_marvels_helmet::get_captain_marvels_helmet(),
        cosmic_flight::get_cosmic_flight(),
        energy_channel::get_energy_channel(),
    ]
}

pub fn get_obligation() -> Card {
    obligation::get_obligation()
}

pub fn get_all() -> Vec<Card> {
    [
        get_player_cards(),
        vec![get_obligation()],
        nemesis_set::get_nemesis_set(),
    ]
    .concat()
}
