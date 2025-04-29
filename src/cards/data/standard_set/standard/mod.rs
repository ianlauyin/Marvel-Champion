mod advance;
mod assault;
mod caught_off_guard;
mod gang_up;
mod shadow_of_the_past;

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        advance::get_card(),
        assault::get_card(),
        caught_off_guard::get_card(),
        gang_up::get_card(),
        shadow_of_the_past::get_card(),
    ]
}
