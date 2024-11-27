mod chase_them_down;
mod combat_training;
mod hulk;
mod relentless_assault;
mod tac_team;
mod the_power_of_aggression;
mod tigra;
mod uppercut;
use crate::features::cards::Card;

pub fn get_all() -> Vec<Card> {
    vec![
        hulk::get_hulk(),
        tigra::get_tigra(),
        chase_them_down::get_chase_them_down(),
        relentless_assault::get_relentless_assault(),
        uppercut::get_uppercut(),
        the_power_of_aggression::get_the_power_of_aggression(),
        tac_team::get_tac_team(),
        combat_training::get_combat_training(),
    ]
}
