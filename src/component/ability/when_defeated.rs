use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct WhenDefeatedAbility(fn(&mut World));
