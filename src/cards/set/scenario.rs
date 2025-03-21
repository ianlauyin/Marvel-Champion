use bevy::ecs::{entity::Entity, system::Commands};

use crate::cards::data::scenario;
use crate::component::card::CardBasic;

#[derive(Clone)]
pub enum Scenario {
    CoreRhino,
    CoreKlaw,
    CoreUltron,
}

impl Scenario {
    pub fn get_all() -> Vec<Self> {
        vec![Self::CoreRhino, Self::CoreKlaw, Self::CoreUltron]
    }

    pub fn to_str(&self) -> &str {
        match *self {
            Self::CoreRhino => "Core - Rhino",
            Self::CoreKlaw => "Core - Klaw",
            Self::CoreUltron => "Core - Ultron",
        }
    }

    pub fn get_key(&self) -> &str {
        match *self {
            Self::CoreRhino => "core_rhino",
            Self::CoreKlaw => "core_klaw",
            Self::CoreUltron => "core_ultron",
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic> {
        match *self {
            Self::CoreRhino => scenario::core_rhino::get_infos(),
            Self::CoreKlaw => scenario::core_klaw::get_infos(),
            Self::CoreUltron => scenario::core_ultron::get_infos(),
        }
    }

    pub fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
        match *self {
            Self::CoreRhino => scenario::core_rhino::get_cards(),
            Self::CoreKlaw => scenario::core_klaw::get_cards(),
            Self::CoreUltron => scenario::core_ultron::get_cards(),
        }
    }
}
