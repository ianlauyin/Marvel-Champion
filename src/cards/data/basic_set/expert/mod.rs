use crate::features::cards::Card;

mod exhaustion;
mod masterplan;
mod under_fire;

pub fn get_all() -> Vec<Card> {
    vec![
        exhaustion::get_exhaustion(),
        masterplan::get_masterplan(),
        under_fire::get_under_fire(),
    ]
}
