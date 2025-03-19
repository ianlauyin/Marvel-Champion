use bevy::prelude::Component;

use super::ability::Ability;

#[derive(Component)]
pub struct ResourceModifier(Vec<Ability>);

impl ResourceModifier {
    pub fn single(ability: Ability) -> Self {
        Self(vec![ability])
    }
}
