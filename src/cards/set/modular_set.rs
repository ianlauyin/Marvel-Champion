use bevy::ecs::world::World;

use crate::component::card::CardBasic;

pub enum ModularSet {
    BombScare,
    MastersOfEvil,
    UnderAttack,
    LegionsOfHydra,
    TheDoomsdayChair,
}

impl ModularSet {
    pub fn to_string(&self) -> String {
        let str = match *self {
            Self::BombScare => "Bomb Scare",
            Self::MastersOfEvil => "Masters of Evil",
            Self::UnderAttack => "Under Attack",
            Self::LegionsOfHydra => "Legions Of Hydra",
            Self::TheDoomsdayChair => "The Doomsday Chair",
        };
        str.to_string()
    }

    pub fn get_key(&self) -> String {
        match *self {
            Self::BombScare => "bomb_scare".to_string(),
            Self::MastersOfEvil => "master_of_evil".to_string(),
            Self::UnderAttack => "under_attack".to_string(),
            Self::LegionsOfHydra => "legions_of_hydra".to_string(),
            Self::TheDoomsdayChair => "the_doomsday_chair".to_string(),
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic> {
        todo!()
    }

    pub fn get_cards(&self) -> Vec<(CardBasic, fn(&mut World))> {
        todo!()
    }
}
