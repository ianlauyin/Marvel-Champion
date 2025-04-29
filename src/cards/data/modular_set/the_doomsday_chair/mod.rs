mod biomechanical_upgrades;
mod modok;
mod the_doomsday_chair;

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        biomechanical_upgrades::get_card(),
        modok::get_card(),
        the_doomsday_chair::get_card(),
    ]
}
