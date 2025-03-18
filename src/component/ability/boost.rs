use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct BoostAbilities(Vec<fn(&mut World)>);
