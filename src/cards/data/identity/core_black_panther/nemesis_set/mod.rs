use bevy::ecs::system::Commands;

use crate::component::card::CardBasic;

mod heart_shaped_herb;
mod killmonger;
mod ritual_combat;
mod usurp_the_throne;

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        heart_shaped_herb::get_info(),
        killmonger::get_info(),
        ritual_combat::get_info(),
        usurp_the_throne::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands))> {
    vec![
        heart_shaped_herb::get_card(),
        killmonger::get_card(),
        ritual_combat::get_card(),
        usurp_the_throne::get_card(),
    ]
}
