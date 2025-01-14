use crate::features::cards::Card;

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

pub fn get_standard_villain_cards() -> Vec<Card> {
    vec![rhino_i::get_rhino_i(), rhino_ii::get_rhino_ii()]
}

pub fn get_expert_villain_cards() -> Vec<Card> {
    vec![rhino_ii::get_rhino_ii(), rhino_iii::get_rhino_iii()]
}

pub fn get_main_scheme_cards() -> Vec<Card> {
    vec![
        the_break_in_1a::get_the_break_in_1a(),
        the_break_in_1b::get_the_break_in_1b(),
    ]
}

pub fn get_encounter_cards() -> Vec<Card> {
    vec![
        armored_rhino_suit::get_armored_rhino_suit(),
        charge::get_charge(),
        charge::get_charge(),
        enhanced_ivory_horn::get_enhanced_ivory_horn(),
        hydra_mercenary::get_hydra_mercenary(),
        hydra_mercenary::get_hydra_mercenary(),
        sandman::get_sandman(),
        shocker::get_shocker(),
        hard_to_keep_down::get_hard_to_keep_down(),
        hard_to_keep_down::get_hard_to_keep_down(),
        im_tough::get_im_tough(),
        im_tough::get_im_tough(),
        stampede::get_stampede(),
        stampede::get_stampede(),
        stampede::get_stampede(),
        breakin_and_takin::get_breakin_and_takin(),
        crowd_control::get_crowd_control(),
    ]
}

pub fn get_all() -> Vec<Card> {
    [
        vec![
            rhino_i::get_rhino_i(),
            rhino_ii::get_rhino_ii(),
            rhino_iii::get_rhino_iii(),
        ],
        get_main_scheme_cards(),
        get_encounter_cards(),
    ]
    .concat()
}
