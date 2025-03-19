use bevy::prelude::Component;

use super::ability::Ability;

#[derive(Component)]
pub struct ConstantAbilities(pub Vec<Ability>);

impl ConstantAbilities {
    pub fn single(ability: Ability) -> Self {
        Self(vec![ability])
    }
}
