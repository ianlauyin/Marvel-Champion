use bevy::prelude::{Component, World};

use super::ability::Ability;

#[derive(Component)]
pub struct WhenRevealedAbilities(Vec<Ability>);

impl WhenRevealedAbilities {
    pub fn single(ability: Ability) -> Self {
        Self(vec![ability])
    }
}
