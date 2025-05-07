use bevy::color::Color;

use crate::cards::data::modular_set;
use crate::component::Card;

use super::set_trait::SetTrait;

#[derive(Clone, PartialEq, Eq)]
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
}

impl SetTrait for ModularSet {
    fn get_boxed_all() -> Vec<Box<dyn SetTrait>> {
        Self::get_all()
            .into_iter()
            .map(|set| Box::new(set) as Box<dyn SetTrait>)
            .collect()
    }

    fn to_str(&self) -> &str {
        match *self {
            Self::BombScare => "Bomb Scare",
            Self::MastersOfEvil => "Masters of Evil",
            Self::UnderAttack => "Under Attack",
            Self::LegionsOfHydra => "Legions Of Hydra",
            Self::TheDoomsdayChair => "The Doomsday Chair",
        }
    }

    fn get_key(&self) -> &str {
        match *self {
            Self::BombScare => "bomb_scare",
            Self::MastersOfEvil => "masters_of_evil",
            Self::UnderAttack => "under_attack",
            Self::LegionsOfHydra => "legions_of_hydra",
            Self::TheDoomsdayChair => "the_doomsday_chair",
        }
    }

    fn get_cards<'a>(&self) -> Vec<Card<'a>> {
        match *self {
            Self::BombScare => modular_set::bomb_scare::get_cards(),
            Self::MastersOfEvil => modular_set::masters_of_evil::get_cards(),
            Self::UnderAttack => modular_set::under_attack::get_cards(),
            Self::LegionsOfHydra => modular_set::legions_of_hydra::get_cards(),
            Self::TheDoomsdayChair => modular_set::the_doomsday_chair::get_cards(),
        }
    }

    fn get_thumbnail_key(&self) -> Option<String> {
        Some(format!("modular_set/{}", self.get_key()))
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
