use bevy::prelude::Component;

use crate::cards::*;

#[derive(Component)]
pub enum SubMenu {
    Aspect,
    IdentitySet,
    ModularSet,
    Scenario,
    StandardSet,
    ExpertSet,
}

impl SubMenu {
    pub fn get_sets(&self) -> Vec<Box<dyn SetTrait>> {
        match self {
            Self::Aspect => Aspect::get_boxed_all(),
            Self::IdentitySet => IdentitySet::get_boxed_all(),
            Self::ModularSet => ModularSet::get_boxed_all(),
            Self::Scenario => Scenario::get_boxed_all(),
            Self::StandardSet => StandardSet::get_boxed_all(),
            Self::ExpertSet => ExpertSet::get_boxed_all(),
        }
    }
}
