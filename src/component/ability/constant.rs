use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct ConstantAbilities(pub Vec<fn(&mut World)>);

impl ConstantAbilities {
    pub fn single(func: fn(&mut World)) -> Self {
        Self(vec![func])
    }
}
