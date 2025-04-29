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

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
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
