use bevy::prelude::Component;

use super::ability::Ability;

#[derive(Component)]
pub struct WhenDefeatedAbilities(Vec<Ability>);

impl WhenDefeatedAbilities {
    pub fn single(ability: Ability) -> Self {
        Self(vec![ability])
    }
}
