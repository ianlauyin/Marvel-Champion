use crate::features::cards::Card;

mod heart_shaped_herb;
mod killmonger;
mod ritual_combat;
mod usurp_the_throne;

pub fn get_nemesis_set() -> Vec<Card> {
    vec![
        usurp_the_throne::get_usurp_the_throne(),
        killmonger::get_killmonger(),
        heart_shaped_herb::get_heart_shaped_herb(),
        ritual_combat::get_ritual_combat(),
        ritual_combat::get_ritual_combat(),
    ]
}
