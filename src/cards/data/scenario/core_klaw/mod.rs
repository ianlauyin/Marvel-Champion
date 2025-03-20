mod armored_guard;
mod defence_network;
mod illegal_arms_factory;
mod klaw_i;
mod klaw_ii;
mod klaw_iii;
mod klaws_vengenace;
mod secret_rendezvous_2a;
mod secret_rendezvous_2b;
mod solid_sound_body;
mod sonic_boom;
mod sonic_converter;
mod sound_manipulation;
mod the_immortal_klaw;
mod underground_distribution_1a;
mod underground_distribution_1b;
mod weapon_runner;

use crate::component::card::CardBasic;
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_infos() -> Vec<CardBasic<'static>> {
    vec![
        armored_guard::get_info(),
        defence_network::get_info(),
        illegal_arms_factory::get_info(),
        klaw_i::get_info(),
        klaw_ii::get_info(),
        klaw_iii::get_info(),
        klaws_vengenace::get_info(),
        secret_rendezvous_2a::get_info(),
        secret_rendezvous_2b::get_info(),
        solid_sound_body::get_info(),
        sonic_boom::get_info(),
        sonic_converter::get_info(),
        sound_manipulation::get_info(),
        the_immortal_klaw::get_info(),
        underground_distribution_1a::get_info(),
        underground_distribution_1b::get_info(),
        weapon_runner::get_info(),
    ]
}

pub fn get_cards() -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
    vec![
        armored_guard::get_card(),
        defence_network::get_card(),
        illegal_arms_factory::get_card(),
        klaw_i::get_card(),
        klaw_ii::get_card(),
        klaw_iii::get_card(),
        klaws_vengenace::get_card(),
        secret_rendezvous_2a::get_card(),
        secret_rendezvous_2b::get_card(),
        solid_sound_body::get_card(),
        sonic_boom::get_card(),
        sonic_converter::get_card(),
        sound_manipulation::get_card(),
        the_immortal_klaw::get_card(),
        underground_distribution_1a::get_card(),
        underground_distribution_1b::get_card(),
        weapon_runner::get_card(),
    ]
}
