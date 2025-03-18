use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct InstantAbilities(pub Vec<fn(&mut World)>);

impl InstantAbilities {
    pub fn single(func: fn(&mut World)) -> Self {
        Self(vec![func])
    }
}
