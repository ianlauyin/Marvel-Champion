use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct BoostAbility(fn(&mut World));
