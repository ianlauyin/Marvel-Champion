use bevy::prelude::Component;

use super::ability::Ability;

#[derive(Component)]
pub struct BoostAbilities(Vec<Ability>);

impl BoostAbilities {
    pub fn single(ability: Ability) -> Self {
        Self(vec![ability])
    }
}
