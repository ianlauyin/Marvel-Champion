mod alter_ego;
mod ancestral_knowledge;
mod energy_daggers;
mod hero;
mod nemesis_set;
mod obligation;
mod panther_claws;
mod shuri;
mod tactical_genius;
mod the_golden_city;
mod vibranium;
mod vibranium_suit;
mod wakanda_forever_a;
mod wakanda_forever_b;
mod wakanda_forever_c;
mod wakanda_forever_d;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        alter_ego::get_info(),
        ancestral_knowledge::get_info(),
        energy_daggers::get_info(),
        hero::get_info(),
        obligation::get_info(),
        panther_claws::get_info(),
        shuri::get_info(),
        tactical_genius::get_info(),
        the_golden_city::get_info(),
        vibranium::get_info(),
        vibranium_suit::get_info(),
        wakanda_forever_a::get_info(),
        wakanda_forever_b::get_info(),
        wakanda_forever_c::get_info(),
        wakanda_forever_d::get_info(),
        nemesis_set::heart_shaped_herb::get_info(),
        nemesis_set::killmonger::get_info(),
        nemesis_set::ritual_combat::get_info(),
        nemesis_set::usurp_the_throne::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        alter_ego::get_card(),
        ancestral_knowledge::get_card(),
        energy_daggers::get_card(),
        hero::get_card(),
        obligation::get_card(),
        panther_claws::get_card(),
        shuri::get_card(),
        tactical_genius::get_card(),
        the_golden_city::get_card(),
        vibranium::get_card(),
        vibranium_suit::get_card(),
        wakanda_forever_a::get_card(),
        wakanda_forever_b::get_card(),
        wakanda_forever_c::get_card(),
        wakanda_forever_d::get_card(),
        nemesis_set::heart_shaped_herb::get_card(),
        nemesis_set::killmonger::get_card(),
        nemesis_set::ritual_combat::get_card(),
        nemesis_set::usurp_the_throne::get_card(),
    ]
}
