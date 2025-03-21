use bevy::ecs::{entity::Entity, system::Commands};

use crate::cards::data::modular_set;
use crate::component::card::CardBasic;

#[derive(Clone)]
pub enum ModularSet {
    BombScare,
    MastersOfEvil,
    UnderAttack,
    LegionsOfHydra,
    TheDoomsdayChair,
}

impl ModularSet {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::BombScare,
            Self::MastersOfEvil,
            Self::UnderAttack,
            Self::LegionsOfHydra,
            Self::TheDoomsdayChair,
        ]
    }

    pub fn to_str(&self) -> &str {
        match *self {
            Self::BombScare => "Bomb Scare",
            Self::MastersOfEvil => "Masters of Evil",
            Self::UnderAttack => "Under Attack",
            Self::LegionsOfHydra => "Legions Of Hydra",
            Self::TheDoomsdayChair => "The Doomsday Chair",
        }
    }

    pub fn get_key(&self) -> &str {
        match *self {
            Self::BombScare => "bomb_scare",
            Self::MastersOfEvil => "masters_of_evil",
            Self::UnderAttack => "under_attack",
            Self::LegionsOfHydra => "legions_of_hydra",
            Self::TheDoomsdayChair => "the_doomsday_chair",
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic> {
        match *self {
            Self::BombScare => modular_set::bomb_scare::get_infos(),
            Self::MastersOfEvil => modular_set::masters_of_evil::get_infos(),
            Self::UnderAttack => modular_set::under_attack::get_infos(),
            Self::LegionsOfHydra => modular_set::legions_of_hydra::get_infos(),
            Self::TheDoomsdayChair => modular_set::the_doomsday_chair::get_infos(),
        }
    }

    pub fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
        match *self {
            Self::BombScare => modular_set::bomb_scare::get_cards(),
            Self::MastersOfEvil => modular_set::masters_of_evil::get_cards(),
            Self::UnderAttack => modular_set::under_attack::get_cards(),
            Self::LegionsOfHydra => modular_set::legions_of_hydra::get_cards(),
            Self::TheDoomsdayChair => modular_set::the_doomsday_chair::get_cards(),
        }
    }
}
