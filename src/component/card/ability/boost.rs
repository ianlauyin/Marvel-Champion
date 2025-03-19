use bevy::prelude::Component;

use super::ability::Ability;

#[derive(Component)]
pub struct BoostAbilities(Vec<Ability>);
