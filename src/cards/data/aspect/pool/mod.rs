use crate::component::card::CardBasic;
use bevy::ecs::system::Commands;

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands))> {
    vec![]
}
