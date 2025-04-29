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

use crate::component::Card;

pub fn get_identity_cards<'a>() -> Vec<Card<'a>> {
    vec![alter_ego::get_card(), hero::get_card()]
}

pub fn get_deck_cards<'a>() -> Vec<Card<'a>> {
    vec![
        arc_reactor::get_card(),
        mark_v_armor::get_card(),
        mark_v_helmet::get_card(),
        pepper_potts::get_card(),
        powered_gauntlets::get_card(),
        repulsor_blast::get_card(),
        rocket_boots::get_card(),
        stark_tower::get_card(),
        supersonic_punch::get_card(),
        war_machine::get_card(),
    ]
}

pub fn get_obligation_card<'a>() -> Card<'a> {
    obligation::get_card()
}

pub fn get_out_of_play_cards<'a>() -> Vec<Card<'a>> {
    vec![
        nemesis_set::electric_whip_attack::get_card(),
        nemesis_set::electromagnetic_backlash::get_card(),
        nemesis_set::imminent_overload::get_card(),
        nemesis_set::whiplash::get_card(),
    ]
}
