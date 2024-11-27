use crate::features::cards::Card;

mod daredevil;
mod for_justice;
mod great_responsibility;
mod heroic_intuition;
mod interrogation_room;
mod jessica_jones;
mod surveillance_team;
mod the_power_of_justice;

pub fn get_all() -> Vec<Card> {
    vec![
        daredevil::get_daredevil(),
        jessica_jones::get_jessica_jones(),
        for_justice::get_for_justice(),
        great_responsibility::get_great_responsiblity(),
        the_power_of_justice::get_the_power_of_justice(),
        interrogation_room::get_interrogation_room(),
        surveillance_team::get_surveillance_team(),
        heroic_intuition::get_heroic_intuition(),
    ]
}
