mod alter_ego;
mod ancestral_knowledge;
mod energy_daggers;
mod hero;
mod nemesis_set;
mod obligation;
mod panther_claws;
mod shuri;
mod tactical_genius;
mod the_golden_city;
mod vibranium;
mod vibranium_suit;
mod wakanda_forever_a;
mod wakanda_forever_b;
mod wakanda_forever_c;
mod wakanda_forever_d;

use crate::component::Card;

pub fn get_identity_cards<'a>() -> Vec<Card<'a>> {
    vec![alter_ego::get_card(), hero::get_card()]
}

pub fn get_deck_cards<'a>() -> Vec<Card<'a>> {
    vec![
        ancestral_knowledge::get_card(),
        energy_daggers::get_card(),
        panther_claws::get_card(),
        shuri::get_card(),
        tactical_genius::get_card(),
        the_golden_city::get_card(),
        vibranium::get_card(),
        vibranium_suit::get_card(),
        wakanda_forever_a::get_card(),
        wakanda_forever_b::get_card(),
        wakanda_forever_c::get_card(),
        wakanda_forever_d::get_card(),
    ]
}

pub fn get_obligation_card<'a>() -> Card<'a> {
    obligation::get_card()
}

pub fn get_out_of_play_cards<'a>() -> Vec<Card<'a>> {
    vec![
        nemesis_set::heart_shaped_herb::get_card(),
        nemesis_set::killmonger::get_card(),
        nemesis_set::ritual_combat::get_card(),
        nemesis_set::usurp_the_throne::get_card(),
    ]
}
