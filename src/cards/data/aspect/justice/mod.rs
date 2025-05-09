mod daredevil;
mod for_justice;
mod great_responsibility;
mod heroic_intuition;
mod interrogation_room;
mod jessica_jones;
mod surveillance_team;
mod the_power_of_justice;

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        daredevil::get_card(),
        for_justice::get_card(),
        great_responsibility::get_card(),
        heroic_intuition::get_card(),
        interrogation_room::get_card(),
        jessica_jones::get_card(),
        surveillance_team::get_card(),
        the_power_of_justice::get_card(),
    ]
}
