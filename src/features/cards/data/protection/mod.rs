use crate::features::cards::Card;

mod armored_vest;
mod black_widow;
mod counter_punch;
mod get_behind_me;
mod indomitable;
mod luke_cage;
mod med_team;
mod the_power_of_protection;

pub fn get_all() -> Vec<Card> {
    vec![
        black_widow::get_black_widow(),
        luke_cage::get_luke_cage(),
        counter_punch::get_counter_punch(),
        get_behind_me::get_get_behind_me(),
        the_power_of_protection::get_the_power_of_protection(),
        med_team::get_med_team(),
        armored_vest::get_armored_vest(),
        indomitable::get_indomitable(),
    ]
}
