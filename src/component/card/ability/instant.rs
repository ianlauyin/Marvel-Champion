use bevy::prelude::Component;

use super::ability::Ability;

#[derive(Component)]
pub struct InstantAbilities(pub Vec<Ability>);

impl InstantAbilities {
    pub fn single(ability: Ability) -> Self {
        Self(vec![ability])
    }
}
