use bevy::ecs::{entity::Entity, system::Commands};

use crate::component::card::CardBasic;

mod exhaustion;
mod masterplan;
mod under_fire;

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        exhaustion::get_info(),
        masterplan::get_info(),
        under_fire::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        exhaustion::get_card(),
        masterplan::get_card(),
        under_fire::get_card(),
    ]
}
