use bevy::prelude::Component;

use super::{
    data::{core_klaw, core_rhino, core_ultron},
    Card,
};

#[derive(Component, Clone, Eq, PartialEq)]
pub enum Villain {
    CoreRhino,
    CoreKlaw,
    CoreUltron,
}

impl Villain {
    pub fn get_all() -> Vec<Self> {
        vec![Villain::CoreRhino, Villain::CoreKlaw, Villain::CoreUltron]
    }

    pub fn get_all_cards() -> Vec<Card> {
        let mut cards = Vec::new();
        for identity in Villain::get_all() {
            cards.push(identity.get_cards());
        }
        cards.concat()
    }

    pub fn to_string(&self) -> String {
        let str = match *self {
            Villain::CoreRhino => "Core - Rhino",
            Villain::CoreKlaw => "Core - Klaw",
            Villain::CoreUltron => "Core - Ultron",
        };
        str.to_string()
    }
    pub fn get_title_image_path(&self) -> String {
        let prefix = "embedded://villain/";
        let postfix = ".png";
        let name = match *self {
            Villain::CoreRhino => "core_rhino",
            Villain::CoreKlaw => "core_klaw",
            Villain::CoreUltron => "core_ultron",
        };
        format!("{prefix}{name}{postfix}")
    }

    pub fn get_cards(&self) -> Vec<Card> {
        match *self {
            Villain::CoreRhino => core_rhino::get_all(),
            Villain::CoreKlaw => core_klaw::get_all(),
            Villain::CoreUltron => core_ultron::get_all(),
        }
    }

    pub fn get_encounter_set_numbers(&self) -> usize {
        match *self {
            _ => 1,
        }
    }
}
