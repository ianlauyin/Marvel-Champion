use crate::features::cards::Card;

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

pub fn get_player_cards() -> Vec<Card> {
    vec![
        shuri::get_shuri(),
        ancestral_knowledge::get_ancestral_knowledge(),
        wakanda_forever_a::get_wakanda_forever_a(),
        wakanda_forever_b::get_wakanda_forever_b(),
        wakanda_forever_c::get_wakanda_forever_c(),
        wakanda_forever_d::get_wakanda_forever_d(),
        wakanda_forever_d::get_wakanda_forever_d(),
        vibranium::get_vibranium(),
        vibranium::get_vibranium(),
        vibranium::get_vibranium(),
        the_golden_city::get_the_golden_city(),
        energy_daggers::get_energy_daggers(),
        panther_claws::get_panther_claws(),
        tactical_genius::get_tactical_genius(),
        vibranium_suit::get_vibranium_suit(),
    ]
}

pub fn get_identity_cards() -> Vec<Card> {
    vec![alter_ego::get_alter_ego(), hero::get_hero()]
}

pub fn get_obligation() -> Card {
    obligation::get_obligation()
}

pub fn get_all() -> Vec<Card> {
    [
        get_identity_cards(),
        get_player_cards(),
        vec![get_obligation()],
        nemesis_set::get_nemesis_set(),
    ]
    .concat()
}

pub use nemesis_set::get_nemesis_set;
