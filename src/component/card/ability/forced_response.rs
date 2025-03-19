use bevy::prelude::Component;

use super::ability::Ability;

#[derive(Component)]
pub struct ForcedResponseAbilities(Vec<Ability>);

impl ForcedResponseAbilities {
    pub fn single(ability: Ability) -> Self {
        Self(vec![ability])
    }
}
