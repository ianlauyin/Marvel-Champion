use crate::features::cards::Card;

mod nemesis;
mod nemesis_side_scheme;
mod sweeping_swoop;
mod vultures_plan;

pub fn get_nemesis_set() -> Vec<Card> {
    vec![
        nemesis_side_scheme::get_nemesis_side_scheme(),
        nemesis::get_nemesis(),
        sweeping_swoop::get_sweeping_swoop(),
        vultures_plan::get_vultures_plans(),
    ]
}
