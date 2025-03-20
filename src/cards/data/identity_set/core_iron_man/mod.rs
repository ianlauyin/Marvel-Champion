mod alter_ego;
mod arc_reactor;
mod hero;
mod mark_v_armor;
mod mark_v_helmet;
mod nemesis_set;
mod obligation;
mod pepper_potts;
mod powered_gauntlets;
mod repulsor_blast;
mod rocket_boots;
mod stark_tower;
mod supersonic_punch;
mod war_machine;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![]
}
