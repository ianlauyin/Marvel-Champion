use bevy::prelude::Component;

use super::ability::Ability;

#[derive(Component)]
pub struct ForcedInterruptAbilities(Vec<Ability>);

impl ForcedInterruptAbilities {
    pub fn single(ability: Ability) -> Self {
        Self(vec![ability])
    }
}

