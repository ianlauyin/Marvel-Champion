use bevy::prelude::Component;

use super::ability::Ability;

#[derive(Component)]
pub struct SetupAbilities(Vec<Ability>);

impl SetupAbilities {
    pub fn single(ability: Ability) -> Self {
        Self(vec![ability])
    }
}
