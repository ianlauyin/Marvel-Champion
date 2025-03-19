use bevy::prelude::{Component, World};

use super::ability::Ability;

#[derive(Component)]
pub struct InterruptAbilities(Vec<Ability>);

impl InterruptAbilities {
    pub fn single(ability: Ability) -> Self {
        Self(vec![ability])
    }
}
