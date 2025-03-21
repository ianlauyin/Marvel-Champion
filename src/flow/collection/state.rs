use bevy::prelude::*;

use crate::{cards::Aspect, flow::state::AppState};

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(AppState = AppState::Collection)]
pub enum CollectionState {
    #[default]
    MainMenu,
    Aspect,
    IdentitySet,
    ModularSet,
    Scenario,
    StandardSet,
    ExpertSet,
}
