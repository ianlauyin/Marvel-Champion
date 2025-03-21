use bevy::prelude::Component;

use crate::flow::collection::state::CollectionState;

#[derive(Component)]
pub enum CollectionMenuButton {
    Aspect,
    IdentitySet,
    ModularSet,
    Scenario,
    StandardSet,
    ExpertSet,
}

impl CollectionMenuButton {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::Aspect,
            Self::IdentitySet,
            Self::ModularSet,
            Self::Scenario,
            Self::StandardSet,
            Self::ExpertSet,
        ]
    }

    pub fn get_text(&self) -> &str {
        match self {
            Self::Aspect => "Aspect",
            Self::IdentitySet => "Identity Set",
            Self::ModularSet => "Modular Set",
            Self::Scenario => "Scenario",
            Self::StandardSet => "Standard Set",
            Self::ExpertSet => "Expert Set",
        }
    }

    pub fn get_state(&self) -> CollectionState {
        match self {
            Self::Aspect => CollectionState::Aspect,
            Self::IdentitySet => CollectionState::IdentitySet,
            Self::ModularSet => CollectionState::ModularSet,
            Self::Scenario => CollectionState::Scenario,
            Self::StandardSet => CollectionState::StandardSet,
            Self::ExpertSet => CollectionState::ExpertSet,
        }
    }
}
