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
    vec![
        alter_ego::get_info(),
        aunt_may::get_info(),
        backflip::get_info(),
        black_cat::get_info(),
        enhanced_spider_sense::get_info(),
        hero::get_info(),
        obligation::get_info(),
        spider_tracer::get_info(),
        swinging_web_kick::get_info(),
        web_shooter::get_info(),
        webbed_up::get_info(),
        nemesis_set::highway_robbery::get_info(),
        nemesis_set::sweeping_swoop::get_info(),
        nemesis_set::vulture::get_info(),
        nemesis_set::vultures_plan::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        alter_ego::get_card(),
        aunt_may::get_card(),
        backflip::get_card(),
        black_cat::get_card(),
        enhanced_spider_sense::get_card(),
        hero::get_card(),
        obligation::get_card(),
        spider_tracer::get_card(),
        swinging_web_kick::get_card(),
        web_shooter::get_card(),
        webbed_up::get_card(),
        nemesis_set::highway_robbery::get_card(),
        nemesis_set::sweeping_swoop::get_card(),
        nemesis_set::vulture::get_card(),
        nemesis_set::vultures_plan::get_card(),
    ]
}
