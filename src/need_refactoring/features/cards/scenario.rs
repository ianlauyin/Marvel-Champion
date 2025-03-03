use bevy::prelude::Component;

use super::{
    data::{core_klaw, core_rhino, core_ultron},
    Card,
};

#[derive(Component, Clone, Eq, PartialEq)]
pub enum Scenario {
    CoreRhino,
    CoreKlaw,
    CoreUltron,
}

impl Scenario {
    pub fn get_all() -> Vec<Self> {
        vec![
            Scenario::CoreRhino,
            Scenario::CoreKlaw,
            Scenario::CoreUltron,
        ]
    }

    pub fn get_all_cards() -> Vec<Card> {
        let mut cards = Vec::new();
        for identity in Scenario::get_all() {
            cards.push(identity.get_cards());
        }
        cards.concat()
    }

    pub fn to_string(&self) -> String {
        let str = match *self {
            Scenario::CoreRhino => "Core - Rhino",
            Scenario::CoreKlaw => "Core - Klaw",
            Scenario::CoreUltron => "Core - Ultron",
        };
        str.to_string()
    }
    pub fn get_title_image_path(&self) -> String {
        let prefix = "embedded://scenario/";
        let postfix = ".png";
        let name = match *self {
            Scenario::CoreRhino => "core_rhino",
            Scenario::CoreKlaw => "core_klaw",
            Scenario::CoreUltron => "core_ultron",
        };
        format!("{prefix}{name}{postfix}")
    }

    pub fn get_standard_villain_cards(&self) -> Vec<Card> {
        match *self {
            Scenario::CoreRhino => core_rhino::get_standard_villain_cards(),
            Scenario::CoreKlaw => core_klaw::get_standard_villain_cards(),
            Scenario::CoreUltron => core_ultron::get_standard_villain_cards(),
        }
    }

    pub fn get_expert_villain_cards(&self) -> Vec<Card> {
        match *self {
            Scenario::CoreRhino => core_rhino::get_expert_villain_cards(),
            Scenario::CoreKlaw => core_klaw::get_expert_villain_cards(),
            Scenario::CoreUltron => core_ultron::get_expert_villain_cards(),
        }
    }

    pub fn get_main_scheme_cards(&self) -> Vec<Card> {
        match *self {
            Scenario::CoreRhino => core_rhino::get_main_scheme_cards(),
            Scenario::CoreKlaw => core_klaw::get_main_scheme_cards(),
            Scenario::CoreUltron => core_ultron::get_main_scheme_cards(),
        }
    }

    pub fn get_encounter_cards(&self) -> Vec<Card> {
        match *self {
            Scenario::CoreRhino => core_rhino::get_encounter_cards(),
            Scenario::CoreKlaw => core_klaw::get_encounter_cards(),
            Scenario::CoreUltron => core_ultron::get_encounter_cards(),
        }
    }

    pub fn get_cards(&self) -> Vec<Card> {
        match *self {
            Scenario::CoreRhino => core_rhino::get_all(),
            Scenario::CoreKlaw => core_klaw::get_all(),
            Scenario::CoreUltron => core_ultron::get_all(),
        }
    }

    pub fn get_encounter_set_numbers(&self) -> usize {
        match *self {
            _ => 1,
        }
    }
}
