use bevy::prelude::{Component, World};

use super::ability::Ability;

#[derive(Component)]
pub struct ResponseAbilities(Vec<Ability>);

impl ResponseAbilities {
    pub fn single(ability: Ability) -> Self {
        Self(vec![ability])
    }
}
