use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct ResponseAbilities(Vec<fn(&mut World)>);
