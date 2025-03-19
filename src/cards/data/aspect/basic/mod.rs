mod avengers_mansion;
mod emergency;
mod energy;
mod first_aid;
mod genius;
mod haymaker;
mod helicarrier;
mod mockingbird;
mod nick_fury;
mod strength;
mod tenacity;

use crate::component::card::CardBasic;
use bevy::ecs::system::Commands;

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        avengers_mansion::get_info(),
        emergency::get_info(),
        energy::get_info(),
        first_aid::get_info(),
        genius::get_info(),
        haymaker::get_info(),
        helicarrier::get_info(),
        mockingbird::get_info(),
        nick_fury::get_info(),
        strength::get_info(),
        tenacity::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands))> {
    vec![
        avengers_mansion::get_card(),
        emergency::get_card(),
        energy::get_card(),
        first_aid::get_card(),
        genius::get_card(),
        haymaker::get_card(),
        helicarrier::get_card(),
        mockingbird::get_card(),
        nick_fury::get_card(),
        strength::get_card(),
        tenacity::get_card(),
    ]
}
