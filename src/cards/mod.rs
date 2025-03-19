mod belongs;
mod data;
mod set;

pub use belongs::{Belong, Belongs};
use bevy::ecs::system::Commands;
pub use set::{Aspect, BasicSet, IdentitySet, ModularSet, Scenario};

use crate::component::card::CardBasic;
