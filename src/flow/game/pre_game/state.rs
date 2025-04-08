use bevy::prelude::*;

use crate::flow::game::state::GameState;

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(GameState = GameState::PreGame)]
pub enum PreGameState {
    #[default]
    HeroMenu,
    EnemyMenu,
}
