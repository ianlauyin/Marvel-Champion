mod alter_ego;
mod aunt_may;
mod backflip;
mod black_cat;
mod enhanced_spider_sense;
mod hero;
mod nemesis_set;
mod obligation;
mod spider_tracer;
mod swinging_web_kick;
mod web_shooter;
mod webbed_up;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![]
}
