mod exhaustion;
mod masterplan;
mod under_fire;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

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
