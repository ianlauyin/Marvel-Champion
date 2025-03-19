use bevy::prelude::Component;

use super::ability::Ability;

#[derive(Component)]
pub struct ForcedInterruptAbilities(Vec<Ability>);
