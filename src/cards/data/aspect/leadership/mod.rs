mod get_ready;
mod hawkeye;
mod inspired;
mod lead_from_the_front;
mod make_the_call;
mod maria_hill;
mod the_power_of_leadership;
mod the_triskelion;
mod vision;

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        get_ready::get_card(),
        hawkeye::get_card(),
        inspired::get_card(),
        lead_from_the_front::get_card(),
        make_the_call::get_card(),
        maria_hill::get_card(),
        the_power_of_leadership::get_card(),
        the_triskelion::get_card(),
        vision::get_card(),
    ]
}
