use crate::features::cards::Card;

mod hydra_soldier;
mod legions_of_hydra;
mod madame_hydra;

pub fn get_all() -> Vec<Card> {
    vec![
        legions_of_hydra::get_legions_of_hydra(),
        legions_of_hydra::get_legions_of_hydra(),
        madame_hydra::get_madame_hydra(),
        hydra_soldier::get_hydra_soldier(),
        hydra_soldier::get_hydra_soldier(),
        hydra_soldier::get_hydra_soldier(),
    ]
}
