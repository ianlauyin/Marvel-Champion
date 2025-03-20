mod alter_ego;
mod focused_rage;
mod gamma_slam;
mod ground_stomp;
mod hellcat;
mod hero;
mod legal_practice;
mod nemesis_set;
mod obligation;
mod one_two_punch;
mod split_personality;
mod superhuman_law_division;
mod superhuman_strength;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![]
}
