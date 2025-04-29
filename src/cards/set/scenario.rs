use bevy::color::Color;

use crate::cards::data::scenario;
use crate::component::Card;

use super::set_trait::SetTrait;

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
}

impl SetTrait for Scenario {
    fn get_boxed_all() -> Vec<Box<dyn SetTrait>> {
        Self::get_all()
            .into_iter()
            .map(|set| Box::new(set) as Box<dyn SetTrait>)
            .collect()
    }

    fn to_str(&self) -> &str {
        match *self {
            Self::CoreRhino => "Core - Rhino",
            Self::CoreKlaw => "Core - Klaw",
            Self::CoreUltron => "Core - Ultron",
        }
    }

    fn get_key(&self) -> &str {
        match *self {
            Self::CoreRhino => "core_rhino",
            Self::CoreKlaw => "core_klaw",
            Self::CoreUltron => "core_ultron",
        }
    }

    fn get_cards<'a>(&self) -> Vec<Card<'a>> {
        match *self {
            Self::CoreRhino => scenario::core_rhino::get_cards(),
            Self::CoreKlaw => scenario::core_klaw::get_cards(),
            Self::CoreUltron => scenario::core_ultron::get_cards(),
        }
    }

    fn get_thumbnail_key(&self) -> Option<String> {
        Some(format!("scenario/{}", self.get_key()))
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
