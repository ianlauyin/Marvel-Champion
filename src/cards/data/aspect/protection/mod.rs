mod armored_vest;
mod black_widow;
mod counter_punch;
mod get_behind_me;
mod indomitable;
mod luke_cage;
mod med_team;
mod the_power_of_protection;

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        armored_vest::get_card(),
        black_widow::get_card(),
        counter_punch::get_card(),
        get_behind_me::get_card(),
        indomitable::get_card(),
        luke_cage::get_card(),
        med_team::get_card(),
        the_power_of_protection::get_card(),
    ]
}
