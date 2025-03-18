use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct InstantAbility(fn(&mut World));
