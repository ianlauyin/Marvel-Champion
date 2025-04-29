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

use crate::component::Card;

pub fn get_identity_cards<'a>() -> Vec<Card<'a>> {
    vec![alter_ego::get_card(), hero::get_card()]
}

pub fn get_deck_cards<'a>() -> Vec<Card<'a>> {
    vec![
        focused_rage::get_card(),
        gamma_slam::get_card(),
        ground_stomp::get_card(),
        hellcat::get_card(),
        legal_practice::get_card(),
        one_two_punch::get_card(),
        split_personality::get_card(),
        superhuman_law_division::get_card(),
        superhuman_strength::get_card(),
    ]
}

pub fn get_obligation_card<'a>() -> Card<'a> {
    obligation::get_card()
}

pub fn get_out_of_play_cards<'a>() -> Vec<Card<'a>> {
    vec![
        nemesis_set::genetically_enhanced::get_card(),
        nemesis_set::personal_challenge::get_card(),
        nemesis_set::titania::get_card(),
        nemesis_set::titanias_fury::get_card(),
    ]
}
