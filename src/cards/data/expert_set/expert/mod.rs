mod exhaustion;
mod masterplan;
mod under_fire;

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        exhaustion::get_card(),
        masterplan::get_card(),
        under_fire::get_card(),
    ]
}
