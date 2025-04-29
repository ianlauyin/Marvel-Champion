mod chase_them_down;
mod combat_training;
mod hulk;
mod relentless_assault;
mod tac_team;
mod the_power_of_aggression;
mod tigra;
mod uppercut;

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        chase_them_down::get_card(),
        combat_training::get_card(),
        hulk::get_card(),
        relentless_assault::get_card(),
        tac_team::get_card(),
        the_power_of_aggression::get_card(),
        tigra::get_card(),
        uppercut::get_card(),
    ]
}
