use bevy::ecs::{entity::Entity, system::Commands};

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

    pub fn to_string(&self) -> String {
        let str = match *self {
            Self::CoreRhino => "Core - Rhino",
            Self::CoreKlaw => "Core - Klaw",
            Self::CoreUltron => "Core - Ultron",
        };
        str.to_string()
    }

    pub fn get_key(&self) -> String {
        match *self {
            Self::CoreRhino => "core_rhino".to_string(),
            Self::CoreKlaw => "core_klaw".to_string(),
            Self::CoreUltron => "core_ultron".to_string(),
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic> {
        todo!()
    }

    pub fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)> {
        todo!()
    }
}
