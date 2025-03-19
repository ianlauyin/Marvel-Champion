mod daredevil;
mod for_justice;
mod great_responsibility;
mod heroic_intuition;
mod interrogation_room;
mod jessica_jones;
mod surveillance_team;
mod the_power_of_justice;

use crate::component::card::CardBasic;
use bevy::ecs::system::Commands;

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        daredevil::get_info(),
        for_justice::get_info(),
        great_responsibility::get_info(),
        heroic_intuition::get_info(),
        interrogation_room::get_info(),
        jessica_jones::get_info(),
        surveillance_team::get_info(),
        the_power_of_justice::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands))> {
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
