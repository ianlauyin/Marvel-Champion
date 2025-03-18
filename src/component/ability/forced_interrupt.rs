use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct ForcedInterruptAbilities(Vec<fn(&mut World)>);
