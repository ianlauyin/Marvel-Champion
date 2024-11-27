use crate::features::cards::Card;

mod get_ready;
mod hawkeye;
mod inspired;
mod lead_from_the_front;
mod make_the_call;
mod maria_hill;
mod the_power_of_leadership;
mod the_triskelion;
mod vision;

pub fn get_all() -> Vec<Card> {
    vec![
        hawkeye::get_hawkeye(),
        maria_hill::get_maria_hill(),
        vision::get_vision(),
        get_ready::get_get_ready(),
        lead_from_the_front::get_lead_from_the_front(),
        make_the_call::get_make_the_call(),
        the_power_of_leadership::get_the_power_of_leadership(),
        the_triskelion::get_the_triskelion(),
        inspired::get_inspired(),
    ]
}
