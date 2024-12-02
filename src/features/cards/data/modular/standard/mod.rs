use crate::features::cards::Card;

mod advance;
mod assault;
mod caught_off_guard;
mod gang_up;
mod shadow_of_the_past;

pub fn get_all() -> Vec<Card> {
    vec![
        advance::get_advance(),
        advance::get_advance(),
        assault::get_assault(),
        assault::get_assault(),
        caught_off_guard::get_caught_off_guard(),
        gang_up::get_gang_up(),
        shadow_of_the_past::get_shadow_of_the_past(),
    ]
}
