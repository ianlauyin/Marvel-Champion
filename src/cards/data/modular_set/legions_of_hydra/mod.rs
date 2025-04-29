mod hydra_soldier;
mod legions_of_hydra;
mod madame_hydra;

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        hydra_soldier::get_card(),
        legions_of_hydra::get_card(),
        madame_hydra::get_card(),
    ]
}
