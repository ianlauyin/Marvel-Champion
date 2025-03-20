mod alpha_flight_station;
mod alter_ego;
mod captain_marvels_helmet;
mod cosmic_flight;
mod crisis_interdiction;
mod energy_absorption;
mod energy_channel;
mod hero;
mod nemesis_set;
mod obligation;
mod photonic_blast;
mod spider_woman;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![]
}
