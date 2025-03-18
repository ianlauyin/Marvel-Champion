use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct WhenDefeatedAbilities(Vec<fn(&mut World)>);
