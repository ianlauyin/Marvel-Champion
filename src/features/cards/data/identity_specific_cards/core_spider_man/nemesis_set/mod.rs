use crate::features::cards::Card;

mod nemesis;
mod nemesis_side_scheme;
mod sweeping_swoop;
mod vultures_plan;

pub fn get_nemesis_set(player_number: u8) -> Vec<Card> {
    vec![
        nemesis::get_nemesis(),
        nemesis_side_scheme::get_nemesis_side_scheme(player_number),
        sweeping_swoop::get_sweeping_swoop(),
        vultures_plan::get_vultures_plans(),
    ]
}
